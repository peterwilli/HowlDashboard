#[derive(Hash, Eq, PartialEq, Debug)]
pub struct DataBlockKey {
    pub category: String,
    pub title: String,
    pub from: String
}