#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate anyhow;

use args::Command;
use chrono::Local;
use clap::Parser;
use ipmi::{Cmd, Ipmi, IpmiTool};
use log::{error, info};
use std::{io::Write, ops::RangeInclusive};
use tokio::time::{self, Duration};

mod args;
mod ipmi;

fn calc_speed(temperature: u16, target_temperature: u16, threshold: u16, max_fan_speed: u16) -> u16 {
    if temperature <= target_temperature {
        0
    } else if temperature >= threshold {
        max_fan_speed
    } else {
        // Easing-in: fan speed increases slowly at first, then accelerates as temperature approaches threshold
        let temp_range = (threshold - target_temperature) as f32;
        let temp_pos = (temperature - target_temperature) as f32;
        let ratio = temp_pos / temp_range;
        // Ease-in cubic: y = x^3
        let eased = ratio.powi(3);
        (eased * max_fan_speed as f32).round() as u16
    }
}

fn show_all_speeds(
    target_temperature: u16,
    threshold: u16,
    max_fan_speed: u16,
) {
    println!("Temperature\tFan Speed");
    for temp in 20..=100 {
        let speed = calc_speed(temp, target_temperature, threshold, max_fan_speed);
        println!("{:>3}°C\t\t{:>3}%", temp, speed);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = args::Args::parse();

    let mut level = log::LevelFilter::Debug;

    if args.verbose {
        level = log::LevelFilter::Trace;
    }

    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter_level(level)
        .init();

    let tool = IpmiTool::new(Box::new(Cmd::new()));

    match args.command {
        Command::PrintAllSpeeds(a) => {
            let target_temperature = a.target_temperature;
            let threshold = a.threshold;
            let max_fan_speed = a.max_fan_speed;

            show_all_speeds(target_temperature, threshold, max_fan_speed);
        }
        Command::Auto(a) => {
            let mut interval = a.interval;
            if !RangeInclusive::new(5, 120).contains(&interval) {
                interval = 5;
                info!("invalid interval, interval set to 5");
            }

            let mut threshold = a.threshold;
            if !RangeInclusive::new(40, 100).contains(&threshold) {
                threshold = 70;
                info!("invalid threshold, threshold set to {}", threshold);
            }

            info!(
                "auto mode start, interval: {}, threshold: {}",
                interval, threshold
            );

            let mut interval = time::interval(Duration::from_secs(interval));

            // Every hour, clear the air for 10 seconds
            // This is to prevent dust accumulation and ensure the fan runs at full speed periodically
            // This is useful for servers that run 24/7
            let clear_air_duration = Duration::from_secs(a.clear_air_duration);
            let clean_air_interval = Duration::from_secs(a.clear_air_interval);

            let mut last_speed = 0xff;
            let mut last_air_clean_time = std::time::Instant::now();

            loop {
                interval.tick().await;

                if a.clear_air_interval != 0 && last_air_clean_time.elapsed() >= clean_air_interval {
                    // Reset the last air clean time to now
                    last_air_clean_time = std::time::Instant::now();
                    info!("clearing air for 10 seconds");
                    if let Err(e) = tool.set_fan_speed(a.max_fan_speed) {
                        error!("failed to set fan speed to {}%: {}", a.max_fan_speed, e);
                    }
                    tokio::time::sleep(clear_air_duration).await;
                    if let Err(e) = tool.set_fan_speed(0) {
                        error!("failed to set fan speed to 0%: {}", e);
                    }
                }

                if let Ok(temperature) = tool.get_cpu_temperature() {
                    let speed = calc_speed(
                        temperature,
                        a.target_temperature,
                        threshold,
                        a.max_fan_speed,
                    );

                    if last_speed != speed {
                        match tool.set_fan_speed(speed) {
                            Ok(_) => {
                                last_speed = speed;
                                info!("temperature: {}, set fan speed to {}", temperature, speed);
                            }
                            Err(e) => error!("failed to set fan speed: {}", e),
                        }
                    }
                } else {
                    error!("failed to get cpu temperature");
                }
            }
        }
        Command::Fixed { value } => {
            let mut v = value;
            if v > 100 {
                v = 100;
            }
            info!("fixed mode, set fan speed to {}", v);
            if let Err(e) = tool.set_fan_speed(v) {
                error!("set fan speed, error: {}", e);
            }
        }
        Command::Info => match tool.get_info_fan_temp() {
            Ok(info) => {
                println!("{}", info);
            }
            Err(err) => {
                error!("get info error: {}", err);
            }
        },
    }
}
