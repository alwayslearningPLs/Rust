// Data with an unknown size at compile time or a size that might change must be stored on the
// heap.
//
// Ownership rules:
//
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
//
// Data race:
//
// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write the data.
// There's no mechanism being used to synchronize access to the data.
//
// Terms:
// - double free
// - Shallow copy or deep copy
// - referencing
// - dereferencing
// - borrowing
// - dangling pointer
// - lifetime
// - Lifetime Elision
// - static lifetime

pub fn scope() {
    {
        // This is known as a "string literal". We know its size at compile time and are immutable
        let s: &str = "Hello world";

        // Here we can play around with s variable
        println!("{}", s)
    }

    // Cannot find this variable in this scope...
    // println!("{}", s);
}

pub fn string_type() {
    let str_literal: &str = "Hello world";
    let mut string: String = String::from(str_literal);

    string.push_str("!");

    println!("{} and {}", str_literal, string); // Rust call drop function when string goes out of scope
}

pub fn string_double_free() {
    let string_literal: &str = "Hello";
    let s1: String = String::from(string_literal);
    let s2: String = s1; // Here s2 and s1 are pointing to the same place of memory. To avoid
                         // double free of this memory when both variables go out of scope, Rust
                         // invalidates the first one, so you can only use the last one.

    // assert_eq!(s1, string_literal);
    assert_eq!(s2, string_literal); // Notice how we can use string_literal despite being used in
                                    // the first assigment of String variables. Remember that
                                    // string literals have a fixed size at compile time, so they
                                    // reside on the stack, not on the heap.
}

pub fn string_double_free_solution() {
    let string_literal: &str = "Hello";
    let s1: String = String::from(string_literal);
    // The name of this operation is: shallow copy or deep copy.
    let s2: String = s1.clone(); // String implements the trait Copy, so Rust can copy all the
                                 // structure of the String object, and give it to s2 (the pointer
                                 // to that structure on the heap). So, we have two different
                                 // instances os the same content structure but with different
                                 // pointer.
    // Rust won't let us annotate a type with Copy if the type, or any of its parts, has
    // implemented the Drop trait.

    assert_eq!(s1, string_literal);
    assert_eq!(s2, string_literal);
    assert_ne!(format!("{:p}", s1.as_ptr()), format!("{:p}", s2.as_ptr()));
}

pub fn ownership_on_functions() {
    let s: String = String::from("Hello world");

    i_take_ownership_of_you(s);

    // We can't use it because the variable s passed away
    // assert_eq!(s, "Hello world");
   
    let d: i32 = 2;

    i_copy_you_because_you_have_copy_trait(d); // We just call Copy so we have two instances of the
                                               // same value. Remember that this value is on the
                                               // stack.

    assert_eq!(d, 2);
}

fn i_take_ownership_of_you(i: String) { // It take ownership of the variable passed as a parameter
    assert_eq!(i, "Hello world");
} // I drop the variable and its pointer.

fn i_copy_you_because_you_have_copy_trait(i: i32) {
    assert_eq!(i, 2);
}

pub fn ownership_on_functions_borrowing() {
    let s: String = String::from("Hello world");

    let size = i_borrow_pointer_and_do_not_owner_it(&s); // borrowing the pointer so the function
                                                         // does not own it.

    assert_eq!(s, "Hello world");
    assert_eq!(size, s.len());
}

fn i_borrow_pointer_and_do_not_owner_it(s: &String) -> usize {
    return s.len();
}

pub fn mutable_references() {
    let mut s: String = String::from("Hello world");

    push_some_value(&mut s);

    assert_eq!(s, "Hello world!");
}

fn push_some_value(s: &mut String) {
    s.push_str("!");
}

pub fn mutable_references_no_more_than_one() {
    let mut s: String = String::from("Hello world");

    let _ss1: &String = &mut s;
    // let ss2: &String = &mut s; // cannot borrow `s` as mutable more than once at a time.
    // this prevents data races at compile time.

    // println!("{} {}", ss1, ss2);
}

pub fn mutable_references_in_diff_scopes_but_not_simultaneous() {
    let mut s: String = String::from("Hello world");

    {
        let r: &mut String = &mut s;
    
        r.push('!');
        assert_eq!(r, "Hello world!");
    }

    assert_eq!(s, "Hello world!");
}

