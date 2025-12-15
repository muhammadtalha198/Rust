/*

Tuple structs, which are, basically, named tuples.
The classic C structs
Unit structs, which are field-less, are useful for generics.

*/

//An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}


//A unit Struct 
struct Unit;

//A tuple Struct
struct Pair(i32,f32);


// Astruct with two fields
struct Point{
    x:f32,
    y:f32,
}

//A struct canbe used as a field for another struct 
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}   

fn main(){
    // Create struct with field innit shorthand 
    let name = String::from("Muhammad");
    let age = 27;

    let muhammad = Person {name , age};

    // Print debug struct
    println!("{:?}",muhammad);

    // Instantiate a `Point`
    let point : Point = Point {x:4.3, y:4.3};
    let another_point : Point = Point {x:0.0, y:0.0};

    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one

    let bottom_right = Point {x:5.2, ..another_point};

     // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    println!("pair contains {} and {}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("pair contains {} and {}", integer, decimal);

}
