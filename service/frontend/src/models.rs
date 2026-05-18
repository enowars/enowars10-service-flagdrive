#[derive(Debug, Clone, PartialEq)]
pub enum FileVisibility {
    Private,
    Public,
    OnlyFollowed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub visibility: FileVisibility,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub followers_count: usize,
    pub following_count: usize,
    pub is_followed: bool,
}
