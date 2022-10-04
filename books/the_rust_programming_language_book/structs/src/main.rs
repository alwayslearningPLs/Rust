// keywords:
//  - field init shorthand (syntax)
//  - struct update (syntax)
//  - tuple structs (the ones with fields without names)
//  - unit-like structs (the ones that doesn't have any field)
//  - automatic referencing/dereferencing (instead of using "dot" or "arrow", to call a pointer or
//  not)

use std::fmt::{Display, Formatter, Result};

fn main() {
    let u = build_user("mrtimeout".to_owned(), "mrtimeout@gmail.com".to_owned());

    dbg!(u);
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "username {} with email {} is still {} with the amount of sign in {}",
               self.username, self.email, self.active, self.sign_in_count)
    }
}

#[derive(Debug, PartialEq)]
struct User1<'a> {
    username: &'a str,
    email: &'a str,
}

#[allow(dead_code)]
struct Rgb(u8, u8, u8);

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Point(i32, i32, i32);

impl Point {

    fn add(&self, p: &Point) -> Self {
        Self (self.0 + p.0, self.1 + p.1, self.2 + p.2)
    }

    // sqrt((x2 - x1)^2 + (y2 - y1)^2 + (z2 - z1)^2)
    fn distance(&self, p: &Point) -> f64 {
        (((p.0 - self.0).pow(2u32) + (p.1 - self.1).pow(2u32) + (p.2 - self.2).pow(2u32)) as f64).sqrt()
    }

}

#[allow(dead_code)]
// Here we are using the "field init shorthand" syntax, so we don't need to rewrite the email and
// username structs names.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[test]
fn user_test() {
    let u: User = User {
        username: "MrTimeout".to_owned(),
        email: "mrtimeout@gmail.com".to_owned(),
        active: true,
        sign_in_count: 1,
    };

    assert_eq!(u.username, "MrTimeout".to_owned(), "username is not equal");
    assert_eq!(u.email, "mrtimeout@gmail.com".to_owned(), "email is not equal");
    assert!(u.active, "actice is not true");
    assert_eq!(u.sign_in_count, 1, "sign in count is not equal");
}

#[test]
fn user_mut_test() {
    let mut u: User = User {
        username: "MrTimeout".to_owned(),
        email: "mrtimeout@gmail.com".to_owned(),
        active: true,
        sign_in_count: 1,
    };

    u.active = false;
    u.sign_in_count += 1;


    assert_eq!(u.username, "MrTimeout".to_owned(), "username is not equal");
    assert_eq!(u.email, "mrtimeout@gmail.com".to_owned(), "email is not equal");
    assert!(!u.active, "actice is not false");
    assert_eq!(u.sign_in_count, 2, "sign in count is not equal");
}

#[test]
fn build_user_test() {
    let u: User = build_user(String::from("mrtimeout@gmail.com"), String::from("MrTimeout"));

    assert_eq!(u.username, "MrTimeout".to_owned(), "username is not equal");
    assert_eq!(u.email, "mrtimeout@gmail.com".to_owned(), "email is not equal");
    assert!(u.active, "actice is not true");
    assert_eq!(u.sign_in_count, 1, "sign in count is not equal");
}

#[test]
fn user_update_test() {
    let from: User = build_user(String::from("mrtimeout@gmail.com"), String::from("MrTimeout"));
    let to: User = User {
        email: "another@gmail.com".to_owned(),
        username: "another".to_owned(),
        ..from
    };

    assert_eq!(from.active, to.active);
    assert_eq!(from.sign_in_count, to.sign_in_count);
}

#[test]
fn user_display_test() {
    let u: User = build_user(String::from("mrtimeout@gmail.com"), String::from("MrTimeout"));

    assert_eq!(format!("{}", u), "username MrTimeout with email mrtimeout@gmail.com is still true with the amount of sign in 1");
}

#[test]
fn user1_test() {
    let from: User1 = User1 {
        username: "username here",
        email: "email here",
    };
    let to: User1 = User1 {
        ..from
    };

    assert_eq!(from, to, "users are different");
}

#[test]
fn rgb_test() {
    let rgb = Rgb(216u8, 52u8, 21u8); // Rust color code

    assert_eq!(rgb.0, 216u8);
    assert_eq!(rgb.1, 52u8);
    assert_eq!(rgb.2, 21u8);
}

#[test]
fn point_addition_test() {
    let (first, second) = (Point(1, 5, 4), Point(4, 10, 3));

    assert_eq!(first.add(&second), Point(5, 15, 7));
}

#[test]
fn point_distance_test() {
    let (first, second) = (Point(1, 10, 5), Point(10, 4, 2));

    assert_eq!(first.distance(&second), 11.224972160321824f64);
}

