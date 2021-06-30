use std::{collections::HashMap, marker::PhantomData, sync::Arc};

pub enum ValueT {}
pub enum FileT {}

// TODO
pub enum Schema {}
pub enum Value {}
pub enum SourceTree {}
pub trait Rule {}

pub enum TokenKind {
    Value(Schema),
    File,
}

pub struct Token<T> {
    kind: TokenKind,
    key: String,
    phantom: PhantomData<T>,
}

pub enum Entry {
    Value(Value),
    File,
}

pub struct Store {
    source_tree: SourceTree,
    entries: HashMap<String, Entry>,
}
