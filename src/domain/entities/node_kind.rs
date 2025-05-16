use serde::{Deserialize, Serialize};
use crate::domain::workflow::{
    ManualTriggerV1Node,
    WebhookV1Node,
    SetV1Node,
    IfV1Node,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum NodeKind {
    #[serde(rename = "ManualTriggerV1")]
    ManualTriggerV1(ManualTriggerV1Node),

    #[serde(rename = "SetV1")]
    SetV1(SetV1Node),

    #[serde(rename = "IfV1")]
    IfV1(IfV1Node),

    #[serde(rename = "WebhookV1")]
    WebhookV1(WebhookV1Node),

    #[serde(other)]
    Unknown,
}