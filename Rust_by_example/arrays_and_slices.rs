use std :: mem;


//Arraarray2 are created using brackets [], and their length, which is known at compile time.
// Slices are similar to arraarray2, but their length is not known at compile time. 

// this function will borrow a slivce.
fn analyzie_slice(slice: &[i32]){
    println!("first element of slice is {}",slice[0]);
    println!("length of slice is {}",slice.len());
}

fn main() {
    //fixed-size array 
    let array1: [i32;5] = [1,2,3,4,5];
    println!("array1 is {:?}",array1);
    println!("first element of array1 is {}",array1[0]);
    println!("length of array1 is {}",array1.len());

    //Arraarray2 are stack allocated
    println!("array occupies {} bytes",mem::size_of_val(&array1));      


    // All elements can be initialized to the same value
    let array2: [i32;500] = [0;500];
    println!("array array2 occupies {} bytes",mem::size_of_val(&array2));

    //Array can be automatically borrowed as a slice
    analyzie_slice(&array1);


    //Slices can point to a section of an array
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.

    println!("Borrow a section of the array as a slice");
    analyzie_slice(&array1[1..4]); //slice of elements 1,2,3


    // Example of an empty slice
    let empty_array: [i32;0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

     // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.

    for i in 0..array1.len() + 1{
        match array1.get(i){
            Some(x) => println!("array1[{}] = {}",i,x),
            None => println!("Sorry, there is no element at index {}",i),
        }

    }

     // Out of bound indexing on array with constant value causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);


}