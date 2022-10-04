use std::io::{Write, Read};

#[test]
fn file_test() {
    let (file_name, content): (&str, &str) = ("my_file.txt", "Hello world");

    {
        // write-only mode, truncate if it exists and create.
        match &mut std::fs::File::create(file_name) {
            Err(e) => panic!("this shouldn't happen {}", e),
            Ok(write_stream) => {
                write_stream.write(content.as_bytes()).expect("error trying to write our string to file");
            }
        }
    }

    {
        // read-only mode
        match &mut std::fs::File::open(file_name) {
            Err(e) => panic!("{}", e),
            Ok(read_stream) => {
                let mut buf = [0; 512];
                let r = match read_stream.read(&mut buf) {
                    Err(e) => panic!("this shouldn't happen {}", e),
                    Ok(size) => match size {
                        0 => panic!("this shouldn't happen"),
                        size => std::str::from_utf8(&buf[..size]).unwrap().to_string(),
                    }
                };

                assert_eq!(r, content);
            }
        }
    }

    std::fs::remove_file(file_name);
}

