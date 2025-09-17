use std::fs;
use std::io::{ Read, Write };
use std::path::{ Path, PathBuf };

pub struct Mv;

pub struct Cmd {
    pub cmd: String,
    pub args: Vec<String>,
    _stdin: Box<dyn Read>,
    pub stdout: Box<dyn Write>,
    pub stderr: Box<dyn Write>,
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

impl Command for Mv {
    fn name(&self) -> &'static str {
        "mv"
    }

    fn run(&self, cmd: &mut Cmd) {
        let mut args: Vec<String> = Vec::new();

        for arg in &cmd.args {
            if arg.starts_with('-') && arg.len() > 1 {
                match arg.as_str() {
                    "--h" | "--help" => {
                        let _ = writeln!(cmd.stdout, "Usage: mv [OPTION]... SOURCE... DEST");
                        return;
                    }
                    _ => {
                        let _ = writeln!(
                            cmd.stderr,
                            "mv: invalid option '{}'\nTry 'mv --help' for more information.",
                            arg
                        );
                        return;
                    }
                }
            } else {
                args.push(arg.clone());
            }
        }

        if args.len() < 2 {
            let _ = writeln!(
                cmd.stderr,
                "mv: missing file operand\nTry 'mv --help' for more information."
            );
            return;
        }

        let sources = &args[..args.len() - 1];
        let dest = Path::new(&args[args.len() - 1]);

        if sources.len() > 1 && !dest.is_dir() {
            let _ = writeln!(cmd.stderr, "mv: target '{}' is not a directory", dest.display());
            return;
        }

        for src in sources {
            let src_path = Path::new(src);

            if !src_path.exists() {
                let _ = writeln!(
                    cmd.stderr,
                    "mv: cannot stat '{}': No such file or directory",
                    src
                );
                continue;
            }

            let mut dest_path = PathBuf::from(dest);
            if dest.is_dir() {
                if let Some(fname) = src_path.file_name() {
                    dest_path.push(fname);
                }
            }

            if let Err(e) = fs::rename(src_path, &dest_path) {
                let _ = writeln!(cmd.stderr, "mv: cannot move directory '{}': {}", src, e);
            }

        }
    }
}
