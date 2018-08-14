use std::fs::OpenOptions;
use std::io::prelude::*;

type Cnf = Vec<Vec<isize>>;

pub struct Lexer;
impl Lexer {
    pub fn new(filename: &str) -> Cnf {
        let mut file = if let Ok(ok) = OpenOptions::new().read(true).open(filename) {
            ok
        } else {
            println!("the file was not found");
            ::std::process::exit(1);
        };

        let mut file_body = String::new();
        match file.read_to_string(&mut file_body) {
            Ok(_) => (),
            Err(e) => {
                println!("an error occurred while reading file: {}", e);
                ::std::process::exit(1);
            }
        }

        let lines = file_body.lines().collect::<Vec<&str>>();

        lines
            .iter()
            .filter(|s| s.chars().nth(0) != Some('c'))
            .filter(|s| s.chars().nth(0) != Some('p'))
            .map(|s| Lexer::parse_line(s))
            .collect()
    }

    fn parse_line(s: &str) -> Vec<isize> {
        s.split(' ')
            .map(|n| n.parse::<isize>().unwrap())
            .filter(|&n| n != 0)
            .collect()
    }
}
