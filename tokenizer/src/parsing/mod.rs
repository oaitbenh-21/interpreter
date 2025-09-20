mod lexing;
pub use lexing::Lexer;

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub program: String,
    pub arguments: Vec<String>,
    pub background: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AstNode {
    Pipeline(Vec<Command>),
    Command(Command),
    None,
}

impl AstNode {
    pub fn new(lexer: Lexer) -> Vec<AstNode> {
        let mut sequence: Vec<AstNode> = Vec::new();
        let mut current_cmd: Command = Command {
            program: String::new(),
            arguments: Vec::new(),
            background: false,
        };
        let mut current_token: AstNode = AstNode::None;
        let mut inside_pipeline: bool = false;

        for lexeme in lexer.lexemes {
            match lexeme.as_str() {
                ";" => {
                    if inside_pipeline && let AstNode::Pipeline(pipeline_cmds) = &mut current_token {
                        pipeline_cmds.push(current_cmd.clone());
                        sequence.push(current_token.clone());
                        inside_pipeline = false;
                    } else {
                        sequence.push(AstNode::Command(current_cmd.clone()));
                    }
                    current_token = AstNode::None;
                    current_cmd = Command {
                        program: String::new(),
                        arguments: Vec::new(),
                        background: false,
                    };
                }
                "|" => {
                    if current_cmd.program.len() == 0 {
                        println!("0-shell: parse error near `|'");
                        return Vec::new();
                    }
                    inside_pipeline = true;
                    if !matches!(current_token, AstNode::Pipeline(_)) {
                        let mut pipes = Vec::new();
                        pipes.push(current_cmd.clone());
                        current_token = AstNode::Pipeline(pipes);
                    } else if let AstNode::Pipeline(pipeline_cmds) = &mut current_token {
                        pipeline_cmds.push(current_cmd.clone());
                    }
                    current_cmd = Command {
                        program: String::new(),
                        arguments: Vec::new(),
                        background: false,
                    };
                }
                "&" => {
                    if current_cmd.program.len() != 0 {
                        current_cmd.background = true;
                        if
                            inside_pipeline &&
                            let AstNode::Pipeline(pipeline_cmds) = &mut current_token
                        {
                            pipeline_cmds.push(current_cmd.clone());
                            sequence.push(current_token.clone());
                            inside_pipeline = false;
                        } else {
                            sequence.push(AstNode::Command(current_cmd.clone()));
                        }
                        current_token = AstNode::None;
                        current_cmd = Command {
                            program: String::new(),
                            arguments: Vec::new(),
                            background: false,
                        };
                    } else {
                        println!("parse error near `&'");
                        return Vec::new();
                    }
                }
                _ => {
                    if !current_cmd.program.is_empty() {
                        current_cmd.arguments.push(lexeme);
                    } else {
                        current_cmd.program = lexeme;
                    }
                }
            }
        }
        sequence
    }
}
