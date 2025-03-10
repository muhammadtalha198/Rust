#[derive(Debug)]
pub trait SomeTrait{
    fn is_valid(&self) -> bool;
    fn get_the_call_counter(&self) -> i64; 
}

pub struct RandomInfo{

    pub call_counter: i64,
    pub some_int: i32,
    pub some_float: f64,
    pub some_bool: bool,
}

impl SomeTrait for RandomInfo{
    fn is_valid(&self) -> bool{
        self.some_int > 0
    }

    fn get_the_call_counter(&self) -> i64{
        self.call_counter
    }
}

impl RandomInfo{

    //constructor function in rust
    pub fn new(call_counter: i64, some_int: i32, some_float: f64, some_bool: bool) -> Self{
        Self{call_counter, some_int, some_float, some_bool}
    }

    pub fn print_info(&self){
        println!("some_int: {}", self.some_int);
        println!("some_float: {}", self.some_float);
        println!("some_bool: {}", self.some_bool);
    }

    pub fn is_smaller(&mut self, compare_to: i32) -> bool{
        self.call_counter += 1;
        self.some_int < compare_to
    }

    pub fn is_valid(&self) -> bool{
        self.some_int > 0
    }

    pub fn is_equal(&self, compare_to: bool) -> bool{
        self.some_bool == compare_to
    }
}

