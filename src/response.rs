use std::collections::HashMap;
use serde::{Deserialize};

enum Namespace {
    Page,
    Category,
}

#[derive(Deserialize,Debug)]
pub struct QueryResponse {
    pub batchcomplete: Option<String>,
    pub query: Query,
}

#[derive(Deserialize,Debug)]
pub struct Query {
    pub pages: HashMap<String, Page>
}

#[derive(Deserialize,Debug,Clone)]
pub struct Page {
    pub pageid: u64,
    pub title: String,
    #[serde(rename(deserialize = "extract"))]
    pub desc: String,
}
