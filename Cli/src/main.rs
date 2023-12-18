
use crate::cfg::{CliConfig, Commands};
use clap::Parser;
#[path = "config/cfg.rs"] mod cfg;

#[allow(warnings)]
#[allow(non_snake_case)]
#[tokio::main]
async fn main() {
    match &CliConfig::parse().command {
        Some(Commands::MyGlobalIp(MyGlobalIp)) => {
            println!("MyGlobalIP called");
        },
        Some(Commands::CheckCreds) => {
            println!("Checking credentials")
        }
        None => {
            println!("Invalid command, press '-h' or '--help'");
        }
    }
}
