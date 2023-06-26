use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub code: String,
}

#[function_component]
pub fn Room(props: &Props) -> Html {
    html! { "Hello world" }
}
