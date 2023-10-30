use std::fs;

use jumprankingsapi::models::ranking_model::Rank;
use rand::Rng;
use walkdir::WalkDir;

use super::update;

pub fn add_slide(src: &str, rank: &Rank, rank_position: i8, previous_rank_position : Option<i8>, current_slide: i8) -> std::io::Result<()> {

    let slide_num: i8;

    let mut variation: i8 = 0;
    if rank_position == 1 {
        slide_num = 5;
    } else if let Some(prev) = previous_rank_position {
        variation = prev - rank_position;
        if variation < 0 {
            if rank_position > 7 {
                slide_num = 4;
            } else {
                slide_num = 6;
            }
        } else if variation > 0 {
            if (variation < 3) && rank_position < 3 {
                slide_num = 6;
            } else if rank_position < 3 || (rank_position < 7 && (variation > 3)){
                slide_num = 3;
            } else if rank_position < 7 && (variation < 3) {
                slide_num = 4;
            } else {
                slide_num = 6;
            }
        } else {
            slide_num = 6;
        }
    } else if rank_position < 3 && rank_position >=7 {
        slide_num = 6;
    } else if rank_position >= 3 {
        slide_num = 3;
    } else {
        slide_num = 4;
    }
    
    // 1. copier slideX.xml
    // fs::copy(format!("{}/ppt/slides/slide{}.xml", src, slide_num-1), format!("{}/ppt/slides/slide{}.xml", src, slide_num))?;
    // 2. modifier le slideX.xml:
    let mut variation_option = None;
    if previous_rank_position.is_some() {
        variation_option = Some(variation)
    }
    update::update_slide(src, slide_num, rank_position, variation_option, current_slide, &rank.name, rank.chapter)?;
    /* 
    - La couleur du texte de variance: <a:srgbClr val="FF0000" />
    - La valeur de la variance: <a:t>-1</a:t>
    - Le texte sous l'image: <a:t>SAKAMOTO DAYS (#143)</a:t>   
    */
    // 3. copier le slideX.xml.rels
    fs::copy(format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, slide_num), format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, current_slide))?;

    let mut rng = rand::thread_rng();
    
    for file in fs::read_dir("./Media").unwrap() {
        let name: String = rank.name.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect();
        let file_name = file.unwrap().file_name();
        let file_name_to_compare: String = file_name.to_string_lossy().chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect();
        if file_name_to_compare.eq_ignore_ascii_case(&name) {
            let path = format!("Media/{}", &name);
            let nb_images = WalkDir::new(path).into_iter().count();
            let mut img_number = 0;
            if nb_images > 1 {
                img_number = rng.gen_range(0..nb_images-1);
            }
            for (i, image) in fs::read_dir(format!("./Media/{}", &file_name.to_string_lossy())).unwrap().enumerate() {
                if i == img_number {
                    // img.save(format!("{}/ppt/media/colorPage{}.png", src, nb)).unwrap();
                    let image_path = image.as_ref().unwrap().path();
                    let new_image_path = format!("{}/ppt/media/{}", src, image.as_ref().unwrap().file_name().to_string_lossy());
                    let relative_image_path = format!("../media/{}", image.as_ref().unwrap().file_name().to_string_lossy());
                    fs::copy(image_path, new_image_path)?;
                    // println!("bonjour: {}", &image.unwrap().path().display());
                    update::update_image(format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, current_slide), &relative_image_path, -1)?;
                    break;
                }
            }
            break;
        }
    }
    
    // Créer fonction pour prendre l'image dans le bon dossier en fonction du nom
    // fs::copy("TroisiemeJet/ppt/slides/_rels/slide3.xml.rels", "TroisiemeJet/ppt/slides/_rels/slide4.xml.rels");
    // 4. modifier le presentation.xml.rels (rajouter la slide et update les chiffres)
    // fs::copy(format!("{}/ppt/_rels/slide{}.xml.rels", src, slide_num-1), format!("{}/ppt/_rels/slide{}.xml.rels", src, slide_num))?;
    let r_id = update::update_rels(format!("{}/ppt/_rels/presentation.xml.rels", src), current_slide)?;
    // 5. modifier le presentation.xml:  ajouter le <p:sldId id="261" r:id="rIdX" /> -> incrémenté de 1 le id et mettre le bon rId
    update::update_presentation_xml(src, r_id)?;
    Ok(())
}

pub fn add_color_slide(src: &str, slide_num: i8, nb_slide: i8, current_slide: i8) -> std::io::Result<()> {
    // 1. copier slideX.xml
    fs::copy(format!("{}/ppt/slides/slide{}.xml", src, slide_num), format!("{}/ppt/slides/slide{}.xml", src, current_slide))?;

    fs::copy(format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, slide_num), format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, current_slide))?;
    
    update::update_image(format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, current_slide), format!("../media/colorPage{}.png", &nb_slide).as_str(), -1)?;
    let r_id = update::update_rels(format!("{}/ppt/_rels/presentation.xml.rels", src), current_slide)?;
    // 5. modifier le presentation.xml:  ajouter le <p:sldId id="261" r:id="rIdX" /> -> incrémenté de 1 le id et mettre le bon rId
    update::update_presentation_xml(src, r_id)?;
    Ok(())
}