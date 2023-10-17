use std::{fs::{File, self}, io::{BufReader, BufWriter}, ffi::OsString};

use xml::reader::{EventReader, XmlEvent};

use rand::Rng;
use walkdir::WalkDir;

use reqwest::blocking::Response;
use minidom::Element;
use std::io::Write;
use regex::Regex;
use std::io::BufRead;
use jumprankingsapi::models::ranking_model::{Rank, Ranking, Rankings};

fn main() -> std::io::Result<()> {
    let src = "SlideTemplateStp";
    let year = 2023;
    let week = 42;
    
    let ranking = get_ranking(year, week);
    let previous_ranking = get_ranking(year, week-1);

    // for r in ranking.ranking.iter().rev() {
    //     println!("{:?}", r.name);
    // }
    let mut counter_slide = 7;
    // COLOR PAGES
    let mut nb = 1;
    for image_src in ranking.color_pages {
        let img_resp = get_img_resp(&image_src.as_str());

        match img_resp.status().is_success() {
            true => {
                let img_bytes = img_resp.bytes().unwrap();
                let img = image::load_from_memory(&img_bytes).unwrap();
                img.save(format!("{}/ppt/media/colorPage{}.png", src, nb)).unwrap();
                if img.height() > img.width() {
                    add_color_slide(src, 1, nb, counter_slide)?;
                    counter_slide += 1;
                } else {
                    add_color_slide(src, 2, nb, counter_slide)?;
                    counter_slide += 1;
                }
                nb+=1;
            }

            false => {
                println!("error");
            }
        }
    }
    
    println!("ranking: {:?}", ranking.ranking);
    println!("previous_ranking: {:?}", previous_ranking.ranking);

    // RANKING
    let len = ranking.ranking.len();
    for (position, rank) in ranking.ranking.iter().rev().enumerate() {
        let rank_position = len - position;
        if let Some(previous_position) = previous_ranking.ranking.iter().position(|x| x.name == rank.name) {
            let previous_ranking_position = (previous_position + 1) as i8;
            println!("{} previous ranking: {}", &rank.name, previous_ranking_position);
            add_slide(src, rank, rank_position as i8, Some(previous_ranking_position), counter_slide)?;
            counter_slide += 1
        } else {
            add_slide(src, rank, rank_position as i8, None, counter_slide)?;
            counter_slide += 1
        }
    }
    
    // add_slide(src, 3)?;
    Ok(())
}

fn add_color_slide(src: &str, slide_num: i8, nb_slide: i8, current_slide: i8) -> std::io::Result<()> {
    // 1. copier slideX.xml
    fs::copy(format!("{}/ppt/slides/slide{}.xml", src, slide_num), format!("{}/ppt/slides/slide{}.xml", src, current_slide))?;

    fs::copy(format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, slide_num), format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, current_slide))?;
    
    update_image(format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, current_slide), format!("../media/colorPage{}.png", &nb_slide).as_str())?;
    let r_id = update_rels(format!("{}/ppt/_rels/presentation.xml.rels", src), current_slide)?;
    // 5. modifier le presentation.xml:  ajouter le <p:sldId id="261" r:id="rIdX" /> -> incrémenté de 1 le id et mettre le bon rId
    update_presentation_xml(src, r_id)?;
    Ok(())
}

fn get_img_resp(img_url: &str) -> Response {
    let img_resp = match reqwest::blocking::get(img_url) {
        Ok(v) => v,
        Err(e) => panic!("error requesting for image {}: {}", &img_url, e)
    };
    img_resp
}

fn get_ranking( year: i32, week: i32) -> Ranking {
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


fn add_slide(src: &str, rank: &Rank, rank_position: i8, previous_rank_position : Option<i8>, current_slide: i8) -> std::io::Result<()> {

    let slide_num: i8;

    let mut variation: Option<i8> = None;
    if rank_position == 1 {
        slide_num = 5;
    } else if let Some(prev) = previous_rank_position {
        variation = Some(rank_position - prev);
        if rank_position < prev {
            if rank_position < 7 {
                slide_num = 4;
            } else {
                slide_num = 6;
            }
        } else if rank_position > prev {
            if (rank_position - prev < 3) && rank_position < 3 {
                slide_num = 6;
            } else if rank_position < 7 && (rank_position - prev > 3) {
                slide_num = 3;
            } else if rank_position < 3 {
                slide_num = 3;
            } else if rank_position < 7 && (rank_position - prev < 3) {
                slide_num = 4;
            } else {
                slide_num = 6;
            }
        } else {
            slide_num = 6;
        }
    } else {
        if rank_position < 3 && rank_position >=7 {
            slide_num = 6;
        }
        else if rank_position >= 3 {
            slide_num = 3;
        } else {
            slide_num = 4;
        }
    }
    
    // 1. copier slideX.xml
    // fs::copy(format!("{}/ppt/slides/slide{}.xml", src, slide_num-1), format!("{}/ppt/slides/slide{}.xml", src, slide_num))?;
    // 2. modifier le slideX.xml:

    update_slide(src, slide_num, rank_position, variation, current_slide, &rank.name)?;
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
                    fs::copy(&image_path, &new_image_path)?;
                    // println!("bonjour: {}", &image.unwrap().path().display());
                    update_image(format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, current_slide), &relative_image_path)?;
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
    let r_id = update_rels(format!("{}/ppt/_rels/presentation.xml.rels", src), current_slide)?;
    // 5. modifier le presentation.xml:  ajouter le <p:sldId id="261" r:id="rIdX" /> -> incrémenté de 1 le id et mettre le bon rId
    update_presentation_xml(src, r_id)?;
    Ok(())
}

