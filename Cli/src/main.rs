
use crate::cfg::{CliConfig, Commands};
use clap::Parser;
#[path = "config/cfg.rs"] mod cfg;

#[allow(warnings)]
#[allow(non_snake_case)]
#[tokio::main]
async fn main() {
    match &CliConfig::parse().command {
        Some(Commands::MyGlobalIp(MyGlobalIp)) => {
            println!("MyGlobalIP called {:?}",MyGlobalIp.source);
        },
        Some(Commands::CheckCreds) => {
            println!("Checking credentials")
        },
        Some(Commands::MyInstallCommand(InstallConfig)) => {
            match InstallConfig.operation {
                cfg::CliOperation::Install => {
                    println!("Installing some software : {}",InstallConfig.software)
                },
                cfg::CliOperation::Uninstall => {
                    println!("Uninstall some software : {}",InstallConfig.software)
                }
            }
        },
        None => {
            println!("Invalid command, press '-h' or '--help'");
        }
    }
}
