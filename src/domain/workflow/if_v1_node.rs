use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IfV1Node {
    pub condition: String
}
