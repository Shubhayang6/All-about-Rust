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

### Scalar datatypes in rust:
> int, float, Bool, Char

#### INT Type:

| Length                 | Signed | Unsigned |
|------------------------|--------|----------|
| 8-bit                  | i8     | u8       |
| 16-bit                 | i16    | u16      |
| 32-bit                 | i32    | u32      |
| 64-bit                 | i64    | u64      |
| 128-bit                | i128   | u128     |
| architecture dependent | isize  | usize    |

> - Signed variant can store numbers from *−2^(n − 1) to 2^(n − 1) − 1* inclusive, where n is the number of bits the variant uses.
> Example: **i8** can store numbers *−(2^7) to 2^(7 − 1) => -128 to 127*
> - Unsigned variant can store numbers from *0 to (2^n) - 1*
> Example: **u8** can store numbers from *0 to (2^8) - 1 => 0 - 255*
> - **isize** and **usize** depends on the architecture of the computer the program is running on => 64 bits if 64-bit arch or 32 bits if 32-bit arch.
> - **INT types in rust defaults to i32**

#### FLOAT Type:

> - Floating point defaults to f64

#### BOOLEAN Type:

> - True and False
> - Booleans are **one byte** in size.
> - Majorly used in conditionals

#### CHAR Type:
> - Always to be denoted by **single quotes** otherwise the error might be:
`expected char, found &str`