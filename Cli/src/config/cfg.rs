use clap::{Parser, Subcommand, Args};

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
}


#[derive(Args,Debug)]
pub struct GlobalIpConfig {
    #[arg(long = "source")]
    pub source: String
}

