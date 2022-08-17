use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

use crate::lib::get_resources;

lazy_static! {
    pub static ref TEST_TXT_RE: Regex = RegexBuilder::new(r#"<code>(.*?)</code>"#)
        .dot_matches_new_line(true)
        .build()
        .unwrap();
}

#[derive(Debug)]
pub struct LuoguTestData {
    pub test_in: String,
    pub test_out: String,
}

impl LuoguTestData {
    pub fn new(test_in: String, test_out: String) -> Self {
        LuoguTestData { test_in, test_out }
    }
}

pub fn get_luogu_test_data(pid: &str) -> Vec<LuoguTestData> {
    let body = get_resources(format!("https://www.luogu.com.cn/problem/{}", pid).as_str());
    let iter = TEST_TXT_RE.find_iter(body.as_str());
    let mut elements: Vec<LuoguTestData> = vec![];
    let mut pre_tmp = String::new();
    for (i, e) in iter.enumerate() {
        let content = e.as_str();
        let close_pos = content.find("</code>").unwrap();
        let plain_text = &content[6..close_pos];
        if i & 1 == 0 {
            pre_tmp = plain_text.to_string()
        } else {
            elements.push(LuoguTestData::new(pre_tmp.clone(), plain_text.to_string()))
        }
    }
    elements
}
