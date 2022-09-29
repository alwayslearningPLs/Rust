#[derive(Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    
    pub fn add(&self, p: &Point<T>) -> Self where T: std::ops::Add<Output = T> + Copy {
        Self {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }

    pub fn translate(&self, multiplier: T) -> Self where T: std::ops::Mul<Output = T> + Copy {
        Self {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }

    pub fn execute_fn<F>(&self, map: F) -> Self where F: std::ops::Fn(&Self) -> Self {
        map(self)
    }
}

impl<T> std::fmt::Display for Point<T> where T: std::fmt::Display {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }

}

// A unit struct
struct A;

// Tuple struct, which are, basically, named tuples.
struct B(A);

struct C<T>(T);

// Classic C structs
#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
}

pub fn removing_warnings() {
    let (_, _, _, _): (A, B, C<i32>, Person) = (A, B(A), C(2i32), Person{ name: "MrTimeout".to_owned(), age: 24u8 });
}

pub fn dropping_structs() {
    let a = A;

    dropping_structs_test(a);

    // We can't use a variable anymore because it was dropped
    // let b = a; <- Uncomment this
}

fn dropping_structs_test<T>(_: T) {
   println!("Dropping after taking ownership");
}
