mod app;
mod config;
use app::App;
use yew::prelude::*;

/* Declare modules. */
mod pages;
mod components;

fn main() {
    yew::Renderer::<App>::new().render();
}