mod ranking;
use jumprankingsapi::models::ranking_model::{Rank, Figure};
use ranking::operations;
mod utils;
use utils::file_operations;
use utils::request;
mod pptx;
use pptx::update;
use pptx::create;

use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut src = "SlidePrintempsV6";
    let year = 2024;
    let week = 47;
    
    let ranking = operations::get_ranking(year, week, false);
    let previous_ranking = operations::get_ranking(year, week-1, false);

    file_operations::copy_directory(Path::new(src), Path::new("Tmp"))?;
    src = "Tmp";

    // miniature
    update::update_miniature_chapter_number(src, week)?;
    update::update_miniature_image(src, &ranking)?;

    let mut counter_slide = 11;

    // absents
    let mut nb = 1;
    let mut miniature_image = 0;
    if ranking.absent.len() > 0 {
        for rank in ranking.absent {
            create::add_absent_or_newbie_slide(src, 8, counter_slide, &rank.name.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect(), rank.chapter)?;
            counter_slide += 1;
        }
    }

    // newbies
    for rank in ranking.newbies {
        create::add_absent_or_newbie_slide(src, 9, counter_slide, &rank.name.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect(), rank.chapter)?;
        counter_slide += 1;
    }
    
    // color pages
    for figure in ranking.color_pages {
        let name = figure.rank.name;
        let chapter = figure.rank.chapter;
        for image_src in figure.imgs {
            let img_resp = request::get_img_resp(image_src.as_str());
    
            match img_resp.status().is_success() {
                true => {
                    let img_bytes = img_resp.bytes().unwrap(); 
                    let img = image::load_from_memory(&img_bytes).unwrap();
                    img.save(format!("{}/ppt/media/colorPage{}.png", src, nb)).unwrap();
                    if img.height() > img.width() {
                        create::add_color_slide(src, 1, nb, counter_slide, &name, chapter)?;
                        // miniature
                        match miniature_image {
                            0 | 1 => update::update_image(format!("{}/ppt/slides/_rels/slide7.xml.rels", src), format!("../media/colorPage{}.png", nb).as_str(), 0)?,
                            2 => update::update_image(format!("{}/ppt/slides/_rels/slide7.xml.rels", src), format!("../media/colorPage{}.png", nb).as_str(), 2)?,
                            _ => {},
                        };
                        miniature_image += 1;
                        counter_slide += 1;
                    } else {
                        create::add_color_slide(src, 2, nb, counter_slide, &name, chapter)?;
                        counter_slide += 1;
                    }
                    nb+=1;
                }
    
                false => {
                    panic!("error getting color page image");
                }
            }
        }
    }



    // RANKING
    let len = &ranking.ranking.len();
    for (position, rank) in ranking.ranking.iter().rev().enumerate() {
        let rank_position = len - position;
        if let Some(previous_position) = previous_ranking.ranking.iter().position(|x| x.name == rank.name) {
            let previous_ranking_position = (previous_position + 1) as i8;
            println!("{} previous ranking: {}", &rank.name, previous_ranking_position);
            create::add_slide(src, rank, rank_position as i8, Some(previous_ranking_position), counter_slide)?;
        } else {
            create::add_slide(src, rank, rank_position as i8, None, counter_slide)?;
        }
        counter_slide += 1
    }
    
    // add_slide(src, 3)?;

    file_operations::create_pptx(src)?;
    Ok(())
}