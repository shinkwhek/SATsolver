mod lexer;
mod cnf;

extern crate clap;
use clap::{App, Arg};

const VERSION_STR: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut app = App::new("satsolver")
        .version(VERSION_STR)
        .author("shinkwhek")
        .about("SAT solver written in Rust")
        .arg(Arg::with_name("FILE").help("Input file").index(1));

    let app_matches = app.clone().get_matches();

    let filename = match app_matches.value_of("FILE") {
        Some(filename) => filename,
        None => {
            app.print_help().unwrap();
            ::std::process::exit(0);
        }
    };

    let lexed = lexer::Lexer::run(filename);
    let cnf = cnf::Cnf::new(lexed);
    println!("{:?}", cnf);
}
