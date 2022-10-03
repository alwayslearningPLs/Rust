mod raw_pointer;

fn main() {
    raw_pointer::references();

    do_not_use_unsafe_code();
    get_uncheck_test();
    mem_test();
}
#[forbid(unsafe_code)] fn do_not_use_unsafe_code() {}
#[allow(dead_code)]
unsafe fn hello() {}

#[allow(dead_code)]
unsafe trait Greeter {}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn get_uncheck_test() {
    // Playing around with unsigned/signed
    let v_u8: Vec<u8> = vec![1, 2, 3];
    let v_i8: Vec<i8> = vec![10; 20];

    assert_eq!(v_u8[0], 1);
    assert_eq!(v_u8.get(1), Some(&2u8));
    assert_eq!(v_u8.get(0..3), Some(&[1u8, 2u8, 3u8][..]));
    assert_eq!(v_i8.len(), 20);

    // Creating vector of strings
    let v_str: Vec<&str> = vec!["Hello"; 3];
    let v_str_1: &[&str] = &["Hello"; 3];
    let v_string: Vec<String> = vec!["Hello".to_owned(); 3];
    // let v_string_1: &[String] = &["Hello".to_owned(); 3]; // the trait bound Copy is not
    // implemented
    let v_point: &[Point] = &[Point{ x: 1, y: 2 }; 3];

    assert_eq!(v_str[0], "Hello");
    assert_eq!(v_str_1[0], "Hello");
    assert_eq!(v_string[0], String::from("Hello"));
    assert_eq!(v_point[0], Point{x: 1, y: 2});

    let mut v_new: Vec<u8> = std::vec::Vec::<u8>::new();
    v_new.push(2u8);

    unsafe {
        assert_eq!(v_new.get_unchecked(0), &2u8);
    }

    assert_eq!(format!("{:?}", v_new), "[2]");
}

fn mem_test() {
    let (size_u8,
         size_i8, 
         size_u16, 
         size_i16, 
         size_u32, 
         size_i32, 
         size_u64, 
         size_i64, 
         size_u128, 
         size_i128,
         size_char,
         size_bool,
         size_f32,
         size_f64,
         size_empty_tuple):
        (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) = (
             std::mem::size_of::<u8>(),
             std::mem::size_of::<i8>(),
             std::mem::size_of::<u16>(),
             std::mem::size_of::<i16>(),
             std::mem::size_of::<u32>(),
             std::mem::size_of::<i32>(),
             std::mem::size_of::<u64>(),
             std::mem::size_of::<i64>(),
             std::mem::size_of::<u128>(),
             std::mem::size_of::<i128>(),
             std::mem::size_of::<char>(),
             std::mem::size_of::<bool>(),
             std::mem::size_of::<f32>(),
             std::mem::size_of::<f64>(),
             std::mem::size_of::<()>());

    assert_eq!(size_u8, 1);
    assert_eq!(size_i8, 1);

    assert_eq!(size_u16, 2);
    assert_eq!(size_i16, 2);

    assert_eq!(size_u32, 4);
    assert_eq!(size_i32, 4);

    assert_eq!(size_u64, 8);
    assert_eq!(size_i64, 8);

    assert_eq!(size_u128, 16);
    assert_eq!(size_i128, 16);

    assert_eq!(size_char, 4);

    assert_eq!(size_bool, 1);

    assert_eq!(size_f32, 4);
    assert_eq!(size_f64, 8);

    assert_eq!(size_empty_tuple, 0);
}
