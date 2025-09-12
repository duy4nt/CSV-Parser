use std::fs::Files;
use std::io::ErrorKind;
use std::io::prelude::*;

fn main() {
    let file_result = File::open("file.csv");
    
    let mut file = match file_result {
        Ok(file) => file,
        Err(file) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.csv") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic("Problem opening the file: {other_error:?}")
            }
        },
    };

}
