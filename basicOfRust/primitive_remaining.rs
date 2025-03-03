fn character_types() {
    let c: char = 'a';
    let c2: char = '🚀';

    println!("c: {}", c);
    println!("c2: {}", c2);
}


fn floating_point_types() {
    let f32: f32 = 1.0;
    let f64: f64 = 1.0;
    
    println!("f32: {}", f32);
    println!("f64: {}", f64);
}   

fn boolean_types() {
    let b: bool = true;
    let b2: bool = false;
    
    println!("b: {}", b);
    println!("b2: {}", b2);
}


fn main() {
    character_types();
    floating_point_types();
    boolean_types();
    
}
