// Mini gtypist script language reference:
// command_char : command_data
//
// 1. Blank lines and lines starting with '#', are ignored
// - B: Clears the whole screen and shows the command_data on top of the screen
//      Maximum one line
// - X: Exit gtypist
pub enum Command {
    Banner(String),
}

pub mod parser {
    use std::fs::File;

    use std::io::BufReader;
    use std::io::BufRead;

    pub fn read() -> Vec<String> {
        let filename = "./lessons/q.typ";

        let f = File::open(filename).expect("File does not exist");
        BufReader::new(&f).lines().map(|l| l.unwrap()).collect()
    }

    /// Remove empty lines and comments from vector
    pub fn cleanup(lines: Vec<String>) -> Vec<String> {
        lines.into_iter().filter_map(|l| {
            match l.chars().next() {
                Some('#') => None,
                Some(_) => Some(l),
                None => None
            }
        }).collect()
    }

    /// Returns a tokenized version of the given collection.
    pub fn tokenize(lines: Vec<String>) {

        // TODO: Figure out a way to test this
        for line in lines {
            // writeln!(writer, "{}", line).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleanup_it_filters_out_empty_lines_and_comments() {
        let lines = vec![String::from("# a"), String::from(""), String::from("abc")];
        let empty_vec: Vec<String> = vec![String::from("abc")];
        assert_eq!(parser::cleanup(lines), empty_vec)
    }
}
