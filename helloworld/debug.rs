// debug.rs - Demos the fmt::Debug trait

use std::fmt;

// This sctructure cannot be printed either with `fmt::Display` or with `fmt::Debug`
struct UnPrintable(i32);

// the `derive` attribute automatically creates the implemntation required to make this `struct`
// printable with `fmt::Debug`
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

struct Structure(i32);

// In order to use the `{}` marker, the trait `fmt::Display` must be implemented manually for the
// type.
impl fmt::Display for Structure {
    // This trait requests `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output stream: `f`. Returns the
        // `fmt::Result` which indicates whether the operation succeeded or failed. Note that
        // `write!` uses syntax which is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

// struct with positional data
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // customeize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} +{}i", self.real, self.imag)
    }
}


fn main() {
    // Printing with `{:?}` is similar to `{}`
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");

    println!("Now {:?} will print!", DebugPrintable(3));
    // The problem with `derive` is there i no control over how the results look. What if I want
    // this to just show a `7`?
    println!("Now {:?} will print!", Deep(DebugPrintable(7)));


    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};
    println!("{:#?}", peter);


    let minmax = MinMax(0, 14);
    println!(" ");
    println!("Compare structures");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    println!(" ");
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}", big = big_range, small = small_range);

    println!(" ");
    let point = Point2D {x: 3.3, y: 7.2};
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    // println!("What does Point2D look like in binary: {:b}?", point);

    println!(" ");
    let complex = Complex {real: 3.3, imag: 7.2};
    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

}
