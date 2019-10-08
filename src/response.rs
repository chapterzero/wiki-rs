use std::collections::HashMap;
use serde::Deserialize;
use std::fmt;
use serde::de::{Error, Deserializer, Visitor};

#[derive(Deserialize,Debug)]
pub struct QueryResponse {
    pub batchcomplete: Option<String>,
    pub query: Query,
    #[serde(rename = "continue")]
    pub cont: Option<Cont>
}

#[derive(Deserialize,Debug)]
pub struct Cont {
    pub gcmcontinue: Option<String>,
}

#[derive(Deserialize,Debug)]
pub struct Query {
    pub pages: HashMap<String, Page>
}

#[derive(Deserialize,Debug,Clone)]
pub struct Page {
    pub pageid: u64,
    pub title: String,
    pub canonicalurl: String,
    pub ns: Namespace,
    #[serde(rename = "extract")]
    pub desc: Option<String>,
    pub categories: Option<Vec<Category>>,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Category {
    pub title: String
}

#[derive(Debug, Clone)]
pub enum Namespace {
    Page,
    Category,
    NotImplemented,
}

impl<'de> Deserialize<'de> for Namespace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NamespaceVisitor;
        impl<'de> Visitor<'de> for NamespaceVisitor {
            type Value = Namespace;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Enum namespace")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where
                E: Error, 
            {
                match v {
                    0 => Ok(Namespace::Page),
                    14 => Ok(Namespace::Category),
                    _ => Ok(Namespace::NotImplemented),
                }
            }
        }
        deserializer.deserialize_u64(NamespaceVisitor)
    }
}
