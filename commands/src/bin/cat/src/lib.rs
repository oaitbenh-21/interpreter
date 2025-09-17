use std::fs::{self, File};
use std::io::{ self, Write, Read };
use std::path::PathBuf;
pub struct Cat;
pub struct Cmd {
    pub cmd: String,
    pub args: Vec<String>,
    stdin: Box<dyn Read>,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}
impl Cmd {
    pub fn new() -> Self {
        Cmd {
            cmd: String::new(),
            args: Vec::new(),
            stdin: Box::new(std::io::stdin()),
            stdout: Box::new(std::io::stdout()),
            stderr: Box::new(std::io::stderr()),
        }
    }
}
pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &mut Cmd);
}

impl Command for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }

    fn run(&self, cmd: &mut Cmd) {
        if cmd.args.is_empty() {
            let stdout = cmd.stdout.as_mut();
            let stdin = cmd.stdin.as_mut();
            if let Err(e) = io::copy(&mut *stdin, &mut *stdout) {
                cmd.stderr.write_all(format!("cat: error reading: {}\n", e).as_bytes()).unwrap();
            }
            return;
        }
        for filename in cmd.args.iter() {
            if filename.ends_with("*") {
                let dir = "./".to_string() + &filename[..filename.len() - 1];
                let files = list_files(&dir);
                let cloned = self;
                let mut cloned_cmd = Cmd::new();
                cloned_cmd.args = files;
                cloned.run(&mut cloned_cmd);
                continue;
            }
            match File::open(filename) {
                Ok(mut file) => {
                    let stdout = cmd.stdout.as_mut();
                    if let Err(e) = io::copy(&mut file, &mut *stdout) {
                        cmd.stderr
                            .write_all(
                                format!("cat: error reading '{}': {}\n", filename, e).as_bytes()
                            )
                            .unwrap();
                    }
                }
                Err(e) => {
                    cmd.stderr.write_all(format!("cat: {}: {}\n", filename, e).as_bytes()).unwrap();
                }
            }
        }
    }
}


fn list_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {

        for entry in entries.flatten() {
            let path: PathBuf = entry.path();
            if path.is_file() {
                if let Some(s) = path.to_str() {
                    files.push(s.to_string());
                }
            }
        }
    }

    files
}