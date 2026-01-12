use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PlatformUser {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub display_name: String,
    pub bot: bool,
    pub status: String,
}

impl PartialEq for PlatformUser {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
