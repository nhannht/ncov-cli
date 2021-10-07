use cursive::{Cursive, CursiveExt, CursiveRunnable};
use cursive::event::Key::Esc;
use cursive::view::Nameable;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, ScrollView, SelectView, TextContent, TextView};

use crate::data::domain::{SELECT_COUNTRIES, SELECTED_CONTINENT};

mod service;
mod data;
mod tui;

#[tokio::main]
async fn main() {
    tui::siv::create_tui().run();

}

