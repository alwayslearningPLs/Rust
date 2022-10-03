// static variables can only store references with the 'static lifetime, which means Rust compiler
// can figure out the lifetime and we aren't required to annotate it explicitly. Accessing an
// immutable static variable is safe.
// One subtle difference between constants and immutable static variable is that values in a static
// variable have a fixed address in memory. 
static HELLO_WORLD: &'static str = "Hello world!"; // or static HELLO_WORLD: &str = "Hello world!";

static mut COUNTER: i32 = 0;

pub fn references() {
    {
        let from: String = "Hello world".to_owned();

        // We can have one or more immutable shared references.
        let to1: &String = &from;
        let to2: &String = &from;
        let to3: &String = &from;

        assert_eq!(*to1, from);
        assert_eq!(*to2, from);
        assert_eq!(*to3, from);
    }

    {
        let mut from: String = "Hello world".to_owned();

        let to1: &String = &from;
        let to2: &String = &from;
        let to3: &String = &from;

        // from = "Hello".to_owned(); // we can't change the inner value of from, because there is
        // one or more shared references that are referencing the value inside the from structure.
        
        // from.push_str("!"); // same here

        assert_eq!(*to1, from);
        assert_eq!(*to2, from);
        assert_eq!(*to3, from);
    }

    // We can create raw pointers in safe code, but not dereference them outside unsafe code.
    {
        let mut from: String = "Hello world".to_owned();

        // (un)safe code allows us to create one immutable raw pointer and one mutable raw pointer at
        // the same time.
        let _to1: *const String = &from as *const String;
        let _to2: *mut String = &mut from as *mut String;
        let _to3: *mut String = &mut from as *mut String;
    }

    // Don't write code like this
    {
        let address = 0x0123456usize;
        let _raw_pointer: *const i32 = address as *const i32; // creating a raw pointer to an
                                                              // arbirtary mem address.
    }

    {
        let mut from: String = "Hello world".to_owned();

        let to1: *const String = &from as *const String;
        let to2: *mut String = &mut from as *mut String;

        from.push_str("!"); // WoW, we can call push_str

        unsafe {
            assert_eq!(*to1, from); // Here the value under to1 and to2 has changed
            assert_eq!(*to2, from);
        }
    }

    {
        // 0x30 - 0x39
        let buf: &[u8] = &[0x38u8, 0x30u8, 0x38u8, 0x30u8, 0x00u8];// ['8', '0', '8', '0', '\0'];

        let mut buf_pointer: *const u8 = &buf[0] as *const u8;

        unsafe {
            let mut r: String = String::from("");

            while *buf_pointer != 0x00u8 {
                r.push(*buf_pointer as char);
                buf_pointer = buf_pointer.add(1);
            }

            assert_eq!(r, "8080");
        }
    }

    {
        let mut v: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        let r: &mut[u8] = &mut v[..];

        let (left, right) = r.split_at_mut(3usize);

        assert_eq!(left, &mut [1, 2, 3]);
        assert_eq!(right, &mut [4, 5, 6]);
    }

    {
        let mut v: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        let r: &mut[u8] = &mut v[..];

        let (left, right) = split_at_must_custom(r, 3usize);

        assert_eq!(left, &mut[1, 2, 3]);
        assert_eq!(right, &mut[4, 5, 6]);
    }

    {
        assert_eq!(HELLO_WORLD, "Hello world!");

        // changing the value of a static mutable variable is unsafe
        // COUNTER += 1;
        unsafe {
            COUNTER += 1;
            assert_eq!(COUNTER, 1);
        }
    }
}

// fn split_at_must_err(v: &mut[u8], mid: usize) -> (&mut[u8], &mut[u8]) {
//     let len = v.len();
//
//      assert!(mid <= len);
//
//    return (v[..mid], v[mid..]); // we can't borrow from two mutable shared references at the same
                                 // time...
//}

fn split_at_must_custom(v: &mut[u8], mid: usize) -> (&mut[u8], &mut[u8]) {
    let len = v.len();
    let ptr: *mut u8 = v as *mut [u8] as *mut u8; // let ptr = v.as_mut_ptr();

    // This code is not needed to the function to work...
    unsafe {
        let first_pointer: *mut [u8] = v as *mut [u8];
        let mut second_pointer: *mut u8 = first_pointer as *mut u8;
        let mut full_pointer: *mut u8 = v as *mut[u8] as *mut u8;

        assert_eq!((*first_pointer).len(), len);
        assert_eq!(*second_pointer, v[0]);
        assert_eq!(*full_pointer, *second_pointer);

        if len >= 2 {
            second_pointer = second_pointer.add(1);
            full_pointer = full_pointer.add(1);

            assert_eq!(*second_pointer, v[1]);
            assert_eq!(*second_pointer, *full_pointer);
        }
    }

    assert!(mid <= len);
    
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}
