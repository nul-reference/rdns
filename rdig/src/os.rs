use std::io::{BufRead, BufReader};
use std::net::IpAddr;
use std::str::FromStr;

#[cfg(not(target_os = "windows"))]
pub(crate) fn get_host_dns() -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    let resolv = std::fs::File::open("/etc/resolv.conf").map_err(|e| Box::new(e))?;
    let reader = BufReader::new(resolv);
    let nameserver_lines: Vec<String> = reader
        .lines()
        .filter(|line| {
            line.as_ref()
                .unwrap_or(&"".to_string())
                .starts_with("nameserver")
        })
        .map(|l| {
            l.unwrap()
                .trim_start_matches("nameserver")
                .trim()
                .to_string()
        })
        .collect();
    Ok(nameserver_lines
        .iter()
        .map(|ns| IpAddr::from_str(ns))
        .collect::<Result<Vec<IpAddr>, std::net::AddrParseError>>()
        .map_err(|e| Box::new(e))?)
}
