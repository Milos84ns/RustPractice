use std::env;
use std::fmt::{Display, format, Formatter};
use std::fs::read_to_string;
use std::process::exit;
use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
struct Config {
    database:Database,
}

#[derive(Debug,Deserialize)]
struct Database {
    server : String,
    ports: Vec<i32>,
    connection_max: i32,
    enabled: bool
}


fn main() {
 //   let file = env::current_dir().unwrap().to_str().unwrap().to_owned() + "/example.toml";
    let file = "./example.toml";

    let contents = match read_to_string(file) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", file);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    let config: Config = toml::from_str(&contents).unwrap();

    println!("{}",config.database.server);
    println!("{:?}",config.database.ports);
    println!("{}",config.database.connection_max);

}