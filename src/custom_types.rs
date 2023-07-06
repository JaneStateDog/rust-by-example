#![allow(dead_code)] // Allow unused code

use crate::List::*;

// Custom types are poggies!!
// Rust custom types are formed mainly through two keywords
// struct and enum
// Constants can also be created via the const and static keywords

// There are three types of structs that can be
// created using the struct keyword
/*
    - Tuple structs, which are, basically, named tuples
    - C structs
    - Unit structs, which are field-less, are useful for generics
*/

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Struct can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where
    // the top left and bottom right are
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)] // Allow us to get debug stuff with prints using this struct
struct Person {
    name: String,
    age: u8,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rectangle;

    let width: f32 = x2 - x1;
    let height: f32 = y2 - y1;

    width * height // This method of returning I believe is called a "tail"
    // I kind of despise it. I can also just do "return" but yeahhh...
}

fn square(top_left: Point, width: f32, height: f32) -> Rectangle {
    let Point { x, y } = top_left;

    Rectangle {
        top_left,
        bottom_right: Point { x: x + width, y: y + height },
    }
}

// Create an enum to classify a web event
// Note how both names and type information together specify the variant:
// "PageLoad != PageUnload" and "KeyPress(char) != Paste(String)"
// Each is different and independant
enum WebEvent {
    // An enum variant may either be unit-like,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures
    Click { x: i64, y: i64 },
}

// A function which takes a WebEvent enum as an argumetn and returns nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        // Destructure c from inside the enum variant
        WebEvent::KeyPress(c) => println!("Pressed \"{}\"", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\"", s),
        // Destructure Click into x and y
        WebEvent::Click { x, y } => { // Any one of these could open up brackets like this
            // and run multiple lines of code. I'm only doing it here as an example
            println!("Clicked at x={}, y={}", x, y)
        },
    }
}

// You can use a type alias to refer to each enum variant via its alias
// This can be useful if the enum's name is too long or generic
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// Create a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// enum can be used as C-like enums
// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// This is a test use case
// A common way to implement a linked-list is with enums
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // Nil has type List
        Nil
    }

    // Consume a list and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // Cons also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // self has to be matched because the behavior of this method
        // depends on the variant of self
        // self has type &List and *self has type List
        // Matching on a
        // concrete type T is preferred over a match on a reference &T
        // After Rust 2018 you can use self here and tail (with no ref) below as well
        match *self {
            // Can't take ownership of the tail because self is borrowed
            // Instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // format! is similar to print! but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Nil => format!("Nil"),
        }
    }
}

// Rust has two types of constants which can be declared in any scope, including global
// Both require explicit type annotation
// const: An unchangable value (the common case)
// static: A possible mutable variable with 'static lifetime. The static lifetime
// is inferred and does nto have to be specified. Accessing or modifying a mutable static
// variable is unsafe

// Globals are declared outside of all other scopes
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constnat in some function
    n > THRESHOLD
}

fn main() {

    // Custom type shizz
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);
    
    // Instantiate a Point
    let point = Point { x: 10.3, y:0.4 };
    println!("Coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point { x:5.2, ..point };
    // bottom_right.y will be the same as point.y because we used that field from point
    println!("Second coordinate: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a "let" binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle: Rectangle = Rectangle {
        // Struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right, // I wanted to do "bottom_right: bottom_right" here but the
        // rust extension wants to use this shorthand so *shrugs*
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    println!("This pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("This pair contains {:?} and {:?}", integer, decimal);

    // Print our stuff
    let rectangle2 = square(Point { x: 0.0, y: 0.0 }, 10.0, 10.0);
    println!("The area of our rectangle is {}^2", rect_area(rectangle2));

    // Enum stuff
    let pressed = WebEvent::KeyPress('x');
    // to_owned() creates an owned String from a string slice
    let pasted = WebEvent::Paste("my test".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // Type alias enum shizz
    let _thingy = Operations::Add;
    // The most common place you'll see this is in impl blocks using the Self alias

    // Explicitly "use" each name so they are available without manual scoping
    use crate::Status::{Poor, Rich};
    // Automatically "use" each name inside Work
    use crate::Work::*;

    // Equivalent to Status::Poor
    let status = Poor;
    // Equivalent to Work::Civilian
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit "use" above
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    // enums can be cast as integers
    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}", Number::One as i32);

    println!("Roses are #{:06x}", Color::Red as i32);
    println!("Violets are #{:06x}", Color::Blue as i32);

    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("Linked list has length: {}", list.len());
    println!("{}", list.stringify());

    // Const stuff
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" }); 
    // Woah you can do an if statement like that in there???

    // Error! Cannot modify a const
    // THRESHOLD = 5;

}