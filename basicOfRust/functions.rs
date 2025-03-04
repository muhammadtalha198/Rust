// functions
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn human_id(name: &str, age: i32, height: i32) -> (String, i32, i32) {
    (name.to_string(), age, height)
}

const MAX_POINTS: u32 = 100_000;

fn main() {

    let sum = add(1, 2);
    println!("sum: {:?}", sum); 
    
    let human = human_id("John", 20, 170);
    println!("human: {:?}", human);
    let (human_name1,human_age1,human_height1) = human;
    println!("human_name: {:?}", human_name1);   
    println!("human_age: {:?}", human_age1);
    println!("human_height: {:?}", human_height1);

    let (human_name,human_age,human_height) = human_id("JohnCena", 210, 140);
    println!("human_name: {:?}", human_name);   
    println!("human_age: {:?}", human_age);
    println!("human_height: {:?}", human_height);
    
    println!("MAX_POINTS: {:?}", MAX_POINTS);
}

// closuress

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}   

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}   
