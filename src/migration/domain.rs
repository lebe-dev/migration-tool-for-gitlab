use serde::Deserialize;

#[derive(Deserialize,Debug,Clone)]
pub struct GitLabGroup {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub visibility: String
}

#[derive(Deserialize,Debug,Clone)]
pub struct GitLabProject {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub visibility: String,

    pub namespace: GitLabNamespace
}

#[derive(Deserialize,Debug,Clone)]
pub struct GitLabNamespace {
    pub id: u32,
    pub name: String,
    pub path: String,
}