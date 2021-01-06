/// Model definition
#[derive(Debug, Default, Clone)]
pub struct Model {
    pub character_list: CharacterListModel,
    pub quest_list: QuestListModel,
    pub quest_assignment_list: Vec<QuestAssignment>,
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
    pub max_characters: u8,
}

#[derive(Debug, Clone)]
pub struct QuestAssignment {
    pub id: String,

    pub quest_id: String,
    pub character_id_list: Vec<String>,
}
