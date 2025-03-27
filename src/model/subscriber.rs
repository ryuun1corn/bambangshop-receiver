use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SubcriberRequest {
    pub url: String,
    pub name: String,
}
