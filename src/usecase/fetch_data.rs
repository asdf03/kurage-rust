use crate::domain::models::{JsonDomain, JsonId};
use serde_json;

pub fn convert_domain(json_str: String) -> Result<JsonDomain, serde_json::Error> {
    let json_id: JsonId = serde_json::from_str(&json_str)?;
    Ok(JsonDomain::new(json_id.id))
}