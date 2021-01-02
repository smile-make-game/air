pub struct Model {
    character_list: CharacterListModel,
    quest_list: QuestListModel
}

pub type CharacterListModel = Vec<CharacterModel>;
pub struct CharacterModel {
    id: String,

    name: String,
}

pub type QuestListModel = Vec<QuestModel>;
pub struct QuestModel {
    id: String,

    title: String,
}
