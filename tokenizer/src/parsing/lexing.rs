#[derive(PartialEq, Debug)]
pub enum ParseState {
    Quote(char),
    Word,
    Operator(char),
    None,
}

pub struct Lexer {
    pub lexemes: Vec<String>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer { lexemes: Vec::new() };
        let mut state: ParseState = ParseState::None;
        let chars: Vec<char> = input.chars().collect();
        let mut buffer: String = String::new();
        let input_len = chars.len();
        let mut index: usize = 0;

        while index < input_len {
            match chars[index] {
                '"' | '\'' => {
                    if matches!(state, ParseState::Quote(c) if c == chars[index]) {
                        lexer.lexemes.push(buffer.clone());
                        state = ParseState::None;
                        buffer.clear();
                    } else if matches!(state, ParseState::Quote(_)) {
                        buffer.push(chars[index]);
                    } else {
                        state = ParseState::Quote(chars[index]);
                    }
                }
                '|' | ';' => {
                    if
                        matches!(state, ParseState::Quote(_)) ||
                        matches!(state, ParseState::Operator(c) if c == chars[index])
                    {
                        buffer.push(chars[index]);
                    } else if state != ParseState::None {
                        lexer.lexemes.push(buffer.clone());
                        buffer.clear();
                    }
                    if state == ParseState::Word || state == ParseState::None {
                        state = ParseState::Operator(chars[index]);
                        buffer.push(chars[index]);
                    }
                }
                ' ' if !buffer.is_empty() && !matches!(state, ParseState::Quote(_)) => {
                    lexer.lexemes.push(buffer.clone());
                    buffer.clear();
                }
                _ => {
                    if chars[index] != ' ' || matches!(state, ParseState::Quote(_)) {
                        if matches!(state, ParseState::Operator(_)) && !buffer.is_empty() {
                            lexer.lexemes.push(buffer.clone());
                            buffer.clear();
                        }
                        if !matches!(state, ParseState::Quote(_)) {
                            state = ParseState::Word;
                        }
                        buffer.push(chars[index]);
                    }
                }
            }
            index += 1;
        }

        if !buffer.is_empty() {
            lexer.lexemes.push(buffer);
        }

        lexer.lexemes = lexer.lexemes
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();

        lexer
    }
}
