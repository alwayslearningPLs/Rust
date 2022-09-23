fn main() {
    memory_example();
    testing_ownership();
    testing_ownership_1();
    test_shared_references_are_immutable();
}

fn memory_example() {
    let x = 42; // by default it is i32
    let y = 43; // by default it is i32
    let _z = "Hello world!"; // This variable does not contain the full string, only the first
                            // character and the length of the string so we can loop it when
                            // needed.

    let pointer1 = &x;
    let mut pointer2 = &x;

    assert_eq!(pointer1, pointer2);
    println!("pointer1: {:p} and pointer2: {:p}", pointer1, pointer2);

    pointer2 = &y;

    println!("pointer2: {:p}", pointer2);
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct ClonePoint {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct CopyPoint {
    x: i32,
    y: i32,
}

fn testing_ownership () {
    let p1 = Point { x: 1, y: 5 };
    let p2 = p1;

    assert_eq!(p2, Point { x: 1, y: 5 });

    // p1.x = 2; // we can't do that, because we don't have the ownership of the variable.
    //
    let cp1 = ClonePoint { x: 1, y: 5 };
    let cp2 = cp1.clone(); // here we are clonning, so we have two different instances of the same
                           // bytes

    assert_ne!(format!("{:p}", &cp1), format!("{:p}", &cp2)); // They don't point to the same
                                                              // structure, but contain the same
                                                              // bytes
    assert_eq!(cp1, cp2);

    let cp1 = CopyPoint { x: 1, y: 5 };
    let cp2 = cp1; // We don't need to call clone, because we have implemented the Copy derive

    assert_ne!(format!("{:p}", &cp1), format!("{:p}", &cp2));
    assert_eq!(cp1, cp2);
}

#[derive(Debug, PartialEq)]
struct Box {
    value: u32,
}

fn testing_ownership_1() {
    let x1 = 42u32; // implements Copy trait
    let y1: Box = Box{ value: 84 }; // does not implement Copy trait
    
    assert_eq!(y1, Box { value: 84 });

    {
        let _z = (x1, y1);
    }

    let _x2 = x1;
    //let y2 = y1;
}

fn test_shared_references_are_immutable() {
    let mut sum: i32 = 2;
    println!("{:p}", &sum);
    shared_references_are_immutable(&2i32, &mut sum); 
}

fn shared_references_are_immutable(input: &i32, sum: &mut i32) {
    println!("{:p}", &sum); // the pointers are differents, when passing the share reference, Rust
                            // is creating a Copy of the previous one.
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}
