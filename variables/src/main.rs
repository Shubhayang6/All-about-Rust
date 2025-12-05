fn main() {
    let mut x = 5; // mut is added as just to show that keyword makes the variable "x" mutable.
    println!("The value of x is: {x}");

    x = 6; // This will throw an error as "x" is immutable {https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html}
    println!("The value of x is {x}");

    shadowing();
}

//declration of constants -> always defined with `type`.
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn shadowing() {
    let x = 5; // actual declaration

    let x = x + 1; // value of x = 5 is shadowed and taken as a new reference

    {
        let x = x * 2; // same shadowing principle here
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");
}
