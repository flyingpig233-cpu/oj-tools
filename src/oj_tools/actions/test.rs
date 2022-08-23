use std::{io::{self, Write}, process::{Stdio, Output}, sync::mpsc::{Sender, Receiver, channel}, thread, fs};

use owo_colors::OwoColorize;
use sysinfo::{Pid, PidExt, SystemExt, ProcessExt};

use crate::{oj_tools::OJTools, config::ProblemConfig, lib::load_config_file};

impl OJTools {
    pub fn run_test(&self) {
        let config_content = load_config_file().unwrap();
        let config: ProblemConfig = toml::from_str(config_content.as_str()).unwrap();
        let script_content = format!(
            "g++ code.cpp -o code {}",
            self.config.test_option.clone().unwrap_or_default()
        );
        println!("{}", script_content);
        let (code, _, error) = run_script::run_script!(script_content.as_str()).unwrap();
        if code != 0 {
            eprintln!("{}", error);
            println!("{}", "CE".bold().magenta());
            return;
        }
        let mut handles = vec![];
        for (i, (input_file, output_file)) in config.test_data.into_iter().enumerate() {
            handles.push(std::thread::spawn(move || {
                let mut lock = io::stdout().lock();

                // run
                let input_file_content = fs::read_to_string(input_file).unwrap();
                let mut command = std::process::Command::new("./code")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                let (tx, rx): (Sender<Output>, Receiver<Output>) = channel();
                let pid = Pid::from_u32(command.id());
                if let Some(mut stdin) = command.stdin.take() {
                    stdin.write_all(input_file_content.as_bytes()).unwrap();
                }

                thread::spawn(move || {
                    tx.send(command.wait_with_output().unwrap()).unwrap();
                });
                let mut s = sysinfo::System::new();
                s.refresh_process(pid);
                let mut memory_max_usage = s.process(pid).unwrap().memory();
                let mut count = 0;
                write!(lock, "{}", format!("Test #{} ==> ", i + 1).bright_cyan()).unwrap();
                loop {
                    let res = rx.try_recv();
                    if let Ok(output) = res {
                        if !output.status.success() {
                            writeln!(lock, "{}", String::from_utf8(output.stderr).unwrap())
                                .unwrap();
                            write!(lock, "{}", "RE".bold().purple()).unwrap();
                            writeln!(lock, " ... {:.3}MB", memory_max_usage as f32 / 1024.)
                                .unwrap();
                            return;
                        }

                        // test data
                        let out_content =
                            String::from_utf8(output.stdout).unwrap().trim().to_string();
                        let answer = fs::read_to_string(output_file.clone())
                            .unwrap()
                            .trim()
                            .to_string();
                        if out_content.eq(&answer) {
                            write!(lock, "{}", "AC".bold().green()).unwrap();
                            writeln!(lock, " ... {:.3}MB", memory_max_usage as f32 / 1024.)
                                .unwrap();
                        } else {
                            write!(lock, "{}", "WA".bold().red()).unwrap();
                            writeln!(lock, " ... {:.3}MB", memory_max_usage as f32 / 1024.)
                                .unwrap();
                            prettydiff::diff_lines(out_content.as_str(), answer.as_str())
                                .names("Your answer", "Answer")
                                .set_show_lines(true)
                                .set_diff_only(false)
                                .set_align_new_lines(true)
                                .prettytable();
                        }
                        break;
                    } else {
                        if count > 50000 {
                            write!(lock, "{}", "TLE".purple()).unwrap();
                            writeln!(lock, " ... {:.3}MB", memory_max_usage as f32 / 1024.)
                                .unwrap();
                            break;
                        }
                        s.refresh_process(pid);
                        if let Some(process) = s.process(pid) {
                            memory_max_usage = process.memory().max(memory_max_usage);
                        }
                        count += 1;
                    }
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
