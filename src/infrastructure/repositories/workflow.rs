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
    #[serde(deserialize_with = "deserialize_u64_keyed_map")]
    pub nodes: BTreeMap<u64, Node>,
}

use serde::de::{Deserializer, MapAccess, Visitor};
use std::fmt;

fn deserialize_u64_keyed_map<'de, D, V>(deserializer: D) -> Result<BTreeMap<u64, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    struct StringKeyMapVisitor<V> {
        marker: std::marker::PhantomData<fn() -> BTreeMap<u64, V>>,
    }

    impl<'de, V> Visitor<'de> for StringKeyMapVisitor<V>
    where
        V: Deserialize<'de>,
    {
        type Value = BTreeMap<u64, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map with u64 keys represented as strings")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = BTreeMap::new();
            while let Some((key, value)) = access.next_entry::<String, V>()? {
                let parsed_key = key.parse::<u64>().map_err(serde::de::Error::custom)?;
                map.insert(parsed_key, value);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(StringKeyMapVisitor {
        marker: std::marker::PhantomData,
    })
}