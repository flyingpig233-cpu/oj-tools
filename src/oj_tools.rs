use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use config::Config;
use owo_colors::OwoColorize;
use run_script::ScriptOptions;

use crate::{Action, HelperCli};

pub enum OJType {
    Luogu,
    Codeforces,
}

pub struct OJTools {
    args: HelperCli,
    oj_type: OJType,
    config: HashMap<String, String>,
}

impl OJTools {
    pub fn new(args: HelperCli, oj_type: OJType, config_path: Option<String>) -> Self {
        let config = Config::builder()
            .add_source(config::File::with_name(
                dirs::home_dir().unwrap()
                    .join(PathBuf::from(".oj_tools/config.toml")).to_str().unwrap()))
            .build()
            .unwrap()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();
        Self {
            args,
            oj_type,
            config,
        }
    }

    pub fn run(&self) {
        match &self.args.action {
            Action::Test => { self.run_test() }
            Action::Run { filename, .. } => {
                self.run_code(Path::new(filename),
                              Path::new(self.config.get("script_path")
                                  .unwrap()
                                  .replace("~", dirs::home_dir().unwrap().to_str().unwrap())
                                  .as_str()));
            }

            Action::Generate => { self.gen_code() }
            _ => {}
        }
    }

    fn run_test(&self) {}

    fn run_code(&self, filepath: &Path, script_path: &Path) {
        let mut template = String::new();
        println!("{:?}", script_path);
        fs::File::open(script_path).unwrap().read_to_string(&mut template).unwrap();
        let content = template
            .replace("$fileNameWithoutExt$", filepath.file_stem().unwrap().to_str().unwrap())
            .replace("$fileName$", filepath.to_str().unwrap())
            .replace("$dir$", filepath.parent().unwrap().to_str().unwrap());
        println!("{}", content);
        let options = ScriptOptions::new();
        let args = vec![];

        println!("{}", "Running script".yellow());
        let (code, output, error) = run_script::run(
            &*content,
            &args,
            &options,
        ).unwrap();
        println!("{}", output);
        match code {
            0 => { println!("{}", "Success!".green()); }
            _ => { println!("{}", format!("Failed to run the script! Exit code: {}", code)).bright_red(); }
        }

        // run part
    }

    fn gen_code(&self) {}
}