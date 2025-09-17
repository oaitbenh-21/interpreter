use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use colored::*;
use std::env;
use std::{ error::Error, fs, io::{ self, Write } };
use tokenizer::evaluate;
fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let mut rl = DefaultEditor::new()?;
    let history_path = "/tmp/.minishell_history";
    println!(
        r"
   __                     __              ___    ___      
 /'__`\                  /\ \            /\_ \  /\_ \     
/\ \/\ \             ____\ \ \___      __\//\ \ \//\ \    
\ \ \ \ \  _______  /',__\\ \  _ `\  /'__`\\ \ \  \ \ \   
 \ \ \_\ \/\______\/\__, `\\ \ \ \ \/\  __/ \_\ \_ \_\ \_ 
  \ \____/\/______/\/\____/ \ \_\ \_\ \____\/\____\/\____\
   \/___/           \/___/   \/_/\/_/\/____/\/____/\/____/
"
    );
    println!(" Welcome to 0-shell! Type 'exit' to quit.\n");
    if rl.load_history(history_path).is_err() {
        fs::File::create(history_path).ok();
    }

    loop {
        let mut input = String::new();
        match read_input(&mut rl, &mut input) {
            Ok(0) => {
                continue;
            }
            Ok(_) => {
                if input.trim().len() == 0 {
                    continue;
                }
                evaluate(&input);
                io::stdout().flush().unwrap();
                rl.add_history_entry(input.trim_end_matches('\n'))?;
            }
            Err(e) => eprint!("{}", e),
        }
    }
}

fn read_input(rl: &mut DefaultEditor, input: &mut String) -> Result<usize, ReadlineError> {
    let mut total_len = 0;
    let mut hello_message = build_prompt();
    loop {
        let line = rl.readline(&hello_message);
        match line {
            Ok(ref l) => {
                input.push_str(l);
                input.push('\n');
                total_len += l.len() + 1;

                if quotes_even(input) {
                    break;
                } else {
                    hello_message = " > ".to_string();
                }
            }
            Err(ReadlineError::Eof) => {
                std::process::exit(0);
            }
            Err(ReadlineError::Interrupted) => {
                return Ok(0);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    let _ = input.trim();
    Ok(total_len)
}

fn quotes_even(input: &str) -> bool {
    let double_quotes = input.matches('"').count();
    let single_quotes = input.matches('\'').count();

    double_quotes % 2 == 0 && single_quotes % 2 == 0
}
pub fn build_prompt() -> String {
    let user = env::var("USER").unwrap_or("user".to_string());
    let path = env::current_dir().unwrap_or("/".into());
    format!(
        "{}{}{}:{}$ ",
        user.bright_green().bold(),
        "@".bold(),
        "0-shell".bright_green().bold(),
        path.to_str().unwrap().to_string().bright_blue()
    )
}
