fn main() {
}

enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[test]
fn vector_new_test() {
    let (v, vv): (Vec<i32>, Vec<u8>) = (Vec::new(), vec![1, 2, 3]);

    assert!(v.len() == 0);
    assert_eq!(vv, [1, 2, 3]);
}

#[test]
fn vector_push_test() {
    let mut v: Vec<u8> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    assert_eq!(v, [1, 2, 3]);
}

#[test]
fn vector_accessing_elements_test() {
    let v: Vec<u8> = vec![1, 2, 3, 4, 5];

    let second_1: &u8 = &v[1];
    let second_2: Option<&u8> = v.get(1); // this option is very useful if we don't know the limit
                                          // of the vector

    match second_2 {
        Some(result) => assert_eq!(second_1, result),
        None => assert!(false),
    }
}

#[test]
fn vector_iter_test() {
    let mut v: Vec<u8> = vec![1, 2, 3, 4];

    // immutable
    for each in &v {
        assert!(*each > 0);
    }

    // mutable
    for each in &mut v {
        *each *= 2;
    }

    assert_eq!(v, [2, 4, 6, 8]);
}

#[test]
fn vector_cell_test() {
    let v: Vec<Cell> = vec![
        Cell::Int(10),
        Cell::Float(2.0),
        Cell::Text("Hello world".to_owned()),
    ];

    assert!(v.len() == 3);
}
