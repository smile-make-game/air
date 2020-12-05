use crate::model::Evolution;

#[allow(unused_variables)]
pub trait Evolute {
    fn evolute(&self, evolution: &Evolution);
}
