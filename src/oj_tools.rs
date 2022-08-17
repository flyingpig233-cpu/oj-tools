use std::fs;
use std::fs::{File, read_to_string};
use std::io::{Read, stdout, Write};
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::thread;

use arboard::Clipboard;
use owo_colors::OwoColorize;
use path_calculate::Calculate;
use promptly::prompt_default;
use run_script::run_script;

use crate::{Action, HelperCli};
use crate::config::{OJTConfig, ProblemConfig};
use crate::lib::load_config_file;
use crate::lib::prompt_run_status;
use crate::lib::replace_template;
use crate::oj_tools::oj_spider::LuoguTestData;

#[path = "oj_spider/luogu_oj.rs"]
mod oj_spider;

pub static DEFAULT_CONFIG_DIR: &'static str = "~/.oj_tools/";
pub static DEFAULT_CONFIG_FILE_NAME: &'static str = "config.toml";

pub struct OJTools {
    args: HelperCli,
    config: OJTConfig,
    config_file: PathBuf,
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
                    exit(0);
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
            fs::read_to_string(config_file.clone()).expect("Failed to read config file");
        let config = toml::from_str(config_content.as_str()).unwrap();
        Self {
            args,
            config_file,
            config,
        }
    }

    pub fn run(&self) {
        match &self.args.action {
            Action::Test => self.run_test(),
            Action::Run { filename } => {
                let key = match self.config.script_path.clone() {
                    Some(key) => key,
                    None => {
                        println!(
                            "It seems that you are not set script_path!\
                        You should setup a default script, \ntry to run \"oj_tools config\""
                        );
                        return;
                    }
                };
                let script_path = PathBuf::from(key);
                if !script_path.is_absolute() {
                    let config_root = self.config_file.parent().unwrap();
                    self.run_code(Path::new(filename), config_root.join(script_path).as_path());
                } else {
                    self.run_code(Path::new(filename), script_path.as_path());
                }
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
                let tests = oj_spider::get_luogu_test_data(pid.as_str());
                if tests.is_empty() {
                    eprintln!("{}", "Failed to pull tests! Please check the PID is right");
                    exit(1);
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
                clipboard.set_text(file_content)
                    .expect("Failed to set system clipboard");
            }
        }
    }

    fn pull_tests(&self, pid: &str, test_data: &Vec<LuoguTestData>) {
        fs::create_dir(pid).expect("Failed to create directory");
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

    fn run_test(&self) {
        let config_content = load_config_file().unwrap();
        let config: ProblemConfig = toml::from_str(config_content.as_str()).unwrap();
        let (code, _, error) = run_script!(format!(
            "g++ code.cpp -o code {}",
            self.config.test_option.clone().unwrap_or_default()
        )
        .as_str())
            .unwrap();
        if code != 0 {
            eprintln!("{}", error);
            println!("{}", "CE".magenta());
            return;
        }
        let mut handles = vec![];
        for (i, (input_file, output_file)) in config.test_data.into_iter().enumerate() {
            handles.push(thread::spawn(move || {
                let mut lock = stdout().lock();
                writeln!(lock, "{}", format!("on test #{}", i + 1).cyan()).unwrap();
                // run
                let (code, _, error) =
                    run_script!(format!(r#"./code < {} > {}"#, input_file, "out.txt").as_str())
                        .unwrap();

                if code != 0 {
                    writeln!(lock, "{}", error).unwrap();
                    writeln!(lock, "{}", "RE".purple()).unwrap();
                    return;
                }

                // test data
                let out_content = fs::read_to_string("out.txt").unwrap();
                let answer = fs::read_to_string(output_file).unwrap();
                let mut wa = false;
                for diff in diff::lines(out_content.trim(), answer.trim()) {
                    match diff {
                        diff::Result::Left(_) => {
                            wa = true;
                            break;
                        }
                        diff::Result::Both(_, _) => {}
                        diff::Result::Right(_) => {
                            wa = true;
                            break;
                        }
                    }
                }
                if wa {
                    writeln!(lock, "{}", "WA".red()).unwrap();
                } else {
                    writeln!(lock, "{}", "AC".green()).unwrap();
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn gen_code(&self, template_path: &Path, mut filename: String) {
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
        // If the input has no extension, it will automatically add .cpp
        if let None = filename.find(".") {
            filename = filename.add(".cpp");
        }

        let mut template = String::new();
        let file_path = Path::new(filename.as_str());

        if file_path.exists() {
            match prompt_default(
                format!(
                    "The file {} is already exists, do you want to overwrite it?",
                    filename
                )
                    .as_str(),
                false,
            ) {
                Ok(true) => {}
                _ => {
                    return;
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
        if !filepath.exists() {
            eprintln!(
                "{}",
                format!("File {} does not exist", filepath.to_str().unwrap()).red()
            );
            return;
        }
        println!("{:?}", script_path);
        let template = fs::read_to_string(script_path).unwrap();
        let content = replace_template(template, filepath);
        println!("{}", "Running script".yellow());
        let (code, output, error) = run_script!(content).unwrap();
        prompt_run_status(code, output, error);
    }
}
