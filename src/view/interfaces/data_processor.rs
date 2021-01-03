pub use crate::model::FromRepositoryMessageItem;
pub use anyhow::Result;

pub trait DataProcessor {
    fn process_data(&self, msg: &FromRepositoryMessageItem) -> Result<()>;
}
