use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub enum RepositoryMessage {
    ToRepository(ToRepositoryMessage),
    FromRepository(FromRepositoryMessage),
}

#[derive(Debug)]
pub struct ToRepositoryMessage {}

pub type FromRepositoryMessage = Vec<FromRepositoryMessageItem>;

#[derive(Debug)]
pub enum FromRepositoryMessageItem {
    Insert,
    Update,
    Remove,
}

pub struct PartialModel {
}
