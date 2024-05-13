use reqwest::{blocking::Client, redirect};
use futures::stream;
use futures::StreamExt;


use std::time::Duration;
use rayon::prelude::*;

use crate::error::Error;
use crate::model::Subdomain;
use crate::subdomains;
use crate::port;



pub fn scan(target: &str) -> Result<(), Error> {
    log::info!("Scanning: {}", target);

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Building toiko's runtime");
    
    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    // 定义多进程
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    pool.install(|| {
        let result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(port::scan_ports)
            .collect();

        for subdomain in result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("    {}", port.port);
            }
            println!();
        }
    });
        
    Ok(())
}