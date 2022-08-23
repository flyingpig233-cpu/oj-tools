use clap::{Parser, Subcommand};

mod config;
mod lib;
mod oj_tools;

#[derive(Subcommand)]
pub enum Action {
    /// Test code
    Test,

    /// Run code
    Run {
        #[clap(default_value_t = String::new())]
        filename: String,
    },

    /// Generate code from the template
    Gen {
        #[clap()]
        filename: String,
    },

    /// Pull tests from OJs
    Pull {
        #[clap()]
        pid: String,
    },

    /// Config oj_tools
    Config,

    /// Copy problem file to clipboard
    Copy {
        #[clap(default_value_t = String::new())]
        filename: String,
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
    let app = oj_tools::OJTools::new(args);
    app.run();
}
