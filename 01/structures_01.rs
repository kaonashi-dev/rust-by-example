// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    role: String,
}

// A unit struct
#[derive(Debug)]
struct Unit;

// A tuple struct
#[derive(Debug)]
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;

    let width = (x2 - x1).abs();
    let height = (y1 - y2).abs(); // y1 > y2

    width * height
}

fn square(bottom_left: Point, size: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: bottom_left.x,
            y: bottom_left.y + size,
        },
        bottom_right: Point {
            x: bottom_left.x + size,
            y: bottom_left.y,
        },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let user_role = String::from("admin");
    let peter = Person {
        name,
        age,
        role: user_role,
    };
    let mike = Person {
        name: String::from("Mike"),
        age: 30,
        role: String::from("user"),
    };

    // Print debug struct
    println!("Peter: {:?}", peter);
    println!("Mike: {:?}", mike);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point {
        x: 10.5,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    println!("left_edge/top_edge point: ({}, {})", left_edge, top_edge);

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: Point { x: 5.0, y: 9.0 },
    };

    // Instantiate a unit struct
    let _unit = Unit;
    println!("_unit => {:?}", _unit);

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let test_rect = Rectangle {
         top_left: Point { x: 1.0, y: 4.0 },
         bottom_right: Point { x: 3.0, y: 1.0 },
     };

     println!("Rectangle: {:?}", test_rect);
     println!(" {:?}", rect_area(test_rect));

     // Probando square
     let start_point = Point { x: 2.0, y: 3.0 };
     let my_square = square(start_point, 5.0);

     println!("\nmy_square: {:?}", my_square);
     println!(" {:?}", rect_area(my_square));
}
