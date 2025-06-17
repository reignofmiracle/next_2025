use std::io::ErrorKind;

fn main() {
    // f1();
    // f2();
    f5();
}

fn f1() {
    panic!("crash and burn");
}

fn f2() {
    let v = vec![1, 2, 3];
    v[99];
}

fn f3() {
    use std::fs::File;

    let greeting_file_result = File::open("hello.txt");

    let greeting_file_result = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("{error:?}"),
    };
}

fn f4() {
    use std::fs::File;

    let greeting_file_result = File::open("hello.txt");

    let greeting_file_result = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("{e:?}"),
            },
            _ => {
                panic!("{error:?}");
            }
        },
    };
}

fn f5() {
    use std::fs::File;

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("{error:?}");
            })
        } else {
            panic!("{error:?}");
        }
    });
}

fn f6() {
    use std::fs::File;

    let greeting_file = File::open("hello.txt").unwrap();
}

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
