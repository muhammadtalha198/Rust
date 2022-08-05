//Variables in rust

fn main() {
    let name = "Muhammad Talha";
    println!("{}", name);

    //  you canot change the variable value unless its mutable
    // name = "Muhammad Asjad"; error

    let mut firstName = "Muhammad";
    println!("{}", firstName);

    firstName = "Ali";          //Now it could be change because it is a mutuable variable.
    println!("{}", firstName);
}
