#[derive(PartialEq, Debug)]
enum TokenState {
    Quote(char),
    Text,
    Symbol(char),
    Empty,
}

struct Tokenizer {
    elements: Vec<String>,
}

impl Tokenizer {
    fn from_input(source: &str) -> Tokenizer {
        let mut result = Tokenizer { elements: Vec::new() };
        let mut mode: TokenState = TokenState::Empty;
        let characters: Vec<char> = source.chars().collect();
        let mut buffer: String = String::new();
        let total = characters.len();
        let mut pos: usize = 0;

        while pos < total {
            match characters[pos] {
                '"' | '\'' => {
                    if matches!(mode, TokenState::Quote(q) if q == characters[pos]) {
                        result.elements.push(buffer.clone());
                        mode = TokenState::Empty;
                        buffer.clear();
                    } else if matches!(mode, TokenState::Quote(_)) {
                        buffer.push(characters[pos]);
                    } else {
                        mode = TokenState::Quote(characters[pos]);
                    }
                }
                '|' | '&' | '>' | '<' | ';' | '(' | ')' | '{' | '}' => {
                    if matches!(mode, TokenState::Quote(_)) ||
                        matches!(mode, TokenState::Symbol(sym) if sym == characters[pos] || (characters[pos] == '}' && sym == '{') || (characters[pos] == ')' && sym == '('))
                    {
                        buffer.push(characters[pos]);
                    } else if mode != TokenState::Empty {
                        result.elements.push(buffer.clone());
                        buffer.clear();
                    }

                    if mode == TokenState::Text || mode == TokenState::Empty
                        || matches!(mode, TokenState::Symbol(sym) if sym != characters[pos] && !(characters[pos] == '}' && sym == '{') && !(characters[pos] == ')' && sym == '(')) {
                        mode = TokenState::Symbol(characters[pos]);
                        buffer.push(characters[pos]);
                    }
                }
                ' ' if !buffer.is_empty() && !matches!(mode, TokenState::Quote(_)) => {
                    result.elements.push(buffer.clone());
                    buffer.clear();
                }
                _ => {
                    if characters[pos] != ' ' || matches!(mode, TokenState::Quote(_)) {
                        if matches!(mode, TokenState::Symbol(_)) && !buffer.is_empty() {
                            result.elements.push(buffer.clone());
                            buffer.clear();
                        }
                        if !matches!(mode, TokenState::Quote(_)) {
                            mode = TokenState::Text;
                        }
                        buffer.push(characters[pos]);
                    }
                }
            }
            pos += 1;
        }

        if !buffer.is_empty() {
            result.elements.push(buffer.clone());
        }

        result.elements = result
            .elements
            .clone()
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();

        println!("{:?}", result.elements);
        result
    }
}

fn main() {
    let input_lines: Vec<&'static str> = vec![
        "ls -l /home/user;",
        "grep \"pattern\" file.txt;",
        "cat file1.txt file2.txt > output.txt;",
        "echo \"Hello, World!\" | tee log.txt;",
        "mkdir new_folder && cd new_folder;",
        "rm -rf old_folder &",
        "let VAR=value;",
        "echo $VAR;",
        "command $(subcommand);",
        "find . -name \"*.py\" -exec grep \"def \" {} \\;",
        "la|ls",
        "ls -l | grep file; la",
        "func main(dsfls) sdfs{ ls -l; cd /; read;};",
        "for ((i = 0 ; i < 100 ; i++)); do echo $i done;"
    ];

    for line in input_lines {
        println!("{}", line);
        let _ = Tokenizer::from_input(line);
        println!();
    }
}
