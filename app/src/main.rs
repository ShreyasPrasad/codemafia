mod app;
mod config;
use app::App;
use yew::prelude::*;

/* Declare modules. */
mod components;
mod pages;

fn main() {
    yew::Renderer::<App>::new().render();
}
