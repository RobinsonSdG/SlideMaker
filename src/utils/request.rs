use std::time::Duration;

use reqwest::blocking::Response;

pub fn get_img_resp(img_url: &str) -> Response {
    let client = reqwest::blocking::Client::builder()
    .timeout(Duration::from_secs(100))
    .build().unwrap();
    match client.get(img_url).send() {
        Ok(v) => v,
        Err(e) => panic!("error requesting for image {}: {}", &img_url, e)
    }
}