 
 pub trait SomeTrait {
    fn is_smaller(&self, compare_to: i64) -> bool;
    fn is_larger(&self, compare_to: i64) -> bool;
    fn is_valid(&self) -> bool;
 }
 
 
 pub struct random_info {
    pub call_count: i64,
    pub some_strings: String,
    pub some_int: i32,
    pub some_float: f64,
    pub some_bool: bool,
}

impl SomeTrait for random_info {
    fn is_smaller(&self, compare_to: i64) -> bool {
        self.some_int < compare_to
    }

    fn is_larger(&self, compare_to: i64) -> bool {
        self.some_int > compare_to
    }

    fn is_valid(&self) -> bool {
        self.some_int > 0
}

impl random_info {
   
    pub fn new(some_strings: String, some_int: i32, some_float: f64, some_bool: bool) -> Self {
        Self { 
            call_count: 0,
            some_strings: "Muhammad".to_string(), 
            some_int: 20,
            some_float: 1.1,
            some_bool: true
        }
    }

    pub fn is_smaller(&mut self, compare_to: i64) -> bool {
        self.call_count += 1;
        self.some_int < compare_to;
    }

    pub fn print_info(&self) {
        println!("some_strings: {}", self.some_strings);
        println!("some_int: {}", self.some_int);
        println!("some_float: {}", self.some_float);
        println!("some_bool: {}", self.some_bool);
    }
}


