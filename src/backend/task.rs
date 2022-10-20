use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub state: String,
    pub description: String
}