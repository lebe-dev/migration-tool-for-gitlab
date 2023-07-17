use serde::Deserialize;

#[derive(Deserialize,Debug,Clone)]
pub struct GitLabGroup {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub visibility: String
}