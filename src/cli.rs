use reqwest::{blocking::Client, redirect};
use std::time::Duration;

use crate::error::Error;
use crate::model::Subdomain;
use crate::subdomains;
use crate::port;



pub fn scan(target: &str) -> Result<(), Error> {
    log::info!("Scanning: {}", target);
    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    let result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)?
        .into_iter()
        .map(port::scan_ports)
        .collect();


    for subdomain in result {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            println!("    {}", port.port);
        }
        println!();
    }
        
    Ok(())
}