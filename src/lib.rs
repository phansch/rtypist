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

    use std::io;
    use std::io::BufReader;
    use std::io::BufRead;

    pub fn lines() -> Vec<String> {
        let filename = "./lessons/q.typ";

        let f = File::open(filename).expect("File does not exist");
        let file = BufReader::new(&f);
        file.lines().map(|l| l.unwrap()).collect()
    }

    pub fn cleanup() {
        // Remove empty lines and comments from vector
    }

    /// Returns a tokenized version of the given collection.
    pub fn tokenize(lines: Vec<String>) {

        // TODO: Figure out a way to test this
        for line in lines {
            // writeln!(writer, "{}", line).unwrap();
        }
    }
}
