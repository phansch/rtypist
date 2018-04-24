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
#[derive(Debug, PartialEq)]
pub enum ItemKind {
    Banner(String),
    GoTo(String),
    Label(String),
    Exit(String),
    Empty
}

// TODO: Make parser ignore empty lines
// TODO: Make a type out of GoTo and Label somehow
named!(banner<&str, ItemKind>,
    do_parse!(
        tag!("B:")               >>
        space0                   >>
        value: take_until_and_consume!("\n") >>
        (ItemKind::Banner(value.to_string()))
    )
);

named!(goto<&str, ItemKind>,
    do_parse!(
        tag!("G:")               >>
        space0                   >>
        value: take_until_and_consume!("\n") >>
        (ItemKind::GoTo(value.to_string()))
    )
);

named!(label<&str, ItemKind>,
    do_parse!(
        tag!("*:")               >>
        space0                   >>
        value: take_until_and_consume!("\n") >>
        (ItemKind::Label(value.to_string()))
    )
);

named!(exit<&str, ItemKind>,
    do_parse!(
        tag!("X:")               >>
        space0                   >>
        value: take_until_and_consume!("\n") >>
        (ItemKind::Exit(value.to_string()))
    )
);

// TODO: Is there some way to combine this with other parsers?
// Because currently, we parse empty lines and drop them again
named!(emptyline<&str, ItemKind>,
    do_parse!(
        space0                   >>
        value: take_until_and_consume!("\n") >>
        (ItemKind::Empty)
    )
);

named!(parse<&str, Vec<ItemKind>>,
    many0!(
        alt_complete!(
            goto |
            label |
            banner |
            exit |
            emptyline
        )
    )
);

pub fn parse_str(input: &str) -> Result<Vec<ItemKind>, String> {
    match parse(input) {
        Ok(result) => {
            let items = result.1;
            let filtered = items.into_iter().filter(|i| *i != ItemKind::Empty).collect();
            Ok(filtered)
        },
        Err(e) => Err(format!("Something went wrong! {:?}", e))
    }
}

pub fn parse_file(filename: String) -> Vec<ItemKind> {
    let f = File::open(&filename).expect(&format!("File does not exist: {}", &filename));
    let mut string = String::new();
    BufReader::new(&f).read_to_string(&mut string);
    parse_str(&string).unwrap()
}

#[test]
fn test_banner_parse() {
    let result = banner("B: Lesson 1: uh\n");
    assert_eq!(result, Ok(("", ItemKind::Banner("Lesson 1: uh".to_string()))));
}

#[test]
fn test_parse_str() {
    let result = parse_str("B: Lesson 1: uh\nB: Lesson 2: um\nX: ignored\n");
    assert_eq!(result, Ok(
            vec![
                ItemKind::Banner("Lesson 1: uh".to_string()),
                ItemKind::Banner("Lesson 2: um".to_string()),
                ItemKind::Exit("ignored".to_string())
            ]
        )
    );
}

#[test]
fn test_parse_with_labels() {
    let result = parse_str("\nG:_D_MENU\n\n*:_D_NO_MENU\nB: Lesson D1\n");
    assert_eq!(result, Ok(
            vec![
                ItemKind::GoTo("_D_MENU".to_string()),
                ItemKind::Label("_D_NO_MENU".to_string()),
                ItemKind::Banner("Lesson D1".to_string()),
            ]
        )
    );
}
