use clap::{Parser, Subcommand};

use crate::oj_tools::{OJTools, OJType};

mod oj_tools;

#[derive(Subcommand, Debug)]
pub enum Action {
    Test,
    Run {
        #[clap()]
        filename: String
    },
    Generate,
}


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct HelperCli {
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let args = HelperCli::parse();
    let mut app = OJTools::new(args, OJType::Luogu, Option::None);
    app.run();
}