'''
__BASICS OF RUST PROGRAM__
- fun is used to create a function
- main is the entry point of the program
- println! is used to print the output to the console
- The ! indicates that println! is a macro, not a function

**a macro in rust is a way of writing code that writes other code

'''

fn main(){
    println!("Hello, World!")
}


'''
__HOW TO RUN A RUST PROGRAM__
1. Open the terminal
2. Navigate to the directory where the file is located
3. Run the command "rustc file.rs" to compile the file
4. Run the command "./file" to execute the program
'''

'''
__CARGO__
- Cargo is the build system and package manager for Rust
- It is used to create, build, and manage Rust projects

__HOW TO CREATE A NEW PROJECT__
1. Open the terminal
2. Run the command "cargo new project_name" to create a new project
3. Navigate to the project directory
4. Run the command "cargo build" to build the project
5. Run the command "cargo run" to execute the project

__STRUCTURE OF A CARGO PROJECT__
- Cargo.toml: configuration file that contains information about the project
- src/main.rs: the main file of the project
- target: directory where the compiled code is stored
- Cargo.lock: file that contains information about the dependencies of the project
- .gitignore: file that contains information about the files and directories that should be ignored by git

__CARGO COMMANDS__
- cargo new project_name: create a new project
- cargo build: build the project
- cargo run: execute the project
- cargo test: run the tests in the project
- cargo doc: generate the documentation for the project
- cargo publish: publish the project to crates.io
- cargo install: install a binary from crates.io
- cargo update: update the dependencies of the project
- cargo clean: clean the target directory of the project
- cargo check: check the syntax of the project without building it
'''
