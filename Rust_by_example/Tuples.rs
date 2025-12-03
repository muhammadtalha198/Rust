use std::fmt;

//tuples can be used as function argument 
fn reverse(pair:(i32,bool)) ->(bool, i32){
    let (integer,boolean)=pair;
    (boolean,integer)   
}


#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);


impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"( {}, {}, {}, {} )",self.0,self.1,self.2,self.3)
    }
}

// Function to transpose a 2x2 matrix
fn transpose(matrix:Matrix) -> Matrix{
    matrix
}


//fist add values in Matrix then return 
fn add_values_in_matrix(v1:f32,v2:f32,v3:f32,v4:f32) ->Matrix{
    let matrix = Matrix(v1,v2,v3,v4);
    matrix
}


fn main () {


    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    let display_matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", display_matrix);

    let tranpose_matrix = transpose(display_matrix);
    println!("transposed Matrix: {}", tranpose_matrix);

    let print_added_matrix = add_values_in_matrix(3.3,4.4,5.5,6.6);
    println!("Added Matrix: {}", print_added_matrix);



    //A tuple with bunch of different types 
     let logn_tuple = (1u8,2u16,3u32,4u64,-1i8,-2i16,-3i32,-4i64,0.1f32,0.2f64,'a',true);
    println!("logn_tuple is {:?}",logn_tuple);

    println!("first element of logn_tuple is {}",logn_tuple.0);
    println!("second element of logn_tuple is {}",logn_tuple.1);

    //Tuple can be tuple members
    let tuple_of_tuples = ((1u8,2u16,3u32),(4u64,-1i8,-2i16), -2i16);
    println!("tuple_of_tuples is {:?}",tuple_of_tuples);
    println!("first element of tuple_of_tuples is {:?}",tuple_of_tuples.0);
    println!("second element of tuple_of_tuples is {:?}",tuple_of_tuples.1);
    println!("third element of tuple_of_tuples is {:?}",tuple_of_tuples.2); 

     // But long Tuples (more than 12 elements) cannot be printed.
    let _too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple); // Erorr
    


    let pair = (1,true);
    println!("pair is {:?}",pair);
    //  println!("pair is {}",pair);  // Error because {} cannot print tuple directly
    println!("pair is {} {}",pair.0,pair.1);
    println!("reversed pair is {:?}",reverse(pair));

    let (boolean,integer)=reverse((12,true));
    println!("reversed pair is {:?} {}",boolean,integer);
    println!("reversed pair is {} {}",boolean,integer);
    println!("reversed pair is {:?} {:?}",boolean,integer);


}