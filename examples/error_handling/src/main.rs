fn main() {
    test_operation_error();
    test_operation_error_option();

    _ = expect_int("1".to_owned());
    _ = expect_int("something".to_owned()); // this will panic. unwrap() method also could panic.
}

#[derive(Debug)]
enum OperationsError {
    DivideByZeroError,
}

impl std::fmt::Display for OperationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::DivideByZeroError => f.write_str("divide by zero error"),
        }
    }
}

impl std::error::Error for OperationsError {
    fn description(&self) -> &str {
        match *self {
            Self::DivideByZeroError => "divide by zero error",
        }
    }
}

fn operation_error(a: u32, b: u32) -> Result<u32, OperationsError> {
    if b == 0 {
        Err(OperationsError::DivideByZeroError)
    } else {
        Ok(a / b)
    }
}

fn test_operation_error() {
    let first_result = operation_error(1, 0);
    let second_result = operation_error(100, 2);

    assert_eq!(first_result.is_err(), true);
    assert_eq!(second_result.is_ok(), true);

    println!("{:?}", first_result);
    println!("{}", first_result.err().unwrap());
    println!("{:?}", second_result.unwrap());
}

fn operation_error_option(a: u32, b: u32) -> Option<u32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn operation_error_myoption(a: u32, b: u32) -> MyOption<u32> {
    if b == 0 {
        Err(())
    } else {
        Ok(a / b)
    }
}

fn test_operation_error_option() {
    println!("result is: {}", test_operation_error_option_helper(1u32, 0u32));
    println!("result is: {}", test_operation_error_option_helper(100u32, 2u32));

    println!("result is: {}", operation_error_myoption(1u32, 0u32).is_err());
    println!("result is: {}", operation_error_myoption(100u32, 2u32).ok().unwrap());
}

fn test_operation_error_option_helper(a: u32, b: u32) -> String {
    match operation_error_option(a, b) {
        None => String::from("error"),
        Some(result) => format!("{} / {} = {}", a, b, result ),
    }
}

type MyOption<T> = Result<T, ()>;

fn expect_int(input: String) -> u64 {
    input.parse::<u64>().expect("Could not parse the integer")
}
