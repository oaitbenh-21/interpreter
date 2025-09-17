# Executer Library

This library provides a simple abstraction for executing commands.  
It supports two kinds of commands:

1. **Built-in commands**: stored in a `Registry` and executed directly.
2. **External commands**: binaries found in a directory passed to `execute`.

The library exposes a `Cmd` type for representing a command invocation, and an `execute` function that runs the command.

---

## Exposed Types

### `Cmd`
Represents a command with:

- A command name (`cmd`)
- A list of arguments (`args`)
- Streams for stdin, stdout, stderr

Example:

```rust
use commands::Cmd;
use std::io;

let cmd = Cmd::new(
    "ls".to_string(),
    vec!["-l".to_string()],
    Box::new(io::stdin()),
    Box::new(io::stdout()),
    Box::new(io::stderr()),
);
````

---

## The `execute` Function

```rust
use std::error::Error;
use executer::execute;
use commands::Cmd;

/// Executes a command:
/// 1. Checks if the command is registered in the `Registry` (built-in).
/// 2. If not found, searches a directory (`env_dir`) for a binary.
/// 3. If found, spawns and runs the process.
/// 4. If not found, returns an error.
///
/// Returns:
/// - `Ok(())` if successful
/// - `Err` if the command fails, is not found, or the directory does not exist
```

Signature:

```rust
pub fn execute(cmd: Cmd, env_dir: &str) -> Result<(), Box<dyn Error>>
```

---

## Example Usage

```rust
use std::io;
use executer::*;
use commands::Cmd;

fn main() {
    // Create a new command
    let cmd = Cmd::new(
        "ls".to_string(),
        vec![],                       // empty args
        Box::new(io::stdin()),        // stdin
        Box::new(io::stdout()),       // stdout
        Box::new(io::stderr()),       // stderr
    );

    // Try executing it against a given directory
    match execute(cmd, "/usr/bin") {
        Ok(()) => println!("Command executed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

---

## Notes

* Built-in commands must be registered in `Registry`.
* External commands are searched in the directory you provide (`env_dir`).
* Errors are returned as `Result::Err` instead of just being printed, so you can handle them however you want.
