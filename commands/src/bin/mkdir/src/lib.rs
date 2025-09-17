use std::fs;
use std::path::Path;
use std::io::{Write,Read};

pub struct Mkdir;
pub struct Cmd {
    pub cmd: String,
    pub args: Vec<String>,
    _stdin: Box<dyn Read>,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}
pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &mut Cmd);
}

impl Cmd {
    pub fn new() -> Self {
        Cmd {
            cmd: String::new(),
            args: Vec::new(),
            _stdin: Box::new(std::io::stdin()),
            stdout: Box::new(std::io::stdout()),
            stderr: Box::new(std::io::stderr()),
        }
    }
}
impl Command for Mkdir {
    fn name(&self) -> &'static str {
        "mkdir"
    }

    fn run(&self, cmd: &mut Cmd) {
        if cmd.args.is_empty() {
            let _ = writeln!(cmd.stderr, "mkdir: missing operand");
            let _ = writeln!(cmd.stderr, "Try 'mkdir --help' for more information.");
            return;
        }

        let mut parents = false;
        let mut verbose = false;
        let mut dirs: Vec<&str> = Vec::new();

        for arg in &cmd.args {
            match arg.as_str() {
                "-p" | "--parents" => parents = true,
                "-v" | "--verbose" => verbose = true,
                "--help" => {
                    let help_text = "\
Usage: mkdir [OPTION]... DIRECTORY...
Create the DIRECTORY(ies), if they do not already exist.

Options:
  -p, --parents     no error if existing, make parent directories as needed
  -v, --verbose     print a message for each created directory
      --help        display this help and exit
";
                    let _ = writeln!(cmd.stdout, "{}", help_text);
                    return;
                }
                _ if arg.starts_with('-') && arg.len() >1 => {
                    let _ = writeln!(cmd.stderr, "mkdir: invalid option -- '{}'", arg);
                    let _ = writeln!(cmd.stderr, "Try 'mkdir --help' for more information.");
                    return;
                }
                _ => dirs.push(arg),
            }
        }

        for dir in dirs {
            let result = if parents {
                fs::create_dir_all(dir)
            } else {
                fs::create_dir(Path::new(dir))
            };

            match result {
                Ok(_) => {
                    if verbose {
                        let _ = writeln!(cmd.stdout, "mkdir: created directory '{}'", dir);
                    }
                }
                Err(e) => {
                    let _ = writeln!(cmd.stderr,"mkdir: cannot create directory '{}': {}",dir, e);
                }
            }
        }
    }
}
