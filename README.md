# daship: A Simple Tool for Expanding Dash-Delimited IP Ranges

`daship` is a command-line utility written in Rust, designed to expand IP ranges specified in a dash-delimited format (e.g., `192.168.1.1-192.168.1.5`) into a list of individual IP addresses.

## Installation

### Binary Release
The recommened way to get `daship` is to download the binary for your platform from the [releases](https://github.com/corysabol/daship/releases)

### Build from source

Before installing `daship`, ensure you have Rust and Cargo installed on your system. If you don't have Rust installed, you can get it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

To install `daship`, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/dasip.git
cd daship
cargo build --release
```

The executable will be located in `./target/release/daship`.

## Usage
```
Expands IP addresses from dash delimited ranges as well as CIDR ranges. Can accept a list of ranges from STDIN.

Usage: daship [OPTIONS]

Options:
  -r, --range <RANGE>  IP range in the format x.x.x.x-x.x.x.x
  -h, --help           Print help
  -V, --version        Print version

Examples:
    cat ranges.txt | daship > ips.txt
    daship --range '10.0.0.0-10.0.0.255' > ips.txt
```

`daship` will then print the list of IP addresses within the specified range to the standard output.
Input files must be newline delimited ranges. The file can contain both CIDR and dash delimited ranges.

## Example

```bash
./daship --range 10.0.0.1-10.0.0.5
```

Output:

```
10.0.0.1
10.0.0.2
10.0.0.3
10.0.0.4
10.0.0.5
```

## Contributing

Contributions to `dasip` are welcome! Whether it's bug reports, feature requests, or code contributions, please feel free to reach out or submit a pull request.

## License

`daship` is released under the MIT License. See the LICENSE file for more details.
