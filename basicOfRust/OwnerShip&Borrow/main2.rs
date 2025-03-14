
fn main(){
    let stack_f64: f64 = 123.4;
    let mut heap_f64: Box<f64> = Box::new(123.4);

    stack_procedure(stack_f64);
    println!("stack_f64: {}", stack_f64);

    heap_procedure(&mut heap_f64);
    println!("heap_f64: {}", heap_f64);
    
}

fn stack_procedure(mut param: f64){
    
    println!("stack_procedure: {}", param);
    param = 456.7;
    println!("stack_procedure: {}", param);
}

fn heap_procedure(param: &mut Box<f64>) {
    println!("heap_procedure: {}", param);
     // Dereference the Box to access the underlying f64 value and modify it
     **param = 456.7;
    
     println!("heap_procedure after change: {}", param);
    
}
