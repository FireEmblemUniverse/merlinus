
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

mod serializers;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct Identifier(String);
impl Identifier {
    pub fn to_string(&self) -> &String {
        let Identifier(s) = self;
        return s;
    }
}

// Types representing Merlinus-defined types

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum MType {
    Byte,
    Short,
    Word,
    #[serde(rename = "String")]
    MString,
    Custom(Identifier),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    #[serde(rename = "type")]
    type_: MType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Meta {
    Product(HashMap<Identifier, Entry>),
    Coproduct(HashMap<Identifier, Entry>),
    Many(MType),
    Alias(MType),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Custom {
    name: Identifier,
    contents: Meta,
}
