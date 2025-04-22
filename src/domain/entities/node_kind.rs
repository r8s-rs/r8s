use serde::Deserialize;
use crate::domain::workflow::{
    ManualTriggerNode,
    SetNode,
    IfNode,
    SwitchNode,
};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum NodeKind {
    #[serde(rename = "manualTrigger")]
    Manual(ManualTriggerNode),

    #[serde(rename = "set")]
    Set(SetNode),

    #[serde(rename = "if")]
    If(IfNode),

    #[serde(rename = "switch")]
    Switch(SwitchNode),

    #[serde(other)]
    Unknown,
}