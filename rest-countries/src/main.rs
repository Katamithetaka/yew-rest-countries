use yew::prelude::*;
mod app;
mod routes;
mod components;
mod models;
use app::App;

use log::Level;
fn main() {
    console_log::init_with_level(Level::Debug).unwrap();

    yew::Renderer::<App>::new().render();
}
