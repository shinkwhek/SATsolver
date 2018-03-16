use std::io::prelude::*;
use std::fs::OpenOptions;

pub struct Lexer {}

impl Lexer {
    pub fn run(filename: &str) -> Vec<Vec<isize>> {
        let mut file = if let Ok(ok) = OpenOptions::new().read(true).open(filename) {
            ok
        } else {
            println!("error: file '{}' was not found", filename);
            ::std::process::exit(1);
        };

        let mut file_body = String::new();
        match file.read_to_string(&mut file_body) {
            Ok(_) => (),
            Err(e) => {
                println!(
                    "error: an error occurred while reading file '{}'\n{}",
                    filename, e
                );
                ::std::process::exit(1);
            }
        }

        let lines = file_body.lines().collect::<Vec<&str>>();

        lines.iter().map(Lexer::parse_line).collect()
    }

    fn parse_line(s: &&str) -> Vec<isize> {
        s.split(' ').map(|n| n.parse::<isize>().unwrap()).collect()
    }
}
