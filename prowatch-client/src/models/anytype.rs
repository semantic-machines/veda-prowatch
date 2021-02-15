#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnyType {}

impl AnyType {
    pub fn new() -> AnyType {
        AnyType {}
    }
}
