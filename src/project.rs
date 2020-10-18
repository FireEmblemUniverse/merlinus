use serde::{Deserialize, Serialize};

use crate::types::Identifier;

pub mod transport;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Base {
    FE6,
    FE7,
    FE8,
    Custom(String),
}

#[derive(Serialize, Deserialize)]
struct ProjectRepr<C> {
    name: Identifier,
    base: Base,
    convoy_items: Vec<C>,
}

impl<C> ProjectRepr<C> {
    fn map<C2>(self, f: impl Fn(C) -> C2) -> ProjectRepr<C2> {
        let ProjectRepr {
            name,
            base,
            convoy_items,
        } = self;
        ProjectRepr {
            name,
            base,
            convoy_items: convoy_items.into_iter().map(f).collect(),
        }
    }

    fn add_item(&mut self, item: C) {
        self.convoy_items.push(item)
    }
}
