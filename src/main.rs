use clap::Parser;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP range in the format x.x.x.x-x.x.x.x
    #[arg(long, short, value_parser = parse_ip_range)]
    range: (Ipv4Addr, Ipv4Addr),
}

fn parse_ip_range(s: &str) -> Result<(Ipv4Addr, Ipv4Addr), &'static str> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return Err("Range must be in the format x.x.x.x-x.x.x.x");
    }
    let start_ip = parts[0]
        .parse::<Ipv4Addr>()
        .map_err(|_| "Invalid start IP")?;
    let end_ip = parts[1].parse::<Ipv4Addr>().map_err(|_| "Invalid end IP")?;
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

    let ip_list = expand_ip_range(args.range.0, args.range.1);

    for ip in ip_list {
        println!("{}", ip);
    }
}
