#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Array {}

impl Array {
    pub fn new() -> Array {
        Array {}
    }
}
