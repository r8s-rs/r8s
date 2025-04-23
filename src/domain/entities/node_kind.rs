use serde::Deserialize;
use crate::domain::workflow::{
    ManualTriggerV1Node,
    SetV1Node,
    IfV1Node,
    SwitchV1Node,
};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum NodeKind {
    #[serde(rename = "ManualTriggerV1")]
    ManualTriggerV1(ManualTriggerV1Node),

    #[serde(rename = "SetV1")]
    SetV1(SetV1Node),

    #[serde(rename = "IfV1")]
    IfV1(IfV1Node),

    #[serde(rename = "SwitchV1")]
    SwitchV1(SwitchV1Node),

    #[serde(other)]
    Unknown,
}