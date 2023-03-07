// git.rs
//use std::ffi::OsStr;
use std::ffi::OsString;
//use std::path::PathBuf;

use clap::Parser;
//use clap::{AppSettings, Parser, Subcommand};

pub mod util;
//pub mod cfg;
pub mod gen;

//#[command(name = "My CLI Tool"
//    , about = "A brief description of your tool"
//    , author = "Your Name")]
#[derive(Debug, Parser)]
#[command(author, version, about, 
    long_about = r#"RSS4mdBook Usage:
- 0: config mdBook's book.toml, append such as:

    [rss4mdbook]
    url-base = "https://rs.101.so"

- 1: mdbook build
- 2: use `gen` command, append the lasted 4 articles as rss.xml 
    $ rss4mdbook gen path/2/u/mdbook/book.toml
    "#)] // Read from `Cargo.toml`
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Parser)]
pub enum Commands {
/* 
    #[command(about = "book /path/2/u/loc./mdbook/book.toml ~ set loc. writing path...")]
    #[command(arg_required_else_help = false)]
    Cfg {
        #[arg(value_name = "BOOK")]
        book: String,
        #[arg(value_name = "PATH")]
        path: String,
    },

 */
    #[command(about = "/path/2/u/loc./mdbook/book.toml ~ NEED absolutely path")]
    #[command(arg_required_else_help = false)]
    Gen {
        #[arg(value_name = "BOOK")]
        book: String,
    },

    #[command(external_subcommand)]
    External(Vec<OsString>),
}

pub fn run() {
    let _guard = clia_tracing_config::build()
        .filter_level("debug") //fatal,error,warn,info,debug
        .with_ansi(true)
        .to_stdout(false)
        .directory("./log")
        .file_name("debug.log")
        .rolling("daily")
        .init();
    //log::debug!("src/inv/mod:{:?}", _guard);

    let args = Cli::parse();

    //log::debug!("src/inv/mod:{:?}", args);

    match args.command {
    // name.path
        //Commands::Cfg {
        //    book, path }=> cfg::set(book, path),
    // not need arg.
        Commands::Gen {
            book} => gen::exp(book),

    // others
        Commands::External(args) => {
            println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
        }
    }
}
