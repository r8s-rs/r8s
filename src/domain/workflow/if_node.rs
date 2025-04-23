use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IfNode {
    pub condition: String
}
