use jumprankingsapi::models::ranking_model::Ranking;

pub fn get_ranking( year: i32, week: i32) -> Ranking {
    let url = format!("http://127.0.0.1:8000/ranking/{}/{}", year, week);
    let resp = match reqwest::blocking::get(url) {
        Ok(v) => match v.status().is_success() {
            true => {
                v.text().unwrap()
            }
            false => {
                panic!("error while getting url on year {} and week {}", year, week)
            }
        },
        Err(e) => panic!("error while getting url on year {} and week {}: {}", year, week, e)
    };
    let ranking: Ranking = serde_json::from_str(&resp).unwrap();
    ranking
}