use std::io::{BufReader,BufRead};
use std::fs::File;


fn main() {
    let mut all_lines: Vec<String> = Vec::new();

    let testing = true;

    let file: File = if testing {
        File::open("test_input.txt").unwrap()
    }
    else{
        File::open("input.txt").unwrap()
    };
	
    for line in BufReader::new(file).lines() {
        all_lines.push(line.unwrap());
    }

    // _one(&all_lines);
    // _two(&all_lines);
}


fn _one(all_lines: &Vec<String>) {
    
}

fn _two(all_lines: &Vec<String>) {
    
}