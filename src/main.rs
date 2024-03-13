use clap::Parser;
use ipnetwork::{IpNetwork, Ipv4Network};
use std::io::{self, BufRead};
use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[command(
    author = "Cory Sabol",
    version = "v0.2.0",
    about = "Expands IP addresses from dash delimited ranges as well as CIDR ranges. Can accept a list of ranges from STDIN.",
    after_help = "Examples:
    cat ranges.txt | daship > ips.txt
    daship --range '10.0.0.0-10.0.0.255' > ips.txt
    "
)]
struct Args {
    /// IP range in the format x.x.x.x-x.x.x.x
    #[arg(long, short)]
    range: Option<String>,
}

fn parse_ip_range(s: &str) -> Result<(Ipv4Addr, Ipv4Addr), &'static str> {
    let (start_ip, end_ip) = if s.contains('-') {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err("Range must be in the format x.x.x.x-x.x.x.x");
        }
        let start_ip = parts[0]
            .parse::<Ipv4Addr>()
            .map_err(|_| "Invalid start IP")?;
        let end_ip = parts[1].parse::<Ipv4Addr>().map_err(|_| "Invalid end IP")?;
        (start_ip, end_ip)
    } else {
        // assume cidr and handle that
        let mut network = s.parse::<Ipv4Network>().unwrap().iter();
        let start_ip = network.next();
        let end_ip = network.last();
        (start_ip.unwrap(), end_ip.unwrap())
    };
    Ok((start_ip, end_ip))
}

fn expand_ip_range(start_ip: Ipv4Addr, end_ip: Ipv4Addr) -> Vec<Ipv4Addr> {
    let mut current_ip = start_ip;
    let mut ips = Vec::new();

    while current_ip <= end_ip {
        ips.push(current_ip);
        let next_ip = u32::from(current_ip).wrapping_add(1);
        current_ip = Ipv4Addr::from(next_ip);
    }

    ips
}

fn main() {
    let args = Args::parse();

    // Did we get a value for range?
    match args.range {
        Some(s) => {
            // Try to parse the range
            let (start_ip, end_ip) = match parse_ip_range(&s) {
                Ok(r) => r,
                Err(err) => {
                    todo!("handle err {:?}", err)
                }
            };

            // Display the IP addresses
            let ip_list = expand_ip_range(start_ip, end_ip);
            for ip in ip_list {
                println!("{}", ip);
            }
        }
        None => {
            // Assume we got input from stdin as list of cidr or dash ranges
            let stdin = io::stdin();
            for range in stdin.lock().lines() {
                // parse the current range
                // TODO: Handle error case of Result from lines iter
                let (start_ip, end_ip) = match parse_ip_range(&range.unwrap()) {
                    Ok(r) => r,
                    Err(err) => todo!("Handle error {:?}", err),
                };

                // output the ips
                let ip_list = expand_ip_range(start_ip, end_ip);
                for ip in ip_list {
                    println!("{}", ip);
                }
            }
        }
    }
}
