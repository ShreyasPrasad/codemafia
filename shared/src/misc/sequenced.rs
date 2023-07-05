use serde::Serialize;

/* The cache sequence is a 0 indexed event sequence number. */
pub type SequenceNum = u8;

/* Structure that supports sequence number based organization of items. */
#[derive(Debug, Clone, Serialize)]
pub struct Sequenced<T> {
    pub item: T,
    pub sequence_num: SequenceNum,
}

impl<T> Sequenced<T> {
    pub fn new(item: T, sequence_num: SequenceNum) -> Self {
        Self { item, sequence_num }
    }
}
