use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser,Debug)]
#[command(author = "Milos Stankovic", version = "1.8.0")]
#[command(about =  "Test CLI to learn in Rust")]
pub struct CliConfig {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand,Debug)]
pub enum Commands {
   MyGlobalIp(GlobalIpConfig),
   #[command(override_usage = "Checking credentials [clDas]  <some_file> \n\
                                Bla bla bla bla \n\
                                 More blah blah")]
   CheckCreds,
    #[command(override_usage = "Example of install command with config operation")]
    MyInstallCommand(InstallConfig),
    #[command(override_usage = "Simple command to mimic echo")]
    EchoCommand,
}


#[derive(Args,Debug)]
pub struct GlobalIpConfig {
    #[arg(long = "source")]
    #[arg(short = 's')]
    pub source: String
}

#[derive(Args,Debug)]
pub struct InstallConfig {
    #[arg(long = "software")]
    pub software:String,
    #[arg(long = "operation")]
    pub operation:CliOperation,
}

#[derive(Debug,Clone,ValueEnum)]
pub enum CliOperation {
    //Install something
    Install,
    //Uninstall something
    Uninstall
}

