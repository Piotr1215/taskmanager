use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Domain {{{
// Using #[derive(Debug, Clone)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: String,
}
impl Task {
    pub fn new(description: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description: description.to_string(),
            status: "pending".to_string(),
        }
    }
    pub fn done(self) -> Task {
        Task {
            id: self.id,
            description: self.description,
            status: "done".to_string(),
        }
    }
}
