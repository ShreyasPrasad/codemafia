use yew::prelude::*;
use patternfly_yew::*;

use crate::components::layoutitem::LayoutItem;

use codemafia::events::game::RoomCode;

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

pub fn get_new_room_code() -> RoomCode {
    todo!()
}

