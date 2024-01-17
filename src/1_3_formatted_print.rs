/*
    All types which want to use std::fmt formatting traits require an implementation to be printable.
        Automatic implementations are only provided for types such as in the std library.
        All others must be manually implemented somehow.

    The fmt::Debug trait makes this very straightforward.
    All types can derive (automatically create) the fmt::Debug implementation.
        This is not true for fmt::Display which must be manually implemented.
 */

fn main() {
    debug();
    display();
    list();
    formatting();
}

// Derive the `fmt::Debug` implementation for `Structure`.
// `Structure` is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn debug() {
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name", "Slater", "Christian", actor="actor's");

    println!("Now {:?} will print!", Structure(3));
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};
    println!("{:#?}", peter);
}


use std::fmt;
use std::fmt::write;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure1(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure1 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn display() {
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3,3);
    println!("The big range is {big} and small is {small}", small = small_range, big = big_range);

    let point = Point2D{x:3.3, y:7.2};
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration count in `count`.
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {write!(f, ", ")?;}
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn list() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

use std::fmt::{Formatter, Display};

struct City {
    name: &'static str,
    lat: f32, // Latitude
    lon: f32, // Longitude
}

impl Display for City{
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn formatting() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        println!("{:?}", color);
    }
}