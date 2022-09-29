// Rust groups errors into two major categories:
//  - recoverable (file not found)
//  - unrecoverable (IndexOutOfBoundException)
//
// For recoverable errors, Rust uses `Result<T, E>`
// panic! macro for unrecoverable
//
// When an unrecoverable error occurs, Rust has two options. One is to unwind the stack, so it
// walks back and starts to clean up or just abort so the allocated memory would be cleaned up by
// the OS.
//
// If you want the second (in the case of having a smaller binary), you can set the property in
// your desired profile inside the toml file:
//
// [profile.release]
// panic = 'abort'
//

// Use with RUST_BACKTRACE=1 or RUST_BACKTRACE=full
#[allow(dead_code)]
pub fn panicking() {
    let v = vec![3];

    v[10]; // it will panic
}

pub fn file_not_found() {
    match std::fs::File::open("not_existent_file.txt") {
        Ok(_) => println!("file found"),
        Err(error) => println!("error opening the file {:?}", error),
    }
}

pub fn file_not_found_matching_error() {
    file_not_found_matching_error_test("not_existent");         // file not found
    file_not_found_matching_error_test("/var/log/boot.log");    // permission denied
    file_not_found_matching_error_test("/var/run/docker.sock"); // other kind of error
                                                                //
    // file_not_found_matching_error_test_using_closures("not_existent");         // file not found
    // file_not_found_matching_error_test_using_closures("/var/log/boot.log");    // permission denied
    // file_not_found_matching_error_test_using_closures("/var/run/docker.sock"); // other kind of error
}

fn file_not_found_matching_error_test(file_name: &str) {
    match std::fs::File::open(file_name) {
        Result::Ok(_) => println!("File found"),
        Result::Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => println!("file not found"),
            // std::io::ErrorKind::IsADirectory => println!("the file is not a file, it is a directory"), // this is an unstable feature
            std::io::ErrorKind::PermissionDenied => println!("permission denied to open this file"),
            other_kind_of_error => println!("{}", other_kind_of_error),
        }
    }
}

#[allow(dead_code)]
fn file_not_found_matching_error_test_using_closures(file_name: &str) {
    std::fs::File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            panic!("file not found");
        } else {
            panic!("other error");
        }
    });
}

// This will panic
#[allow(dead_code)]
pub fn unwrap_file_not_found() {
    std::fs::File::open("not_existent_file.txt").unwrap();
}

#[allow(dead_code)]
pub fn expect_file_not_found() {
    std::fs::File::open("not_existent_file.txt").expect("file was not found");
    std::fs::File::open("not_existent_file.txt").expect_err("file was not found");
}
