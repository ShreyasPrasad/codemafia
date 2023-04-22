use yew::prelude::*;
use patternfly_yew::*;

use crate::components::layoutitem::LayoutItem;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <Stack>
            <StackItem>
                <LayoutItem>
                    <h1 class="landingtitle">{"codemafia"}</h1>
                </LayoutItem>
            </StackItem>
            <StackItem>
                <LayoutItem>
                    <Button variant={ButtonVariant::Primary}>{ "Create Game" }</Button>{" "}
                </LayoutItem>
            </StackItem>
        </Stack>
    }
}