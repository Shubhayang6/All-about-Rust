
fn main() {

    let /*mut*/ x = 5; // mut is added as comment just to show that keyword makes the variable "x" mutable.
    println!("The value of x is: {x}");

    x = 6; // This will throw an error as "x" is immutable {https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html}
    println!("The value of x is {x}");
}
