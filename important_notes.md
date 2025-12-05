# Important things to note in RUST

##### 1. Rust shall be compiled first like C/C++ so that the .exe binary generated can be ran even if rust isn't installed.
> $ rustc <file.rs>
##### 2. Cargo new command takes the name of the project and build for you the TOML file, just run the below command in the root directory:
> $ cargo new <name_of_project>
##### 3. In Rust, variables are immutable by default.
> let apples = 5; // immutable
> let mut bananas = 5; // mutable
##### 4. '&' is used for accesing the reference of the variable so that multiple parts of code can access one data without copying the data to memory multiple times. 
##### 5. The read_line has a return value called is 'variant' which is of type Enum with two states: 
> "Ok" and "Err"
> There shall be '.expect' which is error handling for the function.
##### 6. You can use any dependencies by adding into cargo.toml file and then running build command.
> $ cargo build
##### 7. From [variables/src/main.rs](./variables/src/main.rs) -> Understand the difference between shadowing and mutability.
> - Shadowing: Variables can be assigned different types.
> - Mutability: Doesn't allow you to change the type of the variable.
> - Reference: [Shadowing and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)