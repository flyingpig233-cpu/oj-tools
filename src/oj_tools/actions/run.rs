use std::{path::Path, process::{Command, exit}};

use owo_colors::OwoColorize;

use crate::{lib::prompt_run_status, oj_tools::OJTools};

impl OJTools {
    pub fn run_code(&self, filepath: &Path, compiler_option: String) {
        if !filepath.exists() {
            eprintln!(
                "{}",
                format!("File {} does not exist", filepath.to_str().unwrap()).red()
            );
            return;
        }
        let compiler;
        if let Some(ref cc) = self.config.cc {
            compiler = cc.clone();
        } else {
            compiler = "g++".to_string();
        }
        let fullname = filepath.to_str().unwrap();
        let name_without_ext = filepath.file_stem().unwrap().to_str().unwrap();
        let command = format!(
            "{} {} -o {} {}",
            compiler, fullname, name_without_ext, compiler_option
        );
        println!("{}", format!("{}", command).bright_cyan());
        let (code, output, error) = run_script::run_script!(command).unwrap();
        prompt_run_status(code, output, error, "Compile success", "Failed to build");

        if let Err(_) = Command::new(format!("./{}", name_without_ext)).status() {
            eprintln!("{}", format!("Runtime error occurred. ").bold().red());
            exit(1);
        }
        println!("{}", "Successful execution".green());
    }
}
