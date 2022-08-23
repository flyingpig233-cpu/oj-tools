use std::{fs::{self, File}, process::exit, path::{PathBuf, Path}, io::Write};

use owo_colors::OwoColorize;
use path_calculate::Calculate;

use crate::{config::ProblemConfig, oj_tools::OJTools};
use crate::oj_tools::oj_spider::luogu_oj::LuoguTestData;

impl OJTools {
    pub fn pull_tests(&self, pid: &str, test_data: &Vec<LuoguTestData>) {
        if let Err(_) = fs::create_dir(pid) {
            eprintln!("{}", format!("failed to create {} directory, please check if there is a directory or file with the same name", pid).red());
            exit(1);
        }
        let root_dir = PathBuf::from(pid);
        let mut problem_config = ProblemConfig::new(vec![]);
        for (i, e) in test_data.iter().enumerate() {
            let input_file_name = format!("in{}.txt", i + 1);
            let output_file_name = format!("out{}.txt", i + 1);
            problem_config.add(input_file_name.clone(), output_file_name.clone());
            let mut in_file = File::create(root_dir.join(PathBuf::from(input_file_name)))
                .expect("Failed to create file");
            let mut out_file = File::create(root_dir.join(PathBuf::from(output_file_name)))
                .expect("Failed to create file");
            in_file
                .write(e.test_in.as_bytes())
                .expect("Failed to write file");
            out_file
                .write(e.test_out.as_bytes())
                .expect("Failed to write file");
        }
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
        let code_path = format!("{}/{}", pid, "code.cpp");
        self.gen_code(
            Path::new(key.as_str()).as_absolute_path().unwrap().as_ref(),
            code_path,
        );
        problem_config.code_path = "code.cpp".to_string();
        let config_content = toml::to_string(&problem_config).unwrap();
        let mut config_file =
            File::create(root_dir.join(PathBuf::from(".problem_config.toml"))).unwrap();
        config_file.write(config_content.as_bytes()).unwrap();
        println!("{}{}", "Success to pull tests from ".green(), pid.green());
    }
}
