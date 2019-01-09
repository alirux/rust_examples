use std::path::Path;

fn main() {
    println!("Hello, world!");

    // let defines a constant (so unmutable)
    // No type declaration, Rust infer the type from the assignment
    let test = String::from("Ciao!");
    // The exclamation mark defines a macro
    println!("The test string is: {}", test);
    println!("Function: {}", test_function());
    
    function_with_params(String::from("new text"), 234);
    function_with_params("New text 2".to_string(), 234);
    function_with_condition(0);
    function_with_condition(-30);
    function_with_condition(20);
    let function_result : i32 = loop_function();
    println!("Loop result {}", function_result);

    // Ownership
    let xo = String::from("Ownership test");
    // Transfer the ownership from xo to yo
    let yo = xo;
    // We can no longer use xo because the ownership is on yo
    // println!("xo: {}", xo); // Error!
    // We can use yo:
    println!("yo: {}", yo);

    // Move th ownership from yo to zo
    let zo = move_ownership(yo);
    println!("zo: {}", zo);
    // Ownership issue: we con no longer user yo (ownership moved to zo)
    //println!("yo: {}", yo);  // Error!

    // While loop example
    while_loop();

    // Print a unicode char:
    print_char('😻');

    // File path
    let path = Path::new("/tmp/rust_file.txt");
    println!("Path: {:?}", path);
}

fn print_char(chr: char) {
    println!("The chr is char: {}", chr);
}

fn while_loop() {
    // Adding mut to let, we define a variable (mutable)
    let mut c = 10;
    while c > 0 {
        if c == 1 {
            println!("{}", c);
        } else {
            print!("{}, ", c);
        }
        c -= 1;
    }
}

// Move the ownership to str and then to the return value of the function
fn move_ownership(str: String) -> String {
    str
}

// This function returns an unsigned integer
fn test_function() -> u32 {
    // No semicolon, so the result of the function is 5
    5
}

// Two parameters
fn function_with_params(p1: String, p2: u32) {
    println!("Result: {} number: {}", p1, p2)
}

fn function_with_condition(p: i32) {
    // If without parentesis
    if p == 0 {
        println!("p is zero {}", p);
    } else if p > 0 {
        println!("p is greater than zero {}", p);
    } else {
        println!("p is lower than zero {}", p);
    }
}

// Counts to 10, multiplies for two and returns 20
fn loop_function() -> i32 {
    let mut counter = 0;

    // Loop is an expression, so its result can be assigned to a variable
    // At the end of the loop, result will be 10*2
    let result = loop {
        counter += 1;

        if counter == 10 {
            // break, ends the loop
            // break <value>, ends the loop and return the value
            break counter * 2;
        }
    };

    // A macro usefull to test equality
    assert_eq!(result, 20);

    // The last expression evaluated becomes the result of the funcion
    result
}
