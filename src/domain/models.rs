use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JsonId {
    pub id: String,
}

pub struct JsonDomain {
    pub value: String,
}

impl JsonDomain {
    pub fn new(text: String) -> Self {
        JsonDomain { value: text }
    }
}
