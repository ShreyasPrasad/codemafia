use shared::{events::EventContent, misc::sequenced::Sequenced};

use crate::misc::events::{Event, Recipient};

/* This is a trait to support both sequenced events and normal events as input to the dispatch_event function. */
pub trait DispatcherInfo {
    fn recipient(&self) -> Recipient;
    fn event_content(&self) -> EventContent;
}

/* Implementation for a normal event. */
impl DispatcherInfo for Event {
    fn recipient(&self) -> Recipient {
        self.recipient.clone()
    }

    fn event_content(&self) -> EventContent {
        self.content.clone()
    }
}

/* Implementation for a sequenced event. */
impl DispatcherInfo for Sequenced<Event> {
    fn recipient(&self) -> Recipient {
        self.item.recipient.clone()
    }

    fn event_content(&self) -> EventContent {
        self.item.content.clone()
    }
}
