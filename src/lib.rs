use std::path::{Path, PathBuf};

use chrono::Datelike;

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
