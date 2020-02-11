use super::response::{QueryResponse, Page};

// PageId could be u64 / &str, they only differ in parameter name and
// how to extract page from result
// NOTE when using &str, there is no guarantee page returned is correct
// because in response, page is indexed with pageid
pub trait PageId: std::fmt::Display + std::fmt::Debug {
    fn get_param_name(&self) -> &'static str;
    fn get_cat_param_name(&self) -> &'static str;
    fn get_page_from_response(&self, resp: &QueryResponse) -> Option<Page>;
}

impl PageId for u64 {
    fn get_param_name(&self) -> &'static str {
        "pageids"
    }

    fn get_cat_param_name(&self) -> &'static str {
        "gcmpageid"
    }

    fn get_page_from_response(&self, resp: &QueryResponse) -> Option<Page> {
        resp.query.pages.get(&self.to_string()).map(|p| p.clone())
    }
}

impl PageId for &str {
    fn get_param_name(&self) -> &'static str {
        "titles"
    }

    fn get_cat_param_name(&self) -> &'static str {
        "gcmtitle"
    }

    fn get_page_from_response(&self, resp: &QueryResponse) -> Option<Page> {
        resp.query.pages.values().next().map(|p| p.clone())
    }
}

impl PageId for String {
    fn get_param_name(&self) -> &'static str {
        "titles"
    }

    fn get_cat_param_name(&self) -> &'static str {
        "gcmtitle"
    }

    fn get_page_from_response(&self, resp: &QueryResponse) -> Option<Page> {
        resp.query.pages.values().next().map(|p| p.clone())
    }
}
