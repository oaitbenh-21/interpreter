# Command Module
This module provides a registry for built-in shell commands and allows for their execution. It includes implementations for several common commands such as `echo`, `cd`, `clear`, and `exit`.
### usage
just call the function ```Registry::new() ``` to get the registry with all commands registered.
then call the run method with the command name and Cmd struct.
like this:
```rust
let registry = Registry::new();
let cmd_data = Cmd {
    name: "echo".to_string(), // type: String
    args: vec!["Hello, World!".to_string()], // type: Vec<String>
    stdin: Box::new(io::stdin()), // type: Box<dyn Read>
    stdout: Box::new(io::stdout()), // type: Box<dyn Write>
    stderr: Box::new(io::stderr()), // type: Box<dyn Write>
};
registry.run(cmd_data);```

# Built-in Commands
This module contains the implementation of built-in commands for the shell. Each command is defined in its own submodule, and the `Registry` struct is used to manage and execute these commands.
The built-in commands currently implemented are:
- `echo`: Prints the provided arguments to the standard output.
- `cd`: Changes the current working directory.
- `clear`: Clears the terminal screen.
- `exit`: Exits the shell.