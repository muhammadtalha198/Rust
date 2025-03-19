



mod random_info;
use random_info::random_info;
// use random_info::*;

// structs
struct DoughData {
    some_strings: String,
    some_int: i32,
    fome_float: f64,
    some_bool: bool,
    some_str: &str,
    some_char: char,
    some_tuple: (u8, u8, u8),
    random: random_info,
}

impl SomeTrait for DoughData {
    fn is_smaller(&self, compare_to: i64) -> bool {
        self.some_int < compare_to
    }

    fn is_larger(&self, compare_to: i64) -> bool {
        self.some_int > compare_to
    }

    fn is_valid(&self) -> bool {
        self.some_int > 0
    }
}   

fn print_if_is_valid(check_valid: &dyn SomeTrait){
    if check_valid.is_valid(){
        println!("is valid");
    }
}

impl random_info{
    pub fn is_larger(&self, compare_to: i64) -> bool {
        self.some_int > compare_to
    }
}

fn main(){


    let mut some_random_info = random_info {
        call_count: 0,
        some_strings: String::from("Talha"),
        some_int: 1,
        some_float: 1.1,
        some_bool: true,
    };

    let is_smaller = some_random_info.is_smaller(2);
    let is_larger = some_random_info.is_larger(2);
    let is_valid = some_random_info.is_valid();

    println!("some_random_info: {:?}", some_random_info);

    let mut dough_data = DoughData {
        some_strings: String::from("Hello"),
        some_int: 1,
        fome_float: 1.1,
        some_bool: true,
        some_str: "Hello",
        some_char: 'c', 
        some_tuple: (255, 0, 0),
        random: random_info{
            some_strings: String::from("Talha"),
            some_int: 1,
            some_float: 1.1,
            some_bool: true,
        },
    };

    println!("dough_data: {:?}", dough_data);

    dough_data.some_strings = String::from("World");

    println!("dough_data: {:?}", dough_data);

    let some_other_data = DoughData {
        some_strings: String::from("Muhammad"),
        ..dough_data
    };

    println!("some_other_data: {:?}", some_other_data);

}

    
