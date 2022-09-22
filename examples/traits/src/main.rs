fn main() {
    example();
    point_example();
    partial_ord_examples();
}

#[derive(Debug)]
struct Tuple<T> {
    first: T,
    second: T,
}

trait Max<T> {
    fn max(self) -> T;
}

#[derive(Debug, Hash, Clone, Copy, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: PartialOrd + Copy> Max<T> for Point<T> {
    fn max(self) -> T {
        match self.x.partial_cmp(&self.y) {
            Some(std::cmp::Ordering::Less) => self.y,
            Some(std::cmp::Ordering::Greater) | Some(std::cmp::Ordering::Equal) => self.x,
            None => self.y,
        }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Point<T> {
    type Output = Self;

    /// # Same as
    ///
    /// ```
    /// fn add(self, other: Self) -> Self {
    ///     Self {
    ///         x: self.x + other.x,
    ///         y: self.y + other.y,
    ///     }    
    /// }
    /// ```
    fn add(self, other: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    } 
}

// This is used to write the struct as a string when using {} formatting
impl<T> std::fmt::Display for Point<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait Beautify {
    fn beautify(self) -> String;
}

impl Beautify for u32 {
    fn beautify(self) -> String {
        format!("{self}<u32>")
    }
}

fn point_example() {
    let first_point = Point{x: 2u32, y: 1u32};
    let second_point = Point{x: 1u32, y: 2u32};
    let result = first_point + second_point;

    println!("{:?} or {}", result, result);
    println!("Max value of first_point {:?}", first_point.max());
    println!("Max value of second_point {:?}", second_point.max());
    println!("{} and {}", first_point.x.beautify(), first_point.y.beautify());
}

fn example() { 
    let tuple_u32 = Tuple{first: 1u32, second: 2u32};

    println!("{:?}", test(tuple_u32));
}

fn test<T: std::ops::Add<Output = T>>(t: Tuple<T>) -> T {
    return t.first + t.second
}

fn partial_ord_examples() {
    assert_eq!(1u32.partial_cmp(&2u32), Some(std::cmp::Ordering::Less));
    assert_eq!(2u32.partial_cmp(&1u32), Some(std::cmp::Ordering::Greater));
    assert_eq!(1u32.partial_cmp(&1u32), Some(std::cmp::Ordering::Equal));
}
