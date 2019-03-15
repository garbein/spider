extern crate env_logger;
extern crate log;
extern crate reqwest;

use reqwest::Response;

fn main() {
    env_logger::init();
    println!("Hello, world!");
    search();
}

fn run() {

}

fn search() {
    let name = "大华集团有限公司一分公司";
    let url = format!("https://m.51job.com/search/joblist.php?keyword={}&keywordtype=2", name);
    println!("{}",url);
    let mut response = http_get(url.as_ref());
    println!("{:?}", response);
    println!("{:?}", response.text());
}

fn http_get(url: &str) -> Response {
    reqwest::get(url).unwrap()
}
