use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{landing::Landing, room::Room};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/room/:code")]
    Room { code: String },
    #[at("/")]
    Landing,
    #[not_found]
    #[at("/404")]
    LandingDefault,
}

fn switch_app(routes: Route) -> Html {
    match routes {
        Route::Room { code } => {
            html! { <Room code={code}/> }
        }
        _ => {
            html! { <Landing /> }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch_app} />
        </BrowserRouter>
    }
}
