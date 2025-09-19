use std::env;
use std::sync::Arc;
use std::path::{ Path, PathBuf };
use std::process::{ Child, Command, Stdio };
use std::fs::{ OpenOptions, File };
use std::sync::Mutex;

pub use commands::{ Cmd, Registry, jobs };

pub fn exec(commands: Vec<Cmd>) {
    let mut prev_stdout = None;
    let mut children: Vec<Arc<Mutex<Child>>> = Vec::new();
    let mut cmd_iter = commands.into_iter().peekable();

    while let Some(cmd) = cmd_iter.next() {
        let registry = Registry::new();
        if registry.has(&cmd) {
            if cmd_iter.peek().is_some() || prev_stdout.is_some() {
                eprintln!("Built-in commands cannot be used in pipelines");
                continue;
            }

            registry.run(cmd);
            continue;
        }
        let executable = match find_executable(&cmd.cmd) {
            Some(path) => path,
            None => {
                eprintln!("command not found: {}", cmd.cmd);
                continue;
            }
        };

        let stdin = match prev_stdout.take() {
            Some(output) => Stdio::from(output),
            None => Stdio::inherit(),
        };

        let stdout = if cmd_iter.peek().is_some() {
            Stdio::piped()
        } else if cmd.background {
            let output: Result<File, _> = OpenOptions::new().write(true).open("/dev/null");
            let file = match output {
                Ok(f) => Stdio::from(f),
                Err(_) => Stdio::inherit(),
            };
            file
        } else {
            Stdio::inherit()
        };
        let child = match
            Command::new(&executable).args(&cmd.args).stdin(stdin).stdout(stdout).spawn()
        {
            Ok(child) if !cmd.background => child,
            Err(err) => {
                eprintln!("failed to execute '{}': {}", cmd.cmd, err);
                continue;
            }
            _ => {
                continue;
            }
        };
        let child_arc = Arc::new(Mutex::new(child));
        prev_stdout = child_arc.lock().unwrap().stdout.take();
        children.push(child_arc);
    }
    for child in children {
        if let Err(err) = child.lock().unwrap().wait() {
            eprintln!("failed while waiting for process: {}", err);
        }
    }
}

fn find_executable(cmd: &str) -> Option<PathBuf> {
    let dir = env::var("DIR").ok()?;
    let candidate = Path::new(&dir).join(cmd);
    if candidate.is_file() {
        Some(candidate)
    } else {
        None
    }
}
