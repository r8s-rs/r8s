use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, sqlx::Type, PartialEq, Eq, Deserialize, Serialize)]
#[sqlx(type_name = "webhook_method")]
#[sqlx(rename_all = "lowercase")]
pub enum HttpMethod {
    #[serde(alias = "GET")]
    Get,
    #[serde(alias = "POST")]
    Post,
    #[serde(alias = "DELETE")]
    Delete,
    #[serde(alias = "PUT")]
    Put,
    #[serde(alias = "PATCH")]
    Patch,
    #[serde(alias = "HEAD")]
    Head,
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match &self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Delete => "delete",
            Self::Put => "put",
            Self::Patch => "patch",
            Self::Head => "head",
        }.to_string()
    }
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            "delete" => Ok(Self::Delete),
            "put" => Ok(Self::Put),
            "patch" => Ok(Self::Patch),
            "head" => Ok(Self::Head),
            _ => Err(()),
        }
    }
}