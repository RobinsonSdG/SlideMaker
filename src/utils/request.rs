use reqwest::blocking::Response;

pub fn get_img_resp(img_url: &str) -> Response {
    match reqwest::blocking::get(img_url) {
        Ok(v) => v,
        Err(e) => panic!("error requesting for image {}: {}", &img_url, e)
    }
}