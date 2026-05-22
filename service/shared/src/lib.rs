use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlagDriveFileVisibility {
    Private,
    Public,
    Following,
    Followers,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlagDriveFile {
    pub id: u64,
    pub name: String,
    pub owner: String,
    pub visibility: FlagDriveFileVisibility,
    pub size: u64,
    pub created_at: u64,
    pub is_encrypted: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlagDriveUser {
    pub username: String,
    pub followers_count: usize,
    pub following_count: usize,
    pub is_followed: bool,
}
