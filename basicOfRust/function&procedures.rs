


fn some_function(param_a: f64, param_b: i32) -> f64 {
    println!("I am some function");
    
    if param_a < 100.0 {
        println!("param_a is less than 100.0");
    } else {
        println!("param_a is greater than 100.0");
    }
    
    let return_var = 10.01 * param_a * param_b as f64;
    
    return return_var;
}

fn some_procedure(param_a: f64, param_b: i32) {
    println!("I am some procedure and param_a is {}, param_b is {}", param_a, param_b);
}

fn some_str_procedure(param_a: &str, param_b: &str) {
    println!("I am some str procedure and param_a is {}, param_b is {}", param_a, param_b);
}

fn some_string_procedure(param_a: String, param_b: String) {
    println!("I am some string procedure and param_a is {}, param_b is {}", param_a, param_b);
}

fn main() {
    println!("Hello, world!");

    let returned_data = some_function(2.2, 50);
    println!("returned_data: {:?}", returned_data);

    some_procedure(2.2, 50);
    some_str_procedure("Hello", "World");

    let string_slice_var: &str = "Howdy!";
    let string_var: String = String::from("Howdy2!");
    
    some_str_procedure(string_slice_var, &string_var);
    some_str_procedure(string_slice_var, string_var.as_str());

    some_string_procedure(string_slice_var.to_string(), string_var);
    
    
}
