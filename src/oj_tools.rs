use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, Read, stdin, stdout, Write};
use std::path::{Path, PathBuf};

use config::Config;
use owo_colors::OwoColorize;
use path_calculate::Calculate;
use run_script::ScriptOptions;

use oj_tools::replace_template;

use crate::{Action, HelperCli};

pub enum OJType {
    Luogu,
}

pub struct OJTools {
    args: HelperCli,
    config: HashMap<String, String>,
}

impl OJTools {
    pub fn new(args: HelperCli) -> Self {
        let config_path = match args.config_path {
            Some(ref T) => { T.clone() }
            None => { String::from("~/.config/oj_tools/config.toml") }
        };
        let config = Config::builder()
            .add_source(config::File::with_name(
                config_path.as_str()
            ))
            .build()
            .unwrap()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();
        Self { args, config }
    }

    pub fn run(&self) {
        match &self.args.action {
            Action::Test => self.run_test(),
            Action::Run { filename } => {
                let key = match self.config.get("script_path") {
                    Some(T) => T,
                    None => {
                        println!(
                            "It seems that you are not set script_path!\
                        You should setup a default script, \ntry to run \"oj_tools config\""
                        );
                        return;
                    }
                };
                let script_path = Path::new(key).as_absolute_path().unwrap();
                self.run_code(Path::new(filename), script_path.as_ref());
            }

            Action::Generate { filename } => {
                let key = match self.config.get("template_path") {
                    Some(T) => T,
                    None => {
                        println!(
                            "It seems that you are not set template_path!\
                        You should setup a default script, \ntry to run \"oj_tools config\""
                        );
                        return;
                    }
                };
                let template_path = Path::new(key).as_absolute_path().unwrap();
                self.gen_code(template_path.as_ref(), filename.clone());
            }
        }
    }

    fn run_test(&self) {}

    fn gen_code(&self, template_path: &Path, filename: String) {
        if !template_path.exists() {
            println!(
                "{}",
                format!(
                    "template file {} does not exist",
                    template_path.to_str().unwrap()
                )
                    .red()
            );
            return;
        }
        let mut template = String::new();
        let file_path = Path::new(filename.as_str());
        if file_path.exists() {
            loop {
                print!(
                    "The file {} is already exists, do you want to overwrite it? (Yy/Nn)",
                    filename
                );
                stdout().flush().expect("flush failed!");
                let mut buff = String::new();
                stdin().lock().read_line(&mut buff).unwrap();
                match buff.trim().to_lowercase().as_str() {
                    "y" => break,
                    "n" => {
                        return;
                    }
                    _ => {
                        println!("Unknown input, please input again");
                    }
                }
            }
        }
        let mut file =
            File::create(filename.clone()).expect("Error encountered while creating file!");
        File::open(template_path)
            .unwrap()
            .read_to_string(&mut template)
            .unwrap();
        let content = replace_template(template, file_path);
        file.write(content.as_bytes())
            .expect("Error while writing to file");
        println!(
            "{}",
            format!("Success to generate a new file from the template!").green()
        );
    }

    fn run_code(&self, filepath: &Path, script_path: &Path) {
        let mut template = String::new();
        if !filepath.exists() {
            println!(
                "{}",
                format!("File {} does not exist", filepath.to_str().unwrap()).red()
            );
            return;
        }
        fs::File::open(script_path)
            .unwrap()
            .read_to_string(&mut template)
            .unwrap();
        let content = replace_template(template, filepath);
        let options = ScriptOptions::new();
        let args = vec![];
        println!("{}", "Running script".yellow());
        let (code, output, _) = run_script::run(&*content, &args, &options).unwrap();
        print!("{}", output);
        match code {
            0 => {
                println!("{}", "Success!".green());
            }
            _ => {
                println!(
                    "{}",
                    format!("Failed to run the script! Exit code: {}", code).bright_red()
                )
            }
        }
    }
}
