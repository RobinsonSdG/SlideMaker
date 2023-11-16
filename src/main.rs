mod ranking;
use ranking::operations;
mod utils;
use utils::file_operations;
use utils::request;
mod pptx;
use pptx::update;
use pptx::create;

use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut src = "SlideTemplateStp";
    let year = 2023;
    let week = 50;
    
    let ranking = operations::get_ranking(year, week);
    let previous_ranking = operations::get_ranking(year, week-1);

    file_operations::copy_directory(Path::new(src), Path::new("Tmp"))?;
    src = "Tmp";

    // miniature
    update::update_miniature_chapter_number(src, week)?;
    update::update_miniature_images(src, &ranking)?;

    let mut counter_slide = 8;
    // COLOR PAGES
    let mut nb = 1;
    let mut miniature_image = 0;
    for image_src in ranking.color_pages {
        let img_resp = request::get_img_resp(image_src.as_str());

        match img_resp.status().is_success() {
            true => {
                let img_bytes = img_resp.bytes().unwrap();
                let img = image::load_from_memory(&img_bytes).unwrap();
                img.save(format!("{}/ppt/media/colorPage{}.png", src, nb)).unwrap();
                if img.height() > img.width() {
                    create::add_color_slide(src, 1, nb, counter_slide)?;
                    // miniature
                    match miniature_image {
                        0 => update::update_image(format!("{}/ppt/slides/_rels/slide7.xml.rels", src), format!("../media/colorPage{}.png", nb).as_str(), 1)?,
                        1 | 2 => update::update_image(format!("{}/ppt/slides/_rels/slide7.xml.rels", src), format!("../media/colorPage{}.png", nb).as_str(), 4)?,
                        _ => {},
                    };
                    miniature_image += 1;
                    counter_slide += 1;
                } else {
                    create::add_color_slide(src, 2, nb, counter_slide)?;
                    counter_slide += 1;
                }
                nb+=1;
            }

            false => {
                panic!("error getting color page image");
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
            counter_slide += 1
        } else {
            create::add_slide(src, rank, rank_position as i8, None, counter_slide)?;
            counter_slide += 1
        }
    }
    
    // add_slide(src, 3)?;

    file_operations::create_pptx(src)?;
    Ok(())
}