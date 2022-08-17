use clap::{Parser, Subcommand};

use crate::oj_tools::OJTools;

mod config;
mod lib;
mod oj_tools;

#[derive(Subcommand)]
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
    Pull {
        #[clap()]
        pid: String,
    },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct HelperCli {
    #[clap(subcommand)]
    action: Action,

    #[clap(long, value_name = "CONFIG FILE")]
    config_file: Option<String>,
}

fn main() {
    let args = HelperCli::parse();
    let app = OJTools::new(args);
    app.run();
}