pub fn n_immutable_references() {
    let mut s: String = String::from("Hello world");

    let r1: &String = &s;
    let r2: &String = &s;
   
    // We can't get a mutable reference to s, because we are using r1 and r2 after mutable ref
    //let r3: &mut String = &mut s;

    // We can't execute this order, because we are borrowing a immutable reference of the content
    // and if we try to modify it, the data underlying the r1 r2 will change and we don't want
    // that.
    // s.push('s');

    println!("{}, {}, {}", s, r1, r2);
}

pub fn n_immutable_reference_sol() {
    let mut s: String = String::from("Hello world");

    let r1: &String = &s;
    let r2: &String = &s;

    assert_eq!(*r1, s);
    assert_eq!(*r2, s);

    let r3: &mut String = &mut s;

    r3.push('!');

    assert_eq!(*r3, "Hello world!");
}

// dangling_pointer is a pointer that references a location in memory that may have been given to
// someone else
// pub fn dangling_pointer() {
//   let _s: &String = dangling_pointer_test();
// }

// fn dangling_pointer_test<'a>() -> &'a String {
//    let s: String = String::from("Hello world");
//    return &s;
// }


// lifetime ensure that references are valid as long as we need them to be.
// The main aim of lifetimes is to prevent dangling references

pub fn lifetime() {
    let (s1, s2): (String, String) = (String::from("Hello"), String::from("World"));

    let result: &str = longest_string(s1.as_str(), s2.as_str());

    assert_eq!(result, "Hello");
}

pub fn lifetime_with_the_little_one() {
    let s1: String = String::from("Hello");
    let result: &str;

    {
        let s2: String = String::from("World");
        result = longest_string(s1.as_str(), s2.as_str());
        assert_eq!(result, "Hello");
    } // here Rust drop s2 variable so we can't use s2 or result value because of lifetime.

    // assert_eq!(result, "Hello");
}

pub fn lifetime_with_only_one_parameter() {
    let s1: String = String::from("Hello");
    let result: &str;

    {
        let s2: String = String::from("World");
        result = longest_string_modified(s1.as_str(), s2.as_str());
        assert_eq!(result, "Hello");
    }

    assert_eq!(result, "Hello");
}

fn longest_string<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}

fn longest_string_modified<'a>(left: &'a str, right: &str) -> &'a str {
    println!("{}", right);
    left
}

struct Something<'a> {
    value: &'a str,
}

impl<'a> Something<'a> {
    fn s(&self, another: &str) -> &str {
        assert_eq!(another, "hello");
        return self.value;
    }

    fn upper(&self) -> String {
        self.value.to_uppercase()
    }
}

pub fn lifetime_in_structs() {
    let input: &str = "Hello world";
    let s: Something = Something{ value: input };

    assert_eq!(input, s.value);

    s.s("hello");
    s.upper();
}

pub fn first_word_test() {
    let input: String = String::from("Hello world from Rust");

    let result: &str = first_word(input.as_str());

    assert_eq!(result, "Hello");
}

// We don't need here lifetimes because the lifetime is inferred.
// Lifetime Elision rules:
//  - First rule is that compiler assigns a lifetime for each of the paramters
//  - Second rules is that if there is exactly one parameter, then the output lifetime of the
//  return value will be the same as the input.
//  - Third rule is that if there is more than one input parameter and one of them is &self or &mut
//  self, the lifetime of self if assigned to all output lifetime parameter.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (pos, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[0..pos];
        }
    }

    &s[..]
}

trait Bar<T> {
    fn bar(&self) -> T where T: Copy;
}

struct Foo<'a, T> {
    data: Vec<Box<&'a dyn Bar<T>>>,
}

// We use dyn because we don't know at compile time which structure we are passing, despite knowing
// that we are passing a trait.
impl<'a, T> Foo<'a, T> {
    // x parameter will live for the entire lifetime of the struct Foo.
    fn add(&mut self, x: &'a dyn Bar<T>) {
        self.data.push(Box::new(x));
    }
}

#[derive(Debug, Clone)]
struct BarImpl<T> {
    value: T,
}

impl<T> Bar<T> for BarImpl<T> {
    fn bar(&self) -> T where T: Copy {
        self.value
    }
}

pub fn testing_structs_foo() {
    let bar_impl: &dyn Bar<i32> = &BarImpl{value: 2i32};
    let b: Box<&dyn Bar<i32>> = Box::new(bar_impl);
    let vec_b: Vec<Box<&dyn Bar<i32>>> = vec![b; 3];

    assert_eq!(vec_b.len(), 3);

    let mut f = Foo{ data: vec_b };

    {
        f.add(&BarImpl{value: 3i32});
   
        assert_eq!(f.data[3].bar(), 3);
    }

    assert_eq!(f.data[3].bar(), 3);
}
