use std::io::{ Write, Read };
pub struct Ls;
pub struct Cmd {
    pub cmd: String,
    pub args: Vec<String>,
    _stdin: Box<dyn Read>,
    stdout: Box<dyn Write>,
    _stderr: Box<dyn Write>,
}
impl Cmd {
    pub fn new() -> Self {
        Cmd {
            cmd: String::new(),
            args: Vec::new(),
            _stdin: Box::new(std::io::stdin()),
            stdout: Box::new(std::io::stdout()),
            _stderr: Box::new(std::io::stderr()),
        }
    }
}
pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &mut Cmd);
}

pub struct Echo;
impl Command for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }
    fn run(&self, cmd: &mut Cmd) {
        let lines:Vec<Vec<String>> = cmd.args.join(" ").split("\\n").map(|s| s.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()).collect();
        for line in lines {
            let _ = writeln!(cmd.stdout, "{}", line.join(" "));
        }
    }
}
