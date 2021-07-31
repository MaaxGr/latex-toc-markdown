mod toc_formatter;

use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let input_file_name = std::env::args().nth(1).expect("First argument has to be the input file name!");
    let output_file_name = std::env::args().nth(2).expect("Second argument has to the output file name!");

    let lines_result = read_lines_of_file(input_file_name.as_str());

    if lines_result.is_err() {
        println!("Error while reading lines of {}: {}", input_file_name, lines_result.unwrap_err());
        return;
    }


    let xx: Vec<String> = lines_result.unwrap()
        .into_iter()
        .map(|line| toc_formatter::line_to_md(line.as_str()))
        .collect();

    let markdown = xx.join("\n");

    fs::write(output_file_name, markdown).expect("Unable to write file");
}


fn read_lines_of_file(file_name: &str) -> Result<Vec<String>, &str> {
    let file_result = File::open(file_name);
    if file_result.is_err() {
        return Err("Unable to open")
    }
    let file = file_result.unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut lines_vec = Vec::new();

    for line in lines {
        if let Ok(l) = line {
            lines_vec.push(l);
        } else {
            return Err("Unable to read line")
        }
    }

    return Ok(lines_vec)
}
