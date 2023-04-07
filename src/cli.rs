//! An options for blogengine
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, propagate_version = true)]
pub struct ChabloOpt {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Convert markdown to html
    Convert {
        #[clap(long, short)]
        file: PathBuf,
    },
    /// Build the blog
    Build {},
}

impl ChabloOpt {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }
}
