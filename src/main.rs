use std::env;
use anyhow::Result;
use clap::{Arg, Command};


mod cli;
mod error;
mod subdomains;
mod model;
mod port;
mod common_ports;

fn main() -> Result<()>{
    env::set_var("RUST_LOG", "info,trust_dns_proto=error");
    env_logger::init();

    let cli = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(Command::new("modules").about("List all modules"))
        .subcommand(
            Command::new("scan").about("scan a target").arg(
                Arg::new("target")
                    .help("domain scan")
                    .required(true)
                    .index(1),
            ),
        )
        .arg_required_else_help(true)
        .get_matches();

    if let Some(_) = cli.subcommand_matches("modules") {
        println!("modules list")
    } else if let Some(matches) = cli.subcommand_matches("scan") {
        let target = matches.value_of("target").unwrap();
        // println!("target : {}", target)
        cli::scan(target)?;        
    }

    Ok(())
}
