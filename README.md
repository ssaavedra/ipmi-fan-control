# ipmi-fan-control

[![GitHub license](https://img.shields.io/github/license/yinheli/ipmi-fan-control)](https://github.com/yinheli/ipmi-fan-control/blob/master/LICENSE)

A tool to control the fan speed by monitoring the temperature of CPU via IPMI.

## Why

Dell R730 server's iDRAC may work great for a datacenter, but it's loud for a homelab. This tool uses a ease-in cubic approach to the fan control, to ease in the curve at lowish temps.

## Usage

Download from [release](https://github.com/ssaavedra/ipmi-fan-control/releases) page (prebuilt binary via github actions), or build from source code.

```bash
cargo build --release
```

Install dependency, install (debian/pve):

```bash
apt install ipmitool
```

use `ipmi-fan-control --help` to see the usage.

```bash
ipmi-fan-control --help
```

```
USAGE:
    ipmi-fan-control [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
        --verbose    Verbose output

SUBCOMMANDS:
    auto     Auto adjust fan speed by interval checking CPU temperature
    fixed    Set fixed RPM percentage for fan
    help     Print this message or the help of the given subcommand(s)
    info     Print CPU temperature and fan RPM
```

## Resource

- https://www.intel.com/content/www/us/en/servers/ipmi/ipmi-home.html
- https://github.com/ipmitool/ipmitool
