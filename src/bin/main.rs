extern crate cursive;
extern crate walkdir;
extern crate rtypist;

use walkdir::{WalkDir};
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView};
use rtypist::parser;

fn main() {
    let mut siv = Cursive::new();

    let mut select = SelectView::new().h_align(HAlign::Center);

    let greeting = format!("The following {} lessons are available: ", lesson_count());

    let content = lesson_list();
    select.add_all_str(content);

    // Behave roughly like ncurses with key presses
    siv.add_global_callback('q', Cursive::quit);
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s| {
            s.select_up(1);
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner('j', |s| {
            s.select_down(1);
            Some(EventResult::Consumed(None))
        });

    siv.add_layer(
        Dialog::around(select.fixed_size((20, 10)))
        .title(greeting),
    );

    let lines = parser::lines();
    let tokenized = parser::tokenize(lines);
    // siv.run();
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
