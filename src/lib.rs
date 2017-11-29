// Mini gtypist script language reference:
// command_char : command_data
//
// 1. Blank lines and lines starting with '#', are ignored
// - B: Clears the whole screen and shows the command_data on top of the screen
//      Maximum one line
// - X: Exit gtypist
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command {
    Banner(String),
}

pub mod parser {
    use std::fs::File;

    use std::io::BufReader;
    use std::io::BufRead;
    use Command;

    pub fn parse(filename: &str) -> Vec<Command> {
        let lines = read(filename);
        tokenize(lines)
    }

    /// Turn file lines into collection of commands
    pub fn tokenize(lines: Vec<String>) -> Vec<Command> {
        lines.into_iter().filter_map(|l| {
            match l.chars().next() {
                Some('#') => None,
                Some('B') => Some(Command::Banner(command_from_line(l))),
                Some(_) => None,
                None => None
            }
        }).collect()
    }

    /// Returns a tokenized version of the given collection.
    fn command_from_line(line: String) -> String {
        line.split(':').nth(1).unwrap().trim().to_string()
    }

    fn read(filename: &str) -> Vec<String> {
        let f = File::open(filename).expect("File does not exist");
        BufReader::new(&f).lines().map(|l| l.unwrap()).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_it_filters_out_empty_lines_and_comments() {
            let lines = vec![String::from("# a"), String::from(""), String::from("B : test")];
            let banner_vec = vec![Command::Banner(String::from("test"))];
            assert_eq!(tokenize(lines), banner_vec)
        }
    }
}
