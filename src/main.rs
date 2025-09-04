#[derive(PartialEq, Debug)]
enum ParseState {
    InQuotes(char),
    InBrackets(char),
    InWord,
    InOperation(char),
    Idle,
}

struct CommandParser {
    tokens: Vec<String>,
}

impl CommandParser {
    fn parse(input: &str) -> CommandParser {
        let mut parser = CommandParser {
            tokens: Vec::new(),
        };
        let mut state = ParseState::Idle;
        let mut bracket_depth = 0;

        let chars: Vec<char> = input.chars().collect();
        let mut current_token = String::new();
        let len = chars.len();
        let mut index = 0;

        while index < len {
            let ch = chars[index];

            match ch {
                '"' | '\'' => {
                    if matches!(state, ParseState::InQuotes(q) if q == ch) {
                        parser.tokens.push(current_token.clone());
                        state = ParseState::Idle;
                        current_token.clear();
                    } else if matches!(state, ParseState::InQuotes(_)) || matches!(state, ParseState::InBrackets(_)) {
                        current_token.push(ch);
                    } else {
                        state = ParseState::InQuotes(ch);
                    }
                }

                '{' | '}' | '(' | ')' => {
                    if (ch == '(' || ch == '{') && !matches!(state, ParseState::InQuotes(_)) && bracket_depth == 0 {
                        parser.tokens.push(current_token);
                        current_token = String::new();
                        state = ParseState::InBrackets(ch);
                        bracket_depth += 1;
                        current_token.push(ch);
                    } else if matches!(state, ParseState::InBrackets(open)
                        if (open == '{' && ch == '}') || (open == '(' && ch == ')')) && bracket_depth == 1 {
                        current_token.push(ch);
                        parser.tokens.push(current_token.clone());
                        state = ParseState::Idle;
                        current_token.clear();
                        bracket_depth -= 1;
                    } else if matches!(state, ParseState::InBrackets(_)) && (ch == '}' || ch == ')') && bracket_depth > 1 {
                        bracket_depth -= 1;
                        current_token.push(ch);
                    } else if (ch == '(' || ch == '{') && matches!(state, ParseState::InBrackets(_)) {
                        bracket_depth += 1;
                        current_token.push(ch);
                    }
                }

                '|' | '&' | '>' | '<' | ';' => {
                    if matches!(state, ParseState::InQuotes(_))
                        || matches!(state, ParseState::InBrackets(_))
                        || matches!(state, ParseState::InOperation(op) if op == ch) {
                        current_token.push(ch);
                    } else if state != ParseState::Idle {
                        parser.tokens.push(current_token.clone());
                        current_token.clear();
                    }

                    if state == ParseState::InWord || state == ParseState::Idle {
                        state = ParseState::InOperation(ch);
                        current_token.push(ch);
                    }
                }

                ' ' if !current_token.is_empty()
                    && !matches!(state, ParseState::InQuotes(_))
                    && !matches!(state, ParseState::InBrackets(_)) =>
                {
                    parser.tokens.push(current_token.clone());
                    current_token.clear();
                }

                _ => {
                    if ch != ' ' || matches!(state, ParseState::InQuotes(_)) || matches!(state, ParseState::InBrackets(_)) {
                        if matches!(state, ParseState::InOperation(_)) && !current_token.is_empty() {
                            parser.tokens.push(current_token.clone());
                            current_token.clear();
                        }
                        if !matches!(state, ParseState::InQuotes(_)) && !matches!(state, ParseState::InBrackets(_)) {
                            state = ParseState::InWord;
                        }
                        current_token.push(ch);
                    }
                }
            }

            index += 1;
        }

        if !current_token.is_empty() {
            parser.tokens.push(current_token);
        }

        parser.tokens = parser.tokens.into_iter().filter(|s| !s.is_empty()).collect();
        println!("{:?}", parser.tokens);
        parser
    }
}

fn main() {
    let commands: Vec<&'static str> = vec![
        "ls -l /home/user",
        "grep \"pattern\" file.txt",
        "cat file1.txt file2.txt > output.txt",
        "echo \"Hello, World!\" | tee log.txt",
        "mkdir new_folder && cd new_folder",
        "rm -rf old_folder &",
        "let VAR=value",
        "echo $VAR",
        "command $(subcommand)",
        "find . -name \"*.py\" -exec grep \"def \" {} \\;",
        "la|ls",
        "ls -l | grep file; la",
	"func main(dsfls) sdfs{ ls -l; cd /; read;}",
	"for ((i = 0 ; i < 100 ; i++)); do echo $i done"
    ];
    for cmd in commands {
        println!("{}", cmd);
        let _ = CommandParser::parse(cmd);
        println!();
    }
}
