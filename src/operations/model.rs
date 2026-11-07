use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "type")]
pub enum Operation {
    Insert { position: usize, text: String },
    Delete { position: usize, length: usize },
    Retain { length: usize },
}
// pub enum Operation<'a> {
//     Insert { position: usize, text: &'a str },
//     Delete { position: usize, length: usize },
//     Retain { length: usize },
// }
