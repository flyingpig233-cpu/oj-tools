use std::{fs, ops::Add, path::Path, io::{Read, Write}};

use owo_colors::OwoColorize;
use promptly::prompt_default;

use crate::{lib::replace_template, oj_tools::OJTools};

impl OJTools {
    pub fn gen_code(&self, template_path: &Path, mut filename: String) {
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
            fs::File::create(filename.clone()).expect("Error encountered while creating file!");
        fs::File::open(template_path)
            .unwrap()
            .read_to_string(&mut template)
            .unwrap();
        let content = replace_template(
            template,
            file_path,
            self.config.author_name.clone().unwrap_or_default(),
        );
        file.write(content.as_bytes())
            .expect("Error while writing to file");
        println!(
            "{}",
            format!("Success to generate a new file from the template!").green()
        );
    }
}
