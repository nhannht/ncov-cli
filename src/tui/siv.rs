use cursive::{Cursive, CursiveRunnable, With};
use cursive::event::Key::Esc;
use cursive::view::{Margins, Nameable, Resizable};
use cursive::views::{Button, Dialog, DummyView, LinearLayout, NamedView, ResizedView, ScrollView, SelectView, TextContent, TextView};
use futures::executor::block_on;
use crate::data::domain::{Continent, Country, SELECT_COUNTRIES, SELECTED_CONTINENT};
use crate::service::http::{fetch_data_one_continent, fetch_data_one_contry};

pub fn create_tui() -> CursiveRunnable {
    let mut siv = cursive::default();
    siv.add_global_callback(Esc, |siv| siv.quit());

    let init_frame = country_board_frame();

    siv.add_layer(init_frame);
    siv

}

fn create_info_view() -> ResizedView<NamedView<TextView>> {
    let help = "You can find your country or continent in left side list\n\
     Click or <Enter> to fetch Covid data about it\n
    You can press any character keyboard to quick jump, eg \'v\' to jump to countries start with v\n\
    You can also press Tab to quick move between view and buttons";
    let info_view = TextView::new(help).with_name("info_view").min_width(50);
    info_view
}

fn create_selected_continent_view() -> SelectView<&'static str> {
    let mut selected_continent_view = SelectView::new();
    for i in 0..SELECTED_CONTINENT.len() {
        selected_continent_view.add_item(SELECTED_CONTINENT[i], SELECTED_CONTINENT[i]);
    }
    selected_continent_view.set_autojump(true);
    selected_continent_view.sort();
    selected_continent_view.set_on_submit(|siv, item: &str| {
        siv.call_on_name("info_view", |view: &mut TextView| {
            let f = fetch_data_one_continent(item);
            let continent: Continent = block_on(f);
            let death_and_recovered = ((continent.deaths / continent.recovered) * 100.0);
            let text = format!("
Today case : {today}\n
Today death: {today_death}\n
Today recover: {today_recover}\n
All case: {all_case}\n
All death: {all_death}\n
All recover: {all_recover}\n
Death/Recovered : {death_recovered} %\n
                        ", today = continent.todayCases.to_string()
                               , today_death = continent.todayDeaths.to_string()
                               , today_recover = continent.todayRecovered.to_string()
                               , all_case = continent.cases.to_string()
                               , all_death = continent.deaths.to_string()
                               , all_recover = continent.recovered.to_string()
                               , death_recovered = death_and_recovered.to_string()
            );
            view.set_content(text);
        });
    });
    selected_continent_view
}

fn create_selected_country_view() -> SelectView<&'static str> {
    let mut selected_country_view = SelectView::new();
    for index in 0..SELECT_COUNTRIES.len() {
        selected_country_view.add_item(SELECT_COUNTRIES[index], SELECT_COUNTRIES[index]);
    };

    selected_country_view.set_autojump(true);
    selected_country_view.sort();

    selected_country_view.set_on_submit(|siv,item|{
        siv.call_on_name("info_view",|view:&mut TextView|{
            let f = fetch_data_one_contry(item);
            let country:Country = block_on(f);
            let death_and_recovered = ((country.deaths / country.recovered) * 100.0);
            let text = format!("
Today case : {today}\n
Today death: {today_death}\n
Today recover: {today_recover}\n
All case: {all_case}\n
All death: {all_death}\n
All recover: {all_recover}\n
Death/Recovered : {death_recovered} %\n
                        ", today = country.todayCases.to_string()
                               , today_death = country.todayDeaths.to_string()
                               , today_recover = country.todayRecovered.to_string()
                               , all_case = country.cases.to_string()
                               , all_death = country.deaths.to_string()
                               , all_recover = country.recovered.to_string()
                               , death_recovered = death_and_recovered.to_string()
            );
            view.set_content(text);
        })
    });

    selected_country_view
}


fn country_board_frame() -> Dialog {
    let left_side = create_selected_country_view();
    let right_side = create_info_view();
    let board = LinearLayout::horizontal()
        .child(ScrollView::new(left_side))
        .child(ScrollView::new(right_side));
    let mut board_wrapper = Dialog::around(board).title("Search by country")
        .button("Switch to continent",|s|{
            s.pop_layer();
            let view = continent_board_frame();
            s.add_layer(view);
        })
        .button("Quit",Cursive::quit);

    board_wrapper.set_padding(Margins::lrtb(2,2,2,2));
    board_wrapper

}

fn continent_board_frame() -> Dialog {
    let left_side = create_selected_continent_view();
    let right_side = create_info_view();
    let board = LinearLayout::horizontal().child(ScrollView::new(left_side))
        .child(ScrollView::new(right_side));


    let mut board_wrapper = Dialog::around(board).title("Search by continent")
        .button("Switch to continent",|s|{
            s.pop_layer();
            let view = country_board_frame();
            s.add_layer(view);
        })
        .button("Quit",Cursive::quit)
        ;
    board_wrapper.set_padding(Margins::lrtb(2,2,2,2));


    board_wrapper
}
