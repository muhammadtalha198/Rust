
fn main(){
    let var_a = String::from ("Howdy");
    // let var_b = var_a;

    println!("var_a is {}", var_a);
    // println!("var_b is {}", var_b);

    //Stack variables 

    let stack_i8: i8 = 10;
    let stack_f32: f32 = 10.0;
    let stack_bool: bool = true;
    let stack_char: char = 'a';
    let stack_array: [i32; 3] = [1, 2, 3];
    let stack_tuple: (i32, i32) = (1, 2);
    
    //Heap variables

    let heap_vector : Vec<i32> = Vec::new();
    let heap_string : String = String::new();
    let heap_i8:Box<i8> = Box::new(10);

    let stack_i8 : i8 = stack_i8;

    println!("stack_i8 is {}", stack_i8);

    let heap_i8_2 : i8 = *heap_i8;
    let heap_i8_3 : i8 = heap_i8.clone();
    println!("heap_i8_2 is {}", heap_i8_2);
    println!("heap_i8_3 is {}", heap_i8_3);
    
}
