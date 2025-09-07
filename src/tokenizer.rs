#[derive(PartialEq, Debug)]
enum State {
    Quote(char),
    Text,
    Symbol(char),
    Empty,
}

pub struct Tokenizer {
    elements: Vec<String>,
    tokens: Vec<Token>,
}

enum TokenState {
    Loop,
    Func,
    Null,
    Scope,
    Command,
    Parentheses,
}

enum Token {
    Loop(Loop),
    Command(Cmd),
    Parentheses(String),
    Func(Function),
    Scope(Vec<Token>),
}

pub struct Loop<T> {
    head: (String, (String, T), Box<Token>),
    body: Vec<Tokens>,
}

pub struct Function {
    name: String,
    arg: Vec<String>,
    body: Vec<Token>,
}

impl Tokenizer {
    pub fn from_input(source: &str) -> Tokenizer {
        let mut result = Tokenizer { elements: Vec::new(), tokens: Vec::new() };
        let mut mode: State = State::Empty;
        let characters: Vec<char> = source.chars().collect();
        let mut buffer: String = String::new();
        let total = characters.len();
        let mut pos: usize = 0;

        while pos < total {
            match characters[pos] {
                '"' | '\'' => {
                    if matches!(mode, State::Quote(q) if q == characters[pos]) {
                        result.elements.push(buffer.clone());
                        mode = State::Empty;
                        buffer.clear();
                    } else if matches!(mode, State::Quote(_)) {
                        buffer.push(characters[pos]);
                    } else {
                        mode = State::Quote(characters[pos]);
                    }
                }
                '|' | '&' | '>' | '<' | ';' | '(' | ')' | '{' | '}' => {
                    if
                        matches!(mode, State::Quote(_)) ||
                        matches!(mode, State::Symbol(sym) if sym == characters[pos] || (characters[pos] == '}' && sym == '{') || (characters[pos] == ')' && sym == '('))
                    {
                        buffer.push(characters[pos]);
                    } else if mode != State::Empty {
                        result.elements.push(buffer.clone());
                        buffer.clear();
                    }

                    if
                        mode == State::Text ||
                        mode == State::Empty ||
                        matches!(mode, State::Symbol(sym) if sym != characters[pos] && !(characters[pos] == '}' && sym == '{') && !(characters[pos] == ')' && sym == '('))
                    {
                        mode = State::Symbol(characters[pos]);
                        buffer.push(characters[pos]);
                    }
                }
                ' ' if !buffer.is_empty() && !matches!(mode, State::Quote(_)) => {
                    result.elements.push(buffer.clone());
                    buffer.clear();
                }
                _ => {
                    if characters[pos] != ' ' || matches!(mode, State::Quote(_)) {
                        if matches!(mode, State::Symbol(_)) && !buffer.is_empty() {
                            result.elements.push(buffer.clone());
                            buffer.clear();
                        }
                        if !matches!(mode, State::Quote(_)) {
                            mode = State::Text;
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

        result.elements = result.elements
            .clone()
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();

        println!("{:?}", result.elements);
        result
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut token_state = TokenState::Null;
        let mut currly_brackets_depth = 0;
        let mut parentheses_depth = 0;
        let mut current_token: Token;
        let mut word = String::new();
        let mut tokens = Vec::new();

        for str_token in &self.elements {
            match str_token.as_str() {
                "(" | "((" => {
                    if currly_brackets_depth == 0 {
                        parentheses_depth += 1;
                        if parentheses_depth == 1 {
                            token_state = TokenState::Parentheses;
                        } else {
                            word.push_str(str_token);
                        }
                    }
                }

                "{" => {
                    if parentheses_depth == 0 {
                        currly_brackets_depth += 1;
                        if currly_brackets_depth == 1 {
                            token_state = TokenState::Scope;
                        } else {
                            word.push_str(str_token);
                        }
                    }
                }
                ")" | "))" => {
                    if currly_brackets_depth == 0 && parentheses_depth > 0 {
                        parentheses_depth -= 1;
                        if parentheses_depth == 0 {
                            token_state = TokenState::Null;
                            // Finish and store the token
                        } else {
                            word.push_str(str_token);
                        }
                    }
                }
                "}" => {
                    if parentheses_depth == 0 && currly_brackets_depth > 0 {
                        currly_brackets_depth -= 1;
                        if currly_brackets_depth == 0 {
                            token_state = TokenState::Null;
                            // Finish and store the token
                        } else {
                            word.push_str(str_token);
                        }
                    }
                }

                "func" => {
                    token_state = TokenState::Func;
                    tokens.push(
                        Token::Func(Function {
                            name: String::new(),
                            arg: vec![],
                            body: vec![],
                        })
                    );
                }

                "for" => {
                    token_state = TokenState::Loop;
                    current_token = Token::Loop(Loop {
                        head: ("".into(), ("".into(), 0), Token::Cmd(_)),
                        body: vec![],
                    });
                }

                _ => {
                    word.push_str(str_token);
                }
            }
        }

        tokens
    }
}
