use std::fs;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use arboard::Clipboard;
use owo_colors::OwoColorize;
use path_calculate::Calculate;

use crate::config::{OJTConfig, ProblemConfig};
use crate::lib::load_config_file;
use crate::{Action, HelperCli};
mod actions;
mod oj_spider;

pub static DEFAULT_CONFIG_DIR: &'static str = "~/.oj_tools/";
pub static DEFAULT_CONFIG_FILE_NAME: &'static str = "config.toml";

pub struct OJTools {
    args: HelperCli,
    config: OJTConfig,
}

impl OJTools {
    pub fn new(args: HelperCli) -> Self {
        let config_dir = PathBuf::from(DEFAULT_CONFIG_DIR);
        let abs_config_dir = config_dir.as_absolute_path().unwrap();
        let config_file = match args.config_file {
            Some(ref file) => {
                let path = PathBuf::from(file.clone());
                if !path.exists() {
                    eprintln!(
                        "{}",
                        format!("The config file {} does not exist!", file).red()
                    );
                    std::process::exit(0);
                }
                path
            }
            None => {
                if !abs_config_dir.exists() {
                    fs::create_dir_all(DEFAULT_CONFIG_DIR)
                        .expect("failed to create config directory");
                }
                abs_config_dir.join(PathBuf::from(DEFAULT_CONFIG_FILE_NAME))
            }
        };
        let config_content =
            read_to_string(config_file.clone()).expect("Failed to read config file");
        let config = toml::from_str(config_content.as_str()).unwrap();
        Self { args, config }
    }

    pub fn run(&self) {
        match &self.args.action {
            Action::Test => self.run_test(),
            Action::Run { filename } => {
                let file_path;
                if filename.is_empty() {
                    let config_content = load_config_file().unwrap();
                    let config: ProblemConfig = toml::from_str(config_content.as_str()).unwrap();
                    file_path = PathBuf::from(config.code_path.as_str());
                } else {
                    file_path = PathBuf::from(filename);
                }
                self.run_code(
                    file_path.as_path(),
                    self.config.run_option.clone().unwrap_or_default(),
                );
            }

            Action::Gen { filename } => {
                let key = match self.config.template_path.clone() {
                    Some(key) => key,
                    None => {
                        println!(
                            "It seems that you are not set template_path!\
                        You should setup a default script, \ntry to run \"oj_tools config\""
                        );
                        return;
                    }
                };
                let template_path = Path::new(key.as_str()).as_absolute_path().unwrap();
                self.gen_code(template_path.as_ref(), filename.clone());
            }
            Action::Pull { pid } => {
                let pid = pid.to_uppercase();
                let tests = oj_spider::luogu_oj::get_luogu_test_data(pid.as_str());
                if tests.is_empty() {
                    eprintln!("{}", "Failed to pull tests! Please check the PID is right");
                    std::process::exit(1);
                }
                self.pull_tests(pid.as_str(), &tests);
            }
            Action::Config => {}
            Action::Copy { filename } => {
                let file_content: String;
                if filename.is_empty() {
                    let config_content = load_config_file().unwrap();
                    let config: ProblemConfig = toml::from_str(config_content.as_str()).unwrap();
                    file_content = read_to_string(config.code_path).unwrap();
                } else {
                    file_content = read_to_string(filename).unwrap();
                }
                println!("{}", file_content);
                let mut clipboard = Clipboard::new().unwrap();
                clipboard
                    .set_text(file_content)
                    .expect("Failed to set system clipboard");
            }
        }
    }

}
