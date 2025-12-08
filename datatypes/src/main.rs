fn floating_type_scalar() {
    let x = 2.0; //f64

    let y: f32 = 3.0; //f32

    println!("\nThe default floating value(f64) is x:{x} and y:{y} is f32");
}

fn numeric_ops() {

    //addition
    let sum = 5 + 10;

    // subtraction
    let diff = 95.5 - 4.3;

    // multiplication
    let prod = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("\n The numeric operations:\n sum = {sum}\n diff = {diff}\n product = {prod}\n quotient = {quotient}\n truncated = {truncated}\n remainder = {remainder}");
}

fn bool_type_scalar() {
    let t = true;

    let f: bool = false; //explicit type annotation

    println!("\n The value of true is {t}\n The value of false is {f}")
}

fn char_type_scalar() {
    let c = 'z';
    let z: char = 'Z'; // explicit declaration
    let heart_eyed_cat = '😻';

    println!("\n The first char is: {c} \n The second char is: {z} \n Then comes the emoji: {heart_eyed_cat}")
}



fn main() {

    floating_type_scalar();

    numeric_ops();

    bool_type_scalar();
    
    
    char_type_scalar();
    
}
