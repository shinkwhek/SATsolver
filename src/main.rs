mod lexer;
mod core;

extern crate clap;
use clap::{App, Arg};

extern crate ansi_term;
use self::ansi_term::Colour;

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
            println!("");
            ::std::process::exit(0);
        }
    };

    let mut cnf = lexer::Lexer::run(filename);
    let mut assignment: Vec<isize> = Vec::new();
    if let Some(mut result) = core::DPLL::solver(&mut cnf, &mut assignment) {
        println!("{}", Colour::Blue.bold().paint("SATISFIABLE"));
        esort(&mut result);
        println!("{:?}", result);
    } else {
        println!("{}", Colour::Red.bold().paint("UNSATISFYABLE"));
    }
}

fn esort(a: &mut Vec<isize>) {
    quick_iter(0, a.len() - 1, a);
}

fn quick_iter(left: usize, right: usize, data: &mut Vec<isize>) {
    if left >= right {
        return;
    }

    let (mut lf, mut rg) = (left, right);
    let pivot = data[(lf + rg) >> 1].abs().clone();
    loop {
        while data[lf].abs() < pivot {
            lf += 1;
        }
        while pivot < data[rg].abs() {
            rg -= 1;
        }
        if lf >= rg {
            break;
        }
        data.swap(rg, lf);
        rg -= 1;
        lf += 1;
    }
    if left + 1 < lf {
        quick_iter(left, lf - 1, data);
    }
    if rg + 1 < right {
        quick_iter(rg + 1, right, data);
    }
}
