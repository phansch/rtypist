extern crate cursive;
extern crate walkdir;
extern crate rtypist;

use std::io;
use std::io::Write;
use std::io::BufWriter;
use walkdir::{WalkDir};
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::Printer;
use cursive::views::{Canvas, Dialog, OnEventView, SelectView};
use cursive::view::Boxable;
use cursive::theme::ColorStyle;
use rtypist::parser;
use rtypist::Command;
use std::process;

fn main() {
    let mut siv = Cursive::new();

    let mut select = SelectView::new().h_align(HAlign::Center);

    let greeting = format!("The following {} lessons are available: ", lesson_count());

    let content = lesson_list();
    select.add_all_str(content);
    select.set_on_submit(start_lesson);

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


    // let mut writer = BufWriter::new(io::stdout());
    // for command in parsed {
    //     writeln!(writer, "{:?}", command).unwrap();
    // }
    siv.run();
}

fn start_lesson(siv: &mut Cursive, lesson: &str) {
    siv.pop_layer();
    let lesson_path = format!("./lessons/{}.typ", lesson);
    let mut commands = parser::parse(lesson_path).into_iter();

    while let Some(command) = commands.next() {
        match command {
            Command::Banner(text) => {
                siv.add_fullscreen_layer(
                    BannerView::new(text).full_width()
                )
            }
            _ => process::exit(1)
        }
    }
}

struct BannerView {
    text: String
}

impl BannerView {
    fn new(text: String) -> Self {
        BannerView {
            text: text
        }
    }
}

impl View for BannerView {
    fn draw(&self, printer: &Printer) {
        printer.print((0, 0), &self.text);
    }
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
