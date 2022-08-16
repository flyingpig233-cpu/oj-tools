use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::oj_tools::{OJTools, OJType};

mod lib;
mod oj_tools;

#[derive(Subcommand, Debug)]
pub enum Action {
    Test,
    Run {
        #[clap()]
        filename: String,
    },
    Generate {
        #[clap()]
        filename: String,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct HelperCli {
    #[clap(subcommand)]
    action: Action,

    #[clap(long, value_name = "CONFIG PATH")]
    config_path: Option<String>,
}

fn main() {
    let args = HelperCli::parse();
    let app = OJTools::new(args);
    app.run();
}
