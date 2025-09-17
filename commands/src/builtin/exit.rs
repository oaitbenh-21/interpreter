use super::Command;
use super::Cmd;
pub struct Exit;

impl Command for Exit {
    fn name(&self) -> &'static str { "exit" }

    fn run(&self, cmd: &mut Cmd) {
        let mut code = 0;
        if let Some(arg) = cmd.args.get(0) {
            if let Ok(n) = arg.parse::<i32>() {
                code = n;
            }
        }
        std::process::exit(code);
    }
}