
use crate::cfg::{CliConfig, Commands};
use clap::Parser;
use std::process::Command;
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
        Some(Commands::EchoCommand) => {
           let cmd = Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process");
            println!("status: {}", cmd.status);
            println!("stdout: {}", String::from_utf8_lossy(&cmd.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&cmd.stderr));

        }

        None => {
            println!("Invalid command, press '-h' or '--help'");
        }
    }
}
