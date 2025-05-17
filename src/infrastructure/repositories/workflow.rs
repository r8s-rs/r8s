use serde::{Deserialize, Serialize};
use crate::domain::entities::Node;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Workflow {
    #[serde(default)]
    pub id: Option<i64>,
    pub pub_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_u16_keyed_map")]
    pub nodes: BTreeMap<u16, Node>,
}

use serde::de::{Deserializer, MapAccess, Visitor};
use std::fmt;

fn deserialize_u16_keyed_map<'de, D, V>(deserializer: D) -> Result<BTreeMap<u16, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    struct StringKeyMapVisitor<V> {
        marker: std::marker::PhantomData<fn() -> BTreeMap<u16, V>>,
    }

    impl<'de, V> Visitor<'de> for StringKeyMapVisitor<V>
    where
        V: Deserialize<'de>,
    {
        type Value = BTreeMap<u16, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with u16 keys represented as strings")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = BTreeMap::new();
            while let Some((key, value)) = access.next_entry::<String, V>()? {
                let parsed_key = key.parse::<u16>().map_err(serde::de::Error::custom)?;
                map.insert(parsed_key, value);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(StringKeyMapVisitor {
        marker: std::marker::PhantomData,
    })
}
