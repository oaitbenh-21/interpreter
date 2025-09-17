# 🐚 Custom Shell in Rust

A minimalist Unix-like shell written in **Rust**, supporting a set of builtins and common commands.  
⚠️ Unlike traditional shells, this shell **does not run any system binaries** — it only executes the ones **built and provided inside this project**.

---

## 🚀 Features

### Builtin Commands
- `cd` – Change directory
- `pwd` – Print working directory
- `exit` – Exit the shell
- `clear` – Clear the terminal screen

### External Commands
- `echo` – Print arguments to standard output
- `ls` – List directory contents  
  - Supports:  
    - `-l` → long listing format  
    - `-a` → include hidden files  
    - `-F` → classify entries with `/`, `*`, `@`  
- `cat` – Concatenate and display file contents
- `cp` – Copy files
- `rm` – Remove files and directories (`-r` for recursive)
- `mv` – Move/rename files
- `mkdir` – Create directories

### Supported
- Pipelines: `cmd1 | cmd2`
- Command chaining: `cmd1 && cmd2`


## 📂 Project Structure

```
├── bin/                # Installed command binaries (after build)
├── commands/           # Implementation of external commands
│   ├── src/bin/        # Each subcommand (cat, ls, etc.)
│   └── builtin/        # Builtin commands (cd, pwd, exit, clear)
├── executer/           # Executes parsed commands
├── tokenizer/          # Tokenizer & parser for command-line input
├── shell/              # Main shell entry point
├── Makefile            # Build, install, and run automation
├── Cargo.toml          # Workspace configuration
└── README.md           # You are here!
```

---

## ⚙️ Build & Run

### 1. Build Everything
```sh
make
```

### 2. Run the Shell
```sh
make run
```

### 3. Environment Setup
After `make`, a `.env` file is generated with the shell binary path:
```sh
DIR=/absolute/path/to/project/bin/
```

You can source it if needed:
```sh
source .env
```

---

## 🖥️ Usage Examples

```sh
$ pwd
/home/user/projects/shell

$ ls -laF
drwxr-xr-x   5 user user  160 Sep 15 12:00 ./
drwxr-xr-x  18 user user  576 Sep 15 12:00 ../
-rwxr-xr-x   1 user user 8192 Sep 15 12:00 shell*

$ echo "Hello, Rust!"
Hello, Rust!

$ mkdir test && cd test
$ echo "demo" > file.txt
$ cat file.txt
demo

$ cp file.txt copy.txt
$ ls
file.txt  copy.txt

$ rm -r test

$ echo "hello" | cat
hello
```

---

## ✅ Roadmap

- [ ] Add redirection (`>`, `<`)
- [ ] Implement job control (`&`, `fg`, `bg`)
- [ ] Add more commands (`head`, `tail`, `grep`, etc.)
- [ ] Improve error handling & messages

---

## 📜 License

This project is licensed under the MIT License.  
Feel free to use, modify, and share it!
