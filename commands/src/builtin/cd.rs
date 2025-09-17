use super::Command;
use std::env;
pub struct Cd;
use super::Cmd;



impl Command for Cd {
    fn name(&self) -> &'static str { "cd" }
    fn run(&self, cmd:  &mut Cmd) {
        if let Some(path) = cmd.args.get(0) {
            if let Err(e) = env::set_current_dir(path) {
                let _ = writeln!(cmd.stderr, "cd: {}: {}", path, e);
            }
        } else {
            let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
            if let Err(e) = env::set_current_dir(&home) {
                let _ = writeln!(cmd.stderr, "cd: {}: {}", home, e);
            }
        }
    }
}