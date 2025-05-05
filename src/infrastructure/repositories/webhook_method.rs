use std::str::FromStr;

#[derive(Debug, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "webhook_method")]
#[sqlx(rename_all = "lowercase")]
pub enum WebhookMethod {
    Get,
    Post,
    Delete,
    Put,
    Patch,
    Head,
}

impl ToString for WebhookMethod {
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

impl FromStr for WebhookMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(WebhookMethod::Get),
            "post" => Ok(WebhookMethod::Post),
            "delete" => Ok(WebhookMethod::Delete),
            "put" => Ok(WebhookMethod::Put),
            "patch" => Ok(WebhookMethod::Patch),
            "head" => Ok(WebhookMethod::Head),
            _ => Err(()),
        }
    }
}