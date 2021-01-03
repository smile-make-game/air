use crate::model::FromRepositoryMessageItem;

pub trait DataProcessor {
    fn process_data(&self, msg: &FromRepositoryMessageItem);
}
