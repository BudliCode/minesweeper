use std::{env, fs};

use lib::annotate;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args.get(1).expect("please add a file as Argument");
    let mut output_file_name: String = input_file_name.to_string();
    output_file_name.replace_range(
        input_file_name.rfind('.').unwrap_or(input_file_name.len())..,
        ".out",
    );
    let minefield_binding = fs::read_to_string(&input_file_name)
        .expect(format!("file does not exist: {}", input_file_name).as_str());
    let minefield = minefield_binding.lines().collect::<Vec<&str>>();

    if !minefield.is_empty() {
        let len = minefield[0].len();
        for row in minefield.iter().skip(1) {
            if row.len() != len {
                std::process::exit(1);
            }
            for c in row.chars() {
                if c != '*' && c != ' ' {
                    std::process::exit(1);
                }
            }
        }
    }

    let new_minefiled = annotate(minefield.as_slice());
    let mut output_string = new_minefiled.join("\n");
    if !output_string.is_empty() {
        output_string.push('\n');
    }
    dbg!(&output_file_name);
    fs::write(output_file_name, output_string).expect("file could not be written")
}
