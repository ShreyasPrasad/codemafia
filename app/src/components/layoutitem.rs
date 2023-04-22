use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(LayoutItem)]
pub fn layout_item(props: &Props) -> Html {
    html! {
        <div style="padding: 1rem; height: 100%;">
            { for props.children.iter() }
        </div>
    }
}