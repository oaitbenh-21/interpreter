use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct Cp;

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

impl Command for Cp {
    fn name(&self) -> &'static str {
        "cp"
    }

    fn run(&self, cmd: &mut Cmd) {
        let mut recursive = false;
        let mut args: Vec<String> = Vec::new();

        for arg in &cmd.args {
            if arg.starts_with('-') && arg.len() > 1 {
                match arg.as_str() {
                  "-R"| "-r"|"--recursive" => recursive = true,
                  "--h"| "--help" => {
                        let _ = writeln!(cmd.stdout, "Usage: cp [-r] SOURCE... DEST");
                        return;
                    }
                    _ => {
                        let _ = writeln!(
                            cmd.stderr,
                            "cp: invalid option '{}'\nTry 'cp --help' for more information.",
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
                "cp: missing file operand\nTry 'cp --help' for more information."
            );
            return;
        }

        let sources = &args[..args.len() - 1];
        let dest = Path::new(&args[args.len() - 1]);

        if sources.len() > 1 && !dest.is_dir() {
            let _ = writeln!(
                cmd.stderr,
                "cp: target '{}' is not a directory",
                dest.display()
            );
            return;
        }

        for src in sources {
            let src_path = Path::new(src);

            if !src_path.exists() {
                let _ = writeln!(
                    cmd.stderr,
                    "cp: cannot stat '{}': No such file or directory",
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

            if src_path.is_dir() {
                if recursive {
                    if let Err(e) = copy_dir(src_path, &dest_path) {
                        let _ = writeln!(cmd.stderr, "cp: cannot copy directory '{}': {}", src, e);
                    }
                } else {
                    let _ = writeln!(
                        cmd.stderr,
                        "cp: -r not specified; omitting directory '{}'",
                        src
                    );
                }
            } else {
                if let Err(e) = fs::copy(src_path, &dest_path) {
                    let _ = writeln!(
                        cmd.stderr,
                        "cp: failed to copy '{}' to '{}': {}",
                        src,
                        dest_path.display(),
                        e
                    );
                }
            }
        }
    }
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let mut new_dst = dst.to_path_buf();
        new_dst.push(entry.file_name());

        if path.is_dir() {
            copy_dir(&path, &new_dst)?;
        } else {
            fs::copy(&path, &new_dst)?;
        }
    }
    Ok(())
}
