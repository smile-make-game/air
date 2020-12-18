use super::evolution::*;
use anyhow::Result;

pub struct DataModel {
    pub events: Vec<EventModel>,
    pub characters: Vec<CharacterModel>,
}

impl Default for DataModel {
    fn default() -> Self {
        DataModel {
            events: vec![],
            characters: vec![],
        }
    }
}

impl DataModel {
    pub async fn evolute<'m>(
        &mut self,
        patch: Option<Evolution<'m>>,
    ) -> Result<Option<Evolution<'m>>> {
        Ok(patch)
    }

    pub async fn load(&mut self) -> Result<()> {
        // TODO: use real data
        self.events = vec![
            EventModel {
                subject: "an event".to_string(),
                content: "content".to_string(),
            },
            EventModel {
                subject: "another event".to_string(),
                content: "content".to_string(),
            },
            EventModel {
                subject: "init event".to_string(),
                content: "content".to_string(),
            },
            EventModel {
                subject: "find a chest".to_string(),
                content: "content".to_string(),
            },
        ];
        self.characters = vec![
            CharacterModel {
                name: "Vactar".to_owned()
            }
        ];
        Ok(())
    }

    pub fn get_evolution(&self) -> Evolution {
        // TODO: implement
        Evolution { new_data: self }
    }
}

#[derive(Debug)]
pub struct EventModel {
    pub subject: String,
    pub content: String,
}

#[derive(Debug)]
pub struct CharacterModel {
    pub name: String
}
