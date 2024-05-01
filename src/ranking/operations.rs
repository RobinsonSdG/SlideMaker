
use std::time::Duration;

use jumprankingsapi::models::ranking_model::Ranking;

pub fn get_ranking( year: i32, week: i32, retry: bool) -> Ranking {
    let mut url = format!("https://wsj.fly.dev/ranking/{}/{}", year, week);
    if week == 22 {
        url = format!("https://wsj.fly.dev/ranking/{}/{}", year, "22-23");
    }
    let ranking: Ranking = match reqwest::blocking::get(url) {
        Ok(v) => match v.status().is_success() {
            true => {
                serde_json::from_str(v.text().unwrap().as_str()).unwrap()
            }
            false => {
                if !retry {
                    let client = reqwest::blocking::Client::new();
                    let _ = match client.post(format!("https://wsj.fly.dev/rankings/{}", year)).timeout(Duration::from_secs(120)).send() {
                        Ok(v) => match v.status().is_success() {
                            true => {
                                return get_ranking( year, week, true)
                            }
                            false => {
                                panic!("error while getting url on year {} and week {}", year, week)
                            }
                        },
                        Err(e) => panic!("error while getting url on year {} and week {}: {}", year, week, e)
                    };
                } else {
                    panic!("error while getting url on year {} and week {}", year, week)
                }
            }
        },
        Err(e) => {
            panic!("error while getting url on year {} and week {}: {}", year, week, e)
        }
    };
    ranking
}