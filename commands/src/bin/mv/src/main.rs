use mv::*;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cmd = Cmd::new();
    cmd.args = args[1..].to_vec();
    Mv.run(&mut cmd);
}