extern crate pancurses;

use std::fs::read_dir;
use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    let greeting = format!("The following {} lessons are available:", lesson_count());
    window.printw(&greeting);
    window.refresh();
    window.getch();
    endwin();
}

fn lesson_count() -> usize {
    // TODO: Handle errors correctly (files missing etc)
    read_dir("lessons/").unwrap().count()
}
