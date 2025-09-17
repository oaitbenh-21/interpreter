use super::Command;
use super::Cmd;
pub struct Pwd;

impl Command for Pwd {
    fn name(&self) -> &'static str { "pwd" }

    fn run(&self, cmd: &mut Cmd) {
        let cwd = std::env::current_dir().unwrap();
        let cwd_str = cwd.to_string_lossy();
        cmd.stdout.write_all(format!("{}\n", cwd_str).as_bytes()).unwrap();
    }
}