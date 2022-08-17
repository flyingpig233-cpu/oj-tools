use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

use chrono::Datelike;
use owo_colors::OwoColorize;
use tokio::io;

pub fn replace_template(template: String, filepath: &Path) -> String {
    let time = chrono::offset::Local::now();
    template
        .replace(
            "$fileNameWithoutExt$",
            filepath.file_stem().unwrap().to_str().unwrap(),
        )
        .replace("$fileName$", filepath.to_str().unwrap())
        .replace("$dir$", filepath.parent().unwrap().to_str().unwrap())
        .replace("$year$", time.date().year().to_string().as_str())
        .replace("$month$", time.date().month().to_string().as_str())
        .replace("$day$", time.date().day().to_string().as_str())
        .replace("$author$", "shit")
}

pub fn get_resources(url: &str) -> String {
    let resp =
        reqwest::blocking::get(url).expect(format!("Failed to get response from {}", url).as_str());
    resp.text().unwrap()
}

pub fn load_config_file() -> io::Result<String> {
    let config_file_path = PathBuf::from(".problem_config.toml");
    if !config_file_path.exists() {
        eprintln!(
            "{}",
            "You should run `oi_tools pull #PID` first! There are not any config file".red()
        );
        exit(1);
    }
    fs::read_to_string(config_file_path)
}

pub fn prompt_run_status(code: i32, output: String, error: String) {
    match code {
        0 => {
            print!("{}", output);
            println!("{}", "Success!".green());
        }
        _ => {
            eprintln!("{}", format!("{:?}", error).bright_red());
            eprintln!(
                "{}",
                format!("Failed to run the script! Exit code: {}", code).bright_red()
            );
        }
    }
}
