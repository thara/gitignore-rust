extern crate reqwest;

// See https://www.gitignore.io/docs
const GITIGNORE_BASEURL: &str = "https://www.gitignore.io/api";
const GITIGNORE_LIST_URL: &str = "https://www.gitignore.io/api/list";

pub fn list() -> reqwest::Result<reqwest::blocking::Response> {
    reqwest::blocking::get(GITIGNORE_LIST_URL)
}

pub fn gen_url(search_items: &Vec<String>) -> String {
    let base_url = GITIGNORE_BASEURL.to_owned();
    let search_strs = search_items.join(",");
    base_url + "/" + &search_strs
}

pub fn search(search_items: &Vec<String>) -> reqwest::Result<reqwest::blocking::Response> {
    let url = gen_url(search_items);
    reqwest::blocking::get(&url)
}
