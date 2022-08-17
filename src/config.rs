#[derive(Deserialize, Serialize)]
pub struct OJTConfig {
    pub template_path: Option<String>,
    pub script_path: Option<String>,
    pub test_option: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ProblemConfig {
    pub test_data: Vec<(String, String)>,
    pub code_path: String,
}

impl ProblemConfig {
    pub fn new(test_data: Vec<(String, String)>) -> Self {
        ProblemConfig {
            test_data,
            code_path: String::from("code.cpp"),
        }
    }

    pub fn add(&mut self, a: String, b: String) {
        self.test_data.push((a, b));
    }
}
