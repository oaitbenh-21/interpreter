mod parsing;
use parsing::Lexer;
use parsing::{ AstNode, Command };
pub use executer::{ exec, Cmd };
use std::io;

pub fn evaluate(user_input: &str) {
    let mut cmd_line: String = user_input.to_string().trim().to_string();
    if !cmd_line.ends_with(";") {
        cmd_line += ";";
    }
    let lexer_tokens = Lexer::new(&cmd_line);
    let ast_data = AstNode::new(lexer_tokens);
    for node in ast_data {
        if let AstNode::Pipeline(commands) = node {
            let mut cmds: Vec<Cmd> = Vec::new();
            for c in commands {
                if c.program.len() != 0 {
                    cmds.push(to_cmd(c.clone()));
                }
            }
            println!("pipe: {:?}", cmds);
            exec(cmds);
        } else if let AstNode::Command(command) = node && command.program.len() != 0 {
            exec(vec![to_cmd(command)]);
        }
    }
}

fn to_cmd(command: Command) -> Cmd {
    Cmd {
        cmd: command.program,
        args: command.arguments,
        stdin: Box::new(io::stdin()),
        stdout: Box::new(io::stdout()),
        stderr: Box::new(io::stderr()),
    }
}
