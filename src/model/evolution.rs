use super::data_model::*;

pub struct Evolution<'e> {
    pub new_data: &'e DataModel,
    pub method: Method
}

pub struct EvolutionRequest {
    pub method: Method
}

pub enum Method {
    Insert(i64),
    Update(i64),
    Remove(i64),
    Refresh,
}
