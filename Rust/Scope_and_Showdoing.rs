fn main() {
    
    let MyAge = 17;  //the scope of MyAge is this whole main function!
    println!("My age is {}", MyAge);

    //Redeclaration of a variable in the same scope is called Shadowing

    let MyAge = 19;
    println!("My age is {}", MyAge);
}
