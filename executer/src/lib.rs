use std::env;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

pub use commands::{Cmd, Registry};

pub fn exec(commands: Vec<Cmd>) {
    let mut prev_stdout = None;
    let mut children: Vec<Child> = Vec::new();
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
        } else {
            Stdio::inherit()
        };
        let mut child = match Command::new(&executable)
            .args(&cmd.args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()
        {
            Ok(child) => child,
            Err(err) => {
                eprintln!("failed to execute '{}': {}", cmd.cmd, err);
                continue;
            }
        };

        prev_stdout = child.stdout.take();
        children.push(child);
    }
    for mut child in children {
        if let Err(err) = child.wait() {
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
