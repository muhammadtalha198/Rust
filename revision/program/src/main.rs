

//declare a struct 
    struct Person {
        name: String,
        age: u8,
        weight: String,
        height: String,
    }






fn main() {


    




    // Single string prin
    println!("Hello, world!");

    ////number print in a place holder
    println!("Number will be: {}", 24);

    // Multiple place holder
    println!("{} is a company {}. ", "Educative", "Software");

    // positional Argument
    println!(
        "Enhance your coding skills {0} courses. {0} course are very {1}",
        "Educative", "interavctive"
    );

    // place holder for a debug trait
    let array: [i32; 4] = [1, 2, 3, 4];
    println!("Array is: {:?}",array);
    println!("length of this array is : {}",array.len());

    let array1:&[i32] = &array;
    let array2:&[i32] = &array[0..2];
    println!("Array1 is: {:?}, length is: {}",array1,array1.len());
    println!("Array2 is: {:?}, length is: {}",array2,array2.len());

    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // define a tuple with type annotated
    let person_data : (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");


    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");

    // access value of a tuple
    println!("The value of the tuple at index 0 and index 1 are {} {}",person_data.0,person_data.1);

    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // get individual values out of tuple
    let (w ,x, y, z) = person_data;
    //print values
    println!("Name : {}", w);
    println!("Age : {}", x);
    println!("Weight : {}", y);
    println!("Height : {}", z);


    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    //print the value of tuple
    println!("Tuple - Person Data : {:?}", person_data);


    
    
}
