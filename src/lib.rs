#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use nom::*;

// Mini gtypist script language reference:
// command_char : command_data
//
// 1. Blank lines and lines starting with '#', are ignored
// - B: Clears the whole screen and shows the command_data on top of the screen
//      Maximum one line
// - X: Exit gtypist (command_data is ignored)
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command {
    Banner(String),
    Exit(String)
}

named!(banner<&str, Command>,
    do_parse!(
        tag!("B:")               >>
        space0                   >>
        value: take_until_and_consume!("\n") >>
        (Command::Banner(value.to_string()))
    )
);

named!(exit<&str, Command>,
    do_parse!(
        tag!("X:")               >>
        space0                   >>
        value: take_until_and_consume!("\n") >>
        (Command::Exit(value.to_string()))
    )
);

named!(parse<&str, Vec<Command>>,
    many0!(
        complete!(
            alt!(
                banner |
                exit
            )
        )
    )
);

pub fn parse_str(input: &str) -> Result<Vec<Command>, String> {
    match parse(input) {
        Ok(result) => Ok(result.1),
        Err(e) => Err(format!("Something went wrong! {:?}", e))
    }
}

pub fn parse_file(filename: String) -> Vec<Command> {
    let f = File::open(filename).expect("File does not exist");
    let mut string = String::new();
    BufReader::new(&f).read_to_string(&mut string);
    parse_str(&string).unwrap()
}

#[test]
fn test_banner_parse() {
    let result = banner("B: Lesson 1: uh\n");
    assert_eq!(result, Ok(("", Command::Banner("Lesson 1: uh".to_string()))));
}

#[test]
fn test_parse_str() {
    let result = parse_str("B: Lesson 1: uh\nB: Lesson 2: um\nX: ignored\n");
    assert_eq!(result, Ok(
            vec![
                Command::Banner("Lesson 1: uh".to_string()),
                Command::Banner("Lesson 2: um".to_string()),
                Command::Exit("ignored".to_string())
            ]
        )
    );
}
