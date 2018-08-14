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

        let without_comment = lines
            .into_iter()
            .filter(|s| s.chars().nth(0) != Some('c'))
            .collect::<Vec<&str>>();

        let p_tags = without_comment
            .clone()
            .into_iter()
            .filter(|s| s.chars().nth(0) == Some('p'))
            .collect::<Vec<&str>>();

        if p_tags.len() == 0 || p_tags.len() > 1 {
            println!("error: there must be only one row with 'p'");
            ::std::process::exit(1);
        }

        let without_p = without_comment
            .into_iter()
            .filter(|s| s.chars().nth(0) != Some('p'))
            .collect::<Vec<&str>>();

        without_p.iter().map(Lexer::parse_line).collect()
    }

    fn parse_line(s: &&str) -> Vec<isize> {
        s.split(' ')
            .map(|n| n.parse::<isize>().unwrap())
            .filter(|&n| n != 0)
            .collect()
    }
}
