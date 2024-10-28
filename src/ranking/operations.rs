
use std::time::Duration;

use jumprankingsapi::models::ranking_model::Ranking;

pub fn get_ranking( year: i32, week: i32, retry: bool) -> Ranking {
    let mut url = format!("https://wsj.fly.dev/ranking/{}/{}", year, week);
    let mut json_file: &str;
    if week == 22 {
        url = format!("https://wsj.fly.dev/ranking/{}/{}", year, "22-23");
        json_file = r#"{
            "week": "24",
            "ranking": [
                {
                    "name": "One Piece",
                    "chapter": 9
                },
                {
                    "name": "Blue Box",
                    "chapter": 16
                },
                {
                    "name": "Sakamoto Days",
                    "chapter": 17
                },
                {
                    "name": "Jujutsu Kaisen",
                    "chapter": 13
                },
                {
                    "name": "My Hero Academia",
                    "chapter": 13
                },
                {
                    "name": "Akane-banashi",
                    "chapter": 14
                },
                {
                    "name": "Witch Watch",
                    "chapter": 18
                },
                {
                    "name": "Mission: Yozakura Family",
                    "chapter": 15
                },
                {
                    "name": "Kill Blue",
                    "chapter": 16
                },
                {
                    "name": "Undead Unluck",
                    "chapter": 15
                },
                {
                    "name": "Me and Roboco",
                    "chapter": 17
                },
                {
                    "name": "The Elusive Samurai",
                    "chapter": 19
                },
                {
                    "name": "Nue's Exorcist",
                    "chapter": 15
                },
                {
                    "name": "Kagurabachi",
                    "chapter": 16
                },
                {
                    "name": "Green Green Greens",
                    "chapter": 13
                },
                {
                    "name": "MamaYuyu",
                    "chapter": 15
                },
                {
                    "name": "Martial Master Asumi",
                    "chapter": 9
                },
                {
                    "name": "Cipher Academy",
                    "chapter": 8
                },
                {
                    "name": "Shadow Eliminators",
                    "chapter": 12
                },
                {
                    "name": "Two on Ice",
                    "chapter": 17
                }
            ],
            "newbies": [],
            "absent": [
                {
                    "name": "Dear Anemone",
                    "chapter": 4
                },
                {
                    "name": "Super Psychic Policeman Chojo",
                    "chapter": 4
                }
            ],
            "cover": {
                "imgs": [
                    "https://static.wikia.nocookie.net/weeky-shonen-jump/images/d/d9/WSJ_Issue_2024_20_Cover.png"
                ],
                "rank": {
                    "name": "Astro Royale",
                    "chapter": 1
                }
            },
            "color_pages": [
                {
                    "imgs": [
                        "https://static.wikia.nocookie.net/weeky-shonen-jump/images/1/17/Astro_Royale_ch001p1_Issue_20_2024.png",
                        "https://static.wikia.nocookie.net/weeky-shonen-jump/images/d/dd/Astro_Royale_ch001_Issue_20_2024.png"
                    ],
                    "rank": {
                        "name": "Astro Royale",
                        "chapter": 1
                    }
                },
                {
                    "imgs": [
                        "https://static.wikia.nocookie.net/weeky-shonen-jump/images/d/d3/Undead_Unluck_ch203_Issue_20_2024.png"
                    ],
                    "rank": {
                        "name": "Undead Unluck",
                        "chapter": 203
                    }
                },
                {
                    "imgs": [
                        "https://static.wikia.nocookie.net/weeky-shonen-jump/images/b/bb/Nue%27s_Exorcist_ch046_Issue_20_2024.png"
                    ],
                    "rank": {
                        "name": "Nue's Exorcist",
                        "chapter": 46
                    }
                }
            ],
            "preview_pages": [
                "https://static.wikia.nocookie.net/weeky-shonen-jump/images/3/3b/WSJ_Issue_2024_21_Preview.png"
            ]
        }"#;
    } else {
        json_file = r#"{
            "week": "24",
            "ranking": [
                {
                    "name": "One Piece",
                    "chapter": 24
                },
                {
                    "name": "Sakamoto Days",
                    "chapter": 41
                },
                {
                    "name": "Jujutsu Kaisen",
                    "chapter": 35
                },
                {
                    "name": "Blue Box",
                    "chapter": 37
                },
                {
                    "name": "Akane-banashi",
                    "chapter": 32
                },
                {
                    "name": "My Hero Academia",
                    "chapter": 28
                },
                {
                    "name": "Kill Blue",
                    "chapter": 19
                },
                {
                    "name": "Witch Watch",
                    "chapter": 43
                },
                {
                    "name": "Me and Roboco",
                    "chapter": 35
                },
                {
                    "name": "Undead Unluck",
                    "chapter": 37
                },
                {
                    "name": "The Elusive Samurai",
                    "chapter": 39
                },
                {
                    "name": "Mission: Yozakura Family",
                    "chapter": 35
                },
                {
                    "name": "Nue's Exorcist",
                    "chapter": 15
                },
                {
                    "name": "Mashle: Magic and Muscles",
                    "chapter": 19
                },
                {
                    "name": "Black Clover",
                    "chapter": 23
                },
                {
                    "name": "The Ichinose Family's Deadly Sins",
                    "chapter": 37
                },
                {
                    "name": "Cipher Academy",
                    "chapter": 38
                },
                {
                    "name": "Martial Master Asumi",
                    "chapter": 12
                },
                {
                    "name": "Ichigoki's Under Control!!",
                    "chapter": 12
                },
                {
                    "name": "High School Family: Kokosei Kazoku",
                    "chapter": 10
                },
                {
                    "name": "Fabricant 100",
                    "chapter": 28
                },
                {
                    "name": "Tenmaku Cinema",
                    "chapter": 14
                },
                {
                    "name": "Ginka &amp; Gl\u00fcna",
                    "chapter": 16
                },
                {
                    "name": "Ice-Head Gill",
                    "chapter": 13
                },
                {
                    "name": "Tokyo Demon Bride Story",
                    "chapter": 16
                },
                {
                    "name": "PPPPPP",
                    "chapter": 10
                },
                {
                    "name": "Do Retry",
                    "chapter": 12
                }
            ],
            "newbies": [],
            "absent": [
                {
                    "name": "Hunter \u00d7 Hunter",
                    "chapter": 0
                },
                {
                    "name": "One Piece",
                    "chapter": 0
                },
                {
                    "name": "Jujutsu Kaisen",
                    "chapter": 0
                },
                {
                    "name": "RuriDragon",
                    "chapter": 0
                }
            ],
            "cover": {
                "imgs": [
                    "https://static.wikia.nocookie.net/weeky-shonen-jump/images/d/d9/WSJ_Issue_2024_20_Cover.png"
                ],
                "rank": {
                    "name": "Astro Royale",
                    "chapter": 1
                }
            },
            "color_pages": [
                {
                    "imgs": [
                        "https://static.wikia.nocookie.net/weeky-shonen-jump/images/1/17/Astro_Royale_ch001p1_Issue_20_2024.png",
                        "https://static.wikia.nocookie.net/weeky-shonen-jump/images/d/dd/Astro_Royale_ch001_Issue_20_2024.png"
                    ],
                    "rank": {
                        "name": "Astro Royale",
                        "chapter": 1
                    }
                },
                {
                    "imgs": [
                        "https://static.wikia.nocookie.net/weeky-shonen-jump/images/d/d3/Undead_Unluck_ch203_Issue_20_2024.png"
                    ],
                    "rank": {
                        "name": "Undead Unluck",
                        "chapter": 203
                    }
                },
                {
                    "imgs": [
                        "https://static.wikia.nocookie.net/weeky-shonen-jump/images/b/bb/Nue%27s_Exorcist_ch046_Issue_20_2024.png"
                    ],
                    "rank": {
                        "name": "Nue's Exorcist",
                        "chapter": 46
                    }
                }
            ],
            "preview_pages": [
                "https://static.wikia.nocookie.net/weeky-shonen-jump/images/3/3b/WSJ_Issue_2024_21_Preview.png"
            ]
        }"#;
    }

    let ranking: Ranking = match reqwest::blocking::get(url) {
        Ok(v) => match v.status().is_success() {
            true => {
                // serde_json::from_str(v.text().unwrap().as_str()).unwrap()
                serde_json::from_str(json_file).unwrap()
            }
            false => {
                if !retry {
                    let client = reqwest::blocking::Client::new();
                    // let client = reqwest::blocking::Client::builder()
                    //     .timeout(Duration::from_secs(10))
                    //     .build()?;
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