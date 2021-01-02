use super::models::Model;

#[derive(Debug, Clone)]
pub enum RepositoryMessage {
    ToRepository(ToRepositoryMessage),
    FromRepository(FromRepositoryMessage),
}

#[derive(Debug, Clone)]
pub struct ToRepositoryMessage {}

pub type FromRepositoryMessage = Vec<FromRepositoryMessageItem>;

#[derive(Debug, Clone)]
pub enum FromRepositoryMessageItem {
    Insert(Model),
    Update(Model),
    Remove(Model),
}
