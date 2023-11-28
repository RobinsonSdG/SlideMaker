use std::{fs::{self, File}, io::{Read, Write}};

use jumprankingsapi::models::ranking_model::Rank;
use rand::Rng;
use regex::Regex;
use walkdir::WalkDir;

use super::update;

pub fn add_slide(src: &str, rank: &Rank, rank_position: i8, previous_rank_position : Option<i8>, current_slide: i8) -> std::io::Result<()> {

    let slide_num: i8;

    let mut variation: i8 = 0;
    if let Some(prev) = previous_rank_position {
        variation = prev - rank_position
    }
    if rank_position == 1 {
        slide_num = 5;
    } else if variation < 0 {
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
    // if rank_position < 3 && rank_position >=7 {
    //     slide_num = 6;
    // } else if rank_position >= 3 {
    //     slide_num = 3;
    // } else {
    //     slide_num = 4;
    // }
    
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
    update::update_image_from_media(src, slide_num, rank, current_slide, rank_position)?;
    
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

pub fn add_absent_slide(src: &str, slide_num: i8, current_slide: i8, rank: &Rank) -> std::io::Result<()> {
    // 1. copier slideX.xml
    // fs::copy(format!("{}/ppt/slides/slide{}.xml", src, slide_num), format!("{}/ppt/slides/slide{}.xml", src, current_slide))?;

    // fs::copy(format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, slide_num), format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, current_slide))?;
    
    // update name
    let input_filename = format!("{}/ppt/slides/slide{}.xml", src, slide_num);
    let output_filename = format!("{}/ppt/slides/slide{}.xml", src, current_slide);
    let mut input_file = File::open(input_filename)?;
    let mut input_contents = String::new();
    input_file.read_to_string(&mut input_contents)?;
    let re = Regex::new(r"<a:t>([a-zA-Z\s]+)</a:t>").unwrap();
    
    let mut modified_contents: String = "".to_string();
    if let Some(caps) = re.captures(&input_contents) {
        let first_match = caps.get(0).unwrap().as_str();
        
        modified_contents = input_contents.replacen(first_match, format!("<a:t>{}</a:t>", &rank.name).as_str(), 1);
        
        println!("Remplacement de la deuxième occurrence terminé. Le résultat a été enregistré dans '{}'", &output_filename);
    }

    let mut output_file = File::create(&output_filename)?;
    output_file.write_all(modified_contents.as_bytes())?;
    
    update::update_image_from_media(src, slide_num, rank, current_slide, 0)?;

    let r_id = update::update_rels(format!("{}/ppt/_rels/presentation.xml.rels", src), current_slide)?;
    // 5. modifier le presentation.xml:  ajouter le <p:sldId id="261" r:id="rIdX" /> -> incrémenté de 1 le id et mettre le bon rId
    update::update_presentation_xml(src, r_id)?;
    Ok(())
}