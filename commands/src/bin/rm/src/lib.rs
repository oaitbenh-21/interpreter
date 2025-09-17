use std::fs;
use std::path::Path;
use std::io::{ Write, Read };

pub struct Rm;
pub struct Cmd {
    pub cmd: String,
    pub args: Vec<String>,
    _stdin: Box<dyn Read>,
    _stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}
impl Cmd {
    pub fn new() -> Self {
        Cmd {
            cmd: String::new(),
            args: Vec::new(),
            _stdin: Box::new(std::io::stdin()),
            _stdout: Box::new(std::io::stdout()),
            stderr: Box::new(std::io::stderr()),
        }
    }
}
pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &mut Cmd);
}

impl Command for Rm {
    fn name(&self) -> &'static str {
        "rm"
    }

    fn run(&self, cmd: &mut Cmd) {
        if cmd.args.is_empty() {
            eprintln!("rm: missing operand");
            return;
        }

        let (recur, targets): (bool, &[String]) = if cmd.args[0] == "-r" {
            if cmd.args.len() < 2 {
                cmd.stderr.write_all(b"rm: missing operand after '-r'\n").unwrap();
                return;
            }
            (true, &cmd.args[1..])
        } else {
            (false, &cmd.args)
        };

        for target in targets {
            let path = Path::new(target);

            let result = if path.is_dir() {
                if recur {
                    fs::remove_dir_all(path)
                } else {
                    cmd.stderr
                        .write_all(
                            format!("rm: cannot remove '{}': Is a directory\n", target).as_bytes()
                        )
                        .unwrap();
                    continue;
                }
            } else {
                fs::remove_file(path)
            };

            if let Err(e) = result {
                cmd.stderr
                    .write_all(format!("rm: cannot remove '{}': {}\n", target, e).as_bytes())
                    .unwrap();
            }
        }
    }
}
