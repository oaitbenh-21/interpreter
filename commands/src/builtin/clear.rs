use super::Command;
use super::Cmd;

pub struct Clear;
impl Command for Clear {
    fn name(&self) -> &'static str {
        "clear"
    }
    fn run(&self, cmd: &mut Cmd) {
        /*Code	Meaning
        \x1B[3J	Clear scrollback buffer
        \x1B[2J	Clear visible screen
        \x1B[H	Move cursor to top-left */
        cmd.stdout.write_all(b"\x1B[3J \x1B[H \x1B[2J").unwrap();
        cmd.stdout.flush().unwrap();
    }
}
