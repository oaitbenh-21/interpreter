mod tokenizer;
use tokenizer::Tokenizer;

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
