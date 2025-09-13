use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;

fn main() {
    let file_result = File::open("file.csv");
    let parsed_file: Vec<Vec<&str>>;
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.csv") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}")
            }
        },
    };
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Problem in reading the file");
    let row: Vec<&str> = content.split(',').collect();
    println!("{:?}", row);

    let mut row_counter: usize = 0;
    
    for content in row {
        if last_two_chars(content) == "/n" {
            row_counter += 1;
        }
        parsed_file[row_counter].push(content);
    }
    println!("{:?}", parsed_file);
}

fn last_two_chars(s: &str) -> Option<String> {
    let mut char_iter = s.chars().rev();
    let last_char = char_iter.next();
    let second_last_char = char_iter.next();
    
    match (second_last_char, last_char) {
        (Some(c2), Some(c1)) => Some(format!("{}{}", c2, c1)),
        _=> None,
    }

}

