// NEXT: List lessons in middle of screen
extern crate pancurses;
extern crate walkdir;

use walkdir::{WalkDir};
use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    let greeting = format!("The following {} lessons are available: ", lesson_count());

    window.printw(&greeting);

    for path in lesson_list() {
        window.printw(&path);
        window.printw(" ");
    }
    window.refresh();
    window.getch();
    endwin();
}

fn lesson_dir() -> WalkDir {
    WalkDir::new("./lessons/").min_depth(1)
}

fn lesson_count() -> usize {
    lesson_dir().into_iter().count()
}

fn lesson_list() -> Vec<String> {
    lesson_dir()
        .into_iter()
        .filter_map(|f| f.ok())
        .map(|f| f.path().file_stem().unwrap().to_str().unwrap().to_string())
        .collect()
}
