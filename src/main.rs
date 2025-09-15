use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;

fn main() {
    let file_result = File::open("file.csv");
    let mut parsed_file: Vec<Vec<String>> = vec![];
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
    let mut column_counter: usize = 0;

    for content in row {
        let last_two_chars_chars: Option<String> = last_two_chars(content);
        let last_two_chars_chars_string: String = match last_two_chars_chars {
            Some(s) => s,
            None => "Unable to unfold the last_to_chars_chars".to_string(),
        };
        if last_two_chars_chars_string == "/n" {
            row_counter += 1;
            column_counter = 0;
        }
        parsed_file[row_counter][column_counter].push_str(content);
        column_counter += 1; 
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

