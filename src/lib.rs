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

pub mod parser {
    use std::fs::File;

    use std::io::BufReader;
    use std::io::BufRead;
    use Command;

    pub fn parse(filename: String) -> Vec<Command> {
        let lines = read(filename);
        tokenize(lines)
    }

    /// Turn file lines into collection of commands
    pub fn tokenize(lines: Vec<String>) -> Vec<Command> {
        lines.into_iter().filter_map(|l| {
            match l.chars().next() {
                Some('B') => Some(Command::Banner(command_from_line(l))),
                Some('X') => Some(Command::Exit(command_from_line(l))),
                Some('#') | Some(_) | None => None,
            }
        }).collect()
    }

    /// Returns a tokenized version of the given collection.
    fn command_from_line(line: String) -> String {
        line.split(':').nth(1).unwrap().trim().to_string()
    }

    fn read(filename: String) -> Vec<String> {
        let f = File::open(filename).expect("File does not exist");
        BufReader::new(&f).lines().map(|l| l.unwrap()).collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_it_filters_out_empty_lines_and_comments() {
            let lines = vec![String::from("# a"), String::from("")];
            let expected_vec = vec![];
            assert_eq!(tokenize(lines), expected_vec)
        }

        #[test]
        fn test_it_parses_command_banner() {
            let lines = vec![String::from("B : test")];
            let expected_vec = vec![Command::Banner(String::from("test"))];
            assert_eq!(tokenize(lines), expected_vec)
        }

        #[test]
        fn test_it_parses_command_exit() {
            let lines = vec![String::from("# a"), String::from(""), String::from("X : ignored")];
            let expected_vec = vec![Command::Exit(String::from("ignored"))];
            assert_eq!(tokenize(lines), expected_vec)
        }
    }
}
