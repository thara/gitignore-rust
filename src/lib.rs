extern crate reqwest;

// See https://www.gitignore.io/docs
const GITIGNORE_BASEURL: &str = "https://www.gitignore.io/api";
const GITIGNORE_LIST_URL: &str = "https://www.gitignore.io/api/list";

pub fn list() -> reqwest::Result<reqwest::Response> {
    reqwest::get(GITIGNORE_LIST_URL)
}

pub fn list_by(client: reqwest::Client) -> reqwest::Result<reqwest::Response> {
    client.get(GITIGNORE_LIST_URL).send()
}

pub fn gen_url(search_items: &Vec<String>) -> String {
    let base_url = GITIGNORE_BASEURL.to_owned();
    let search_strs = search_items.join(",");
    base_url + "/" + &search_strs
}

pub fn search(search_items: &Vec<String>) -> reqwest::Result<reqwest::Response> {
    let url = gen_url(search_items);
    reqwest::get(&url)
}

pub fn search_by(client: reqwest::Client,
                 search_items: &Vec<String>)
                 -> reqwest::Result<reqwest::Response> {
    let url = gen_url(search_items);
    client.get(&url).send()
}
