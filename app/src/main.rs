mod app;

use app::App;

use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

use pages::landing::Landing;

use crate::pages::room::Room;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/room/:code")]
    Room { code: String },
    #[at("/")]
    Landing,
    #[not_found]
    #[at("/404")]
    LandingDefault
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Room { code } => {
            html! { <Room/> }
        }
        _ => {
            html! { <Landing /> }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}