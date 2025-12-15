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
#[derive(Debug)]
struct Point{
    x:f32,
    y:f32,
}

//A struct canbe used as a field for another struct 
#[derive(Debug)]
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

    // Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: Point { x: 5.2, y: 0.0 },
    };

    fn rect_area (rect:Rectangle) -> f32 {
      
      let Rectangle{top_left: Point{x:x1,y:y1},bottom_right: Point{x:x2,y:y2}} = rect;


        // Calculate width and height
        let width = (x2 - x1).abs();
        let height = (y2 - y1).abs();

        // Return area
        width * height

    }

    fn add_point(point:Point) -> f32 {
        let Point{x:x1, y:y1} = point;
        x1 + y1
    }

    println!("Area of rectangle is {}",rect_area(_rectangle));
    println!("Sum of x and y coordinates of point is {}",add_point(point));

    fn square( top_left_point:Point , side_length:f32) -> Rectangle{

        // Calculate the coordinates of the bottom_right corner.
        // Width is side_length (x + side_length)
        // Height is side_length (y - side_length
        let Point{x:x1,y:y1} = top_left_point;
        let bottom_right_point = Point{x: x1 + side_length , y: y1 - side_length};  
        Rectangle{ top_left: top_left_point , bottom_right: bottom_right_point}     

    }

    let my_square = square(Point{x:1.0,y:4.0},2.0);
    println!("My square is {:?}",my_square);
    // println!("Area of my square is {}",rect_area(my_square));

}
