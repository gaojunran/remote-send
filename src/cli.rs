use std::future::Future;
use clap::{Command, Parser, Subcommand};
use s3::serde_types::Object;
use crate::config::Config;
use crate::s3_action::{list_files, RuntimeError};

#[derive(Parser)]
#[command(name = "remote-send", author = "gaojunran")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    #[command(aliases = &["s"])]
    Send {
        path: String,
    },
    #[command(aliases = &["r"])]
    Recv,
    #[command(aliases = &["l", "ls"])]
    List
}

pub(crate) async fn cli(config: &Config) {
    let cli = Cli::parse();
    match cli.command {
        Commands::Send { path } => {

        }
        Commands::Recv => {

        }
        Commands::List => {
            match list_files(
                // &None,
                &Option::from(config.delimiter.clone()),
                &config.bucket,
                &config.prefix).await {
                Ok(objects) => {println!("{objects:#?}")},
                Err(e) => {eprintln!("{e:#?}");}
            }

        }
    };
}