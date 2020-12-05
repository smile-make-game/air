use super::data_model::*;

pub struct Evolution<'e> {
    pub new_data: &'e DataModel
}

// pub enum Operator {
//     Insert(i64),
//     Update(i64),
//     Remove(i64),
//     Refresh,
// }
