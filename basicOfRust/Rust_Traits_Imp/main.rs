
#[derive(Debug)]

mod random_info;
use random_info::RandomInfo;



struct Dougsdata{
    some_int: i32,
    some_float: f64,
    some_bool: bool,
    random_info: RandomInfo,
}

impl SomeTrait for Dougsdata{
    fn is_valid(&self) -> bool{
        true
    }
}

fn print_if_valid(data: &dyn SomeTrait){
    if data.is_valid(){
        println!("Data is valid");
    }
}

impl RandomInfo{
    pub fn is_larger(&self, compare_to: i32) -> bool{
        self.some_int > compare_to
    }
}

impl Default for Dougsdata{
    fn default() -> Self{
        Self{
            some_int: 0,
            some_float: 0.0,
            some_bool: false,
            random_info: RandomInfo::default(),
        }
    }
}

fn main() {

    let mut  my_random_info = RandomInfo{
        call_counter: 0,
        some_int: 1,
        some_float: 2.0,
        some_bool: true,
    };

    let is_this_smaller = my_random_info.is_smaller(2);
    let is_this_larger = my_random_info.is_larger(2);
    let is_valid = my_random_info.is_valid();
    let call_counter = my_random_info.get_the_call_counter();

    let mut my_struct = Dougsdata{
        some_int: 1,
        some_float: 2.0,
        some_bool: true,
        // random_info: my_random_info,
        random_info: RandomInfo::new(0,1, 2.0, true),
    };

    // let dougsdata_struct = Dougsdata :: default();
    let my_struct_default = Dougsdata::default();

    my_struct.some_int = 2;

    print_if_valid(&my_random_info);
    print_if_valid(&my_struct);

    println!("my_struct_default: {:?}", my_struct_default);

   
   
    my_struct.random_info.print_info();
    println!("{}", my_struct.some_int);
    my_struct.random_info.print_info();
    println!("{}", is_this_smaller);
}
