use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Subcommands
    #[command(subcommand)]
    pub command: Command,

    /// Verbose output
    #[arg(long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Auto adjust fan speed by interval checking CPU temperature
    Auto(Auto),

    /// Set fixed RPM percentage for fan
    Fixed {
        /// value range 0-100
        #[arg(value_parser)]
        value: u16,
    },

    /// Print CPU temperature and fan RPM
    Info,

    PrintAllSpeeds(Auto),
}

#[derive(clap::Args)]
pub struct Auto {
    /// check CPU temperature every how many seconds
    #[arg(short, long, default_value = "5")]
    pub interval: u64,

    /// threshold CPU temperature for full speed Fan, accepted value range [60-100]
    #[arg(short = 'u', long, default_value = "70")]
    pub threshold: u16,

    /// target temperature to keep CPU below, fans will run quietly below this temperature, accepted value range [20-60]
    #[arg(short = 'l', long, default_value = "35")]
    pub target_temperature: u16,

    /// max fan speed percentage, accepted value range [0-100]
    #[arg(short, long, default_value = "100")]
    pub max_fan_speed: u16,

    /// clear air (run fans at full speed) every how many seconds. 0 disables this feature
    #[arg(short = 'c', long, default_value = "0")]
    pub clear_air_interval: u64,

    /// clear air duration in seconds
    #[arg(short = 'x', long, default_value = "10")]
    pub clear_air_duration: u64,
}
