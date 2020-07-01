use serde::de::{Deserializer, Error, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct QueryResponse {
    pub batchcomplete: Option<String>,
    pub query: Query,
    #[serde(rename = "continue")]
    pub cont: Option<Cont>,
}

#[derive(Deserialize, Debug)]
pub struct Cont {
    pub gcmcontinue: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    pub pages: HashMap<String, Page>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Page {
    pub pageid: u64,
    pub title: String,
    pub canonicalurl: String,
    pub ns: Namespace,
    #[serde(rename = "extract")]
    pub desc: Option<String>,
    pub categories: Option<Vec<Category>>,
    pub pageprops: Option<PageProps>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    pub title: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PageProps {
    pub wikibase_item: Option<String>,
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

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
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

#[derive(Deserialize, Debug)]
pub struct PageViewResponse {
    pub items: Vec<PageViewItem>,
}

#[derive(Deserialize, Debug)]
pub struct PageViewItem {
    pub views: u64,
}

// wikidata.org API response
#[derive(Deserialize, Debug)]
pub struct WikiDataResponse {
    pub entities: HashMap<String, WikiData>,
}

#[derive(Deserialize, Debug)]
pub struct WikiData {
    pub aliases: HashMap<String, Vec<LangData>>,
    pub sitelinks: HashMap<String, SiteLinkData>,
}

#[derive(Deserialize, Debug)]
pub struct SiteLinkData {
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct LangData {
    pub value: String,
}

impl WikiDataResponse {
    pub fn get_unique_alias(&self, wikidata_id: &str) -> Option<HashSet<&str>> {
        self.entities
            .get(wikidata_id)
            .map(|wiki_data| wiki_data.get_unique_alias())
    }

    pub fn get_native_title(&self, wikidata_id: &str) -> Option<&str> {
        self.entities
            .get(wikidata_id)
            .and_then(|wiki_data| wiki_data.get_native_title())
    }
}

impl WikiData {
    pub fn get_unique_alias(&self) -> HashSet<&str> {
        let mut res = HashSet::new();
        for (_, lang) in &self.aliases {
            for lang_data in lang {
                res.insert(lang_data.value.as_str());
            }
        }
        res
    }

    pub fn get_native_title(&self) -> Option<&str> {
        self.get_title("idwiki")
    }

    // sitelink_name: enwiki for en.wikipedia.org
    // idwiki for id.wikipedia.org
    pub fn get_title(&self, sitelink_name: &str) -> Option<&str> {
        self.sitelinks
            .get(sitelink_name)
            .map(|data| data.title.as_str())
    }
}
