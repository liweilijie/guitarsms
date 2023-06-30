use clap::{Parser, ValueEnum};

pub const CONFIG_FILE: &'static str = "read.md";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Which file will to read.
    #[clap(short = 'c', long, default_value = CONFIG_FILE)]
    pub content_file: String,

    /// What person to send
    #[clap(value_enum)]
    pub person: Person,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Person {
    Magic,
    Trust,
    Me,
}
