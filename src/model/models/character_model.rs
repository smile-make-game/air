use super::Model;
use uuid::Uuid;

#[derive(Debug)]
pub struct CharacterModel {
    id: String,

    pub name: String,
}

impl Model for CharacterModel {
    fn get_id(&self) -> String {
        self.id.to_owned()
    }
}

pub struct CharacterModelBuilder {
    id: String,
    name: String,
}

impl CharacterModelBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_owned(),
        }
    }

    pub fn build(&self) -> CharacterModel {
        CharacterModel {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
        }
    }
}
