use std::time::Duration;
use dns_lookup::lookup_host;

async fn get_ping_with_ip(ip: &std::net::IpAddr, payload: &[u8], time: &mut Vec<Duration>) {
    match surge_ping::ping(*ip, payload).await {
        Ok(dur) => {
            time.push(dur.1);
        }
        Err(_) => {}
    }
}

#[tokio::main]
pub async fn get_ping(host: &str, is_ip: bool) -> Result<Duration, &str> {
    let mut time: Vec<Duration> = Vec::new();

    let payload = [0; 8];

    if is_ip {
        match host.parse() {
            Ok(ip) => {
                get_ping_with_ip(&ip, &payload, &mut time).await;
            }
            Err(_) => {
                return Err("Wrong ip.");
            }
        }
    }
    else {
        match lookup_host(host) {
            Ok(ips) => {
                for ip in ips {
                    get_ping_with_ip(&ip, &payload, &mut time).await;
                }
            }
            Err(_) => {
                return Err("No such host is known.");
            }
        }
    }
    
    
    let mut sum = Duration::ZERO;
    for num in &time {
        sum += *num;
    }
    if !time.is_empty() {
        Ok(sum / time.len().try_into().unwrap())
    }
    else {
        Err("No ping elements.")
    }
}