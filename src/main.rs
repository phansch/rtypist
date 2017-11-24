// NEXT: List lessons in middle of screen
extern crate pancurses;

use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;
use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    let greeting = format!("The following {} lessons are available:", lesson_count());

    window.printw(&greeting);

    for path in lesson_list() {
        window.printw(&path.into_os_string().into_string().unwrap());
    }
    window.refresh();
    window.getch();
    endwin();
}

fn lesson_count() -> usize {
    // TODO: Handle errors correctly (files missing etc)
    read_dir("lessons/").unwrap().count()
}

// TODO:
// Not sure if this is a good way to handle errors and still adhere to the function return type?
// Are there better ways to do this?
// Is walkdir overkill?
fn lesson_list() -> Vec<std::path::PathBuf> {
    let paths: Vec<std::path::PathBuf> = match read_dir("lessons/") {
        Err(error) => {
            println!("! {:?}", error.kind());
            vec![PathBuf::new()]
        },
        Ok(paths) => paths.map(|path| path.unwrap().path()).collect(),
    };
    paths
}
