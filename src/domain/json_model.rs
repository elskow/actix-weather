use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct JsonResponse {
    pub status: String,
    pub message: String
}