fn update_presentation_xml(src: &str, r_id: i32) -> std::io::Result<()> {
    let input_file_path = format!("{}/ppt/presentation.xml", src); // Remplacez par le chemin de votre fichier XML d'entrée

    // Lire le contenu du fichier XML en tant que texte
    let mut xml_content = std::fs::read_to_string(&input_file_path).expect("Impossible de lire le fichier");

    // Utiliser une expression régulière pour rechercher la dernière ligne <p:sldId> et obtenir les valeurs actuelles
    let re = Regex::new(r#"<p:sldId id="(\d+)" r:id="rId(\d+)"/>"#).expect("Erreur dans l'expression régulière");
    let mut last_id: i32 = 0;
    let mut last_match_end = 0;

    for captures in re.captures_iter(&xml_content) {
        last_id = captures[1].parse().expect("Impossible de convertir l'id en entier");
        last_match_end = captures.get(0).unwrap().end();
    }

    
    // Calculer les nouvelles valeurs
    let new_id = last_id + 1;

    // Construire la nouvelle ligne <p:sldId>
    let new_line = format!("<p:sldId id=\"{}\" r:id=\"rId{}\"/>", new_id, r_id);
    // Ajouter la nouvelle ligne à la fin du contenu XML
    xml_content.insert_str(last_match_end, &new_line);

    // Écrire le contenu modifié dans le fichier
    std::fs::write(&input_file_path, xml_content).expect("Impossible d'écrire dans le fichier");
   
    Ok(())
}

//Possibilité de créer le nouveau fichier en même temps
fn update_slide(src: &str, slide_num: i8, rank: i8, rank_changement: Option<i8>, counter_slide: i8, name: &String) -> std::io::Result<()> {

    let search_pattern = r"<a:t>-?\d+</a:t>";

    let mut replace_with = String::new();
    match rank_changement {
        Some(r) => {
            println!("{} rank_changement: {}", name, r);
            if r > 0 {
                replace_with = format!("<a:t>+{}</a:t>", r); //r
            } else if r < 0 {
                replace_with = format!("<a:t>-{}</a:t>", r); //r
            } else if r == 0 {
                replace_with = "<a:t>=</a:t>".to_string(); //rank
            }
        },
        None => replace_with = r"<a:t>new</a:t>".to_string(),
    };
    
    let input_file_path = format!("{}/ppt/slides/slide{}.xml", src, slide_num);
    let output_file_path = format!("{}/ppt/slides/slide{}.xml", src, counter_slide);

    fs::copy(input_file_path, &output_file_path)?;

    let output_file = File::open(&output_file_path)?;

    // // Ouvrez le fichier de sortie en écriture
    // let output_file = match File::create(output_file_path) {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("Erreur lors de la création du fichier de sortie : {}", err);
    //     }
    // };

    // Créez un lecteur pour le fichier d'entrée
    let reader = BufReader::new(output_file);

    // Créez un écrivain pour le fichier de sortie
    // let mut writer = BufWriter::new(output_file);

    // Créez une expression régulière pour la recherche
    let regex = Regex::new(search_pattern).expect("Erreur lors de la création de l'expression régulière");

    for line in reader.lines() {
        let line = line.expect("Erreur lors de la lecture de la ligne");

        // Utilisez l'expression régulière pour rechercher et remplacer dans la ligne
        let modified_line = regex.replace_all(&line, &replace_with);

        // Écrivez le résultat modifié dans le fichier de sortie
        // writeln!(writer, "{}", modified_line).expect("Erreur lors de l'écriture dans le fichier de sortie");
    }
    Ok(())
}

fn update_image(src: String, img: &str) -> std::io::Result<()> {
    let data = std::fs::read_to_string(&src)?;
    let mut root: Element = data.parse().unwrap();

    let mut id = 0;
    for _ in root.children() {
        id += 1
    }

    let file = File::open(&src).expect("Failed to open file");
    let parser = EventReader::new(file);

    let mut last_relationship_id: Option<String> = None;

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "Relationship" {
                    for attribute in attributes {
                        if attribute.name.local_name == "Id" {
                            last_relationship_id = Some(attribute.value);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let child_to_remove = root.children().cloned();
    let mut tmp: Vec<Element> = vec![];
    for child in child_to_remove {
        tmp.push(child);
    }

    for child in tmp.clone() {
        let name = child.name();
        let namespace = child.ns();
        root.remove_child(name, namespace.as_str());
    }
    tmp.pop();
    for child in tmp {
        root.append_child(child);
    }

    if let Some(id) = last_relationship_id {
        let newcd = Element::builder("Relationship", "")
            .attr("Id", &id)
            .attr("Type", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")
            .attr("Target", format!("{}", img))
            .build();
        root.append_child(newcd);
    
    } else {
        println!("Aucun Relationship trouvé dans le fichier XML.");
    }

    let mut buf = Vec::<u8>::new();
    root.write_to(&mut buf).unwrap();
    // println!("{}", String::from_utf8(buf).unwrap());

    let mut file = File::create(&src)?;
    file. write_all(&buf)?;
    Ok(())
}

fn update_rels(src: String, slide_num: i8) -> std::io::Result<i32> {
    let data = std::fs::read_to_string(&src)?;
    let mut root: Element = data.parse().unwrap();

    let mut id = 1;
    for _ in root.children() {
        id += 1
    }

    let newcd = Element::builder("Relationship", "http://schemas.openxmlformats.org/package/2006/relationships")
        .attr("Id", format!("rId{}", id))
        .attr("Type", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide")
        .attr("Target", format!("slides/slide{}.xml", slide_num))
        .build();
    root.append_child(newcd);

    let mut buf = Vec::<u8>::new();
    root.write_to(&mut buf).unwrap();

    let mut file = File::create(&src)?;
    file.write_all(&buf)?;
    Ok(id)
}