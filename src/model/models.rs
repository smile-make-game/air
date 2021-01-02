/// Model definition
#[derive(Debug, Default, Clone)]
pub struct Model {
    pub character_list: CharacterListModel,
    pub quest_list: QuestListModel,
}

pub type CharacterListModel = Vec<CharacterModel>;
/// CharacterModel definition
#[derive(Debug, Clone)]
pub struct CharacterModel {
    pub id: String,

    pub name: String,
}

pub type QuestListModel = Vec<QuestModel>;
/// QuestModel definition
#[derive(Debug, Clone)]
pub struct QuestModel {
    pub id: String,

    pub title: String,
}
