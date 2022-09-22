fn main() {
    test_lifetime();
}

fn test_stack_example() {
    let i: i32 = 42;
    stack_example(i);
}

fn stack_example(i: i32) {
    let x = i;
    let _y = x;
    println!("{}", x);
}

fn test_heap_example() {
    let s = String::from("test");
    heap_example(s);
}

fn heap_example(input: String) {
// The owner is created on the stack and points to the value on the heap because the compiler
// doesn't know the length of the string at compile time.
    let mystr = input;
    let _otherstr = mystr.clone(); // How to solve? use mystr.clone(), so we are cloning the string on the
                           // heap
    println!("{}", mystr);
}

fn test_heap_example_reference() {
    let input = String::from("Hello world");
    heap_example_reference(&input);
}

// A reference allows you to refer to a resource without actually owning it.
fn heap_example_reference(input: &String) {
    let mystr = input;
    let _otherstr = mystr;
    println!("{}", mystr);
}

fn test_heap_example_reference_is_immutable() {
    let input = String::from("Hello world!");
    heap_example_reference_is_immutable(&input);
}

fn heap_example_reference_is_immutable(input: &String) {
    let mystr = String::from("Hello world");
    // input = &mystr; // cannot assign to immutable argument `input` AND `mystr` does not live long enough

    // println!("{}", *input);
    println!("{}", mystr);
}

fn test_heap_example_reference_mutable() {
    let mut input = String::from("Hello world");
    heap_example_reference_mutable(&mut input);
}

fn heap_example_reference_mutable(input: &mut String) {
    let mut mystr = String::from("Bye world");
    //input = &mystr; // mismatched types.

    //println!("{}", *input);
    println!("{}", mystr);
}

fn test_heap_example_reference_mutable_borrowed_once_in_scope() {
    let mut input = String::from("hello");
    heap_example_reference_mutable_borrowed_once_in_scope(&mut input);
}

// Mutable references can be only borrowed once in a scope.
fn heap_example_reference_mutable_borrowed_once_in_scope(input: &mut String) {
    let a = input;
    //let _b = a;

    println!("{}", a);
}

fn test_lifetime() {
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6];

    println!("{:?}", lifetime(&v1, &v2));
}

// We need lifetime specifier
// fn lifetime_failing(a: &[i32], b: &[i32]) -> &[i32] {
//    if a.len() > b.len() { a } else { b }
// }

fn lifetime<'a>(a: &'a[i32], b: &'a[i32]) -> &'a[i32] {
    if a.len() > b.len() { a } else { b }
}
