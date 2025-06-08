use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, sqlx::Type, PartialEq, Eq, Deserialize, Serialize)]
#[sqlx(type_name = "execution_status")]
#[sqlx(rename_all = "lowercase")]
pub enum ExecutionStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "waiting")]
    Waiting
}

impl ToString for ExecutionStatus {
    fn to_string(&self) -> String {
        match &self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Canceled => "canceled",
            Self::Success => "success",
            Self::Error => "error",
            Self::Waiting => "waiting",
        }.to_string()
    }
}

impl FromStr for ExecutionStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "canceled" => Ok(Self::Canceled),
            "success" => Ok(Self::Success),
            "error" => Ok(Self::Error),
            "waiting" => Ok(Self::Waiting),
            _ => Err(()),
        }
    }
}