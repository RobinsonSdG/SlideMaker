use std::{fs::{File, OpenOptions}, io::{Write, self, Read, Seek}};
use crate::utils::request::get_img_resp;

use jumprankingsapi::models::ranking_model::Ranking;
use minidom::Element;
use regex::Regex;
use xml::{EventReader, reader::XmlEvent};

pub fn update_image(src: String, img: &str, rel_nb: i8) -> std::io::Result<()> {
    let data = std::fs::read_to_string(&src)?;
    let mut root: Element = data.parse().unwrap();

    let file = File::open(&src).expect("Failed to open file");
    let parser = EventReader::new(file);

    let mut relationship_id_to_remove: Option<String> = None;

    let mut nb = 0;
    for event in parser {
        if let Ok(XmlEvent::StartElement { name, attributes, .. }) = event {
            if name.local_name == "Relationship" {
                for attribute in attributes {
                    if attribute.name.local_name == "Id" && (rel_nb == -1 || nb == rel_nb) {
                        relationship_id_to_remove = Some(attribute.value);
                        break;
                    }
                }
                nb+=1;
            }
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

    if rel_nb == -1 {
        tmp.pop();
    } else {
        tmp.remove(rel_nb as usize);
    }

    for child in tmp {
        root.append_child(child);
    }

    if let Some(id) = relationship_id_to_remove {
        let newcd = Element::builder("Relationship", "http://schemas.openxmlformats.org/package/2006/relationships")
            .attr("Id", id)
            .attr("Type", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image")
            .attr("Target", img.to_string())
            .build();
        root.append_child(newcd);
    
    } else {
        println!("Aucun Relationship trouvé dans le fichier XML: {}.", &src);
    }

    let mut buf = Vec::<u8>::new();
    root.write_to(&mut buf).unwrap();
    // println!("{}", String::from_utf8(buf).unwrap());

    let mut file = File::create(&src)?;
    file.write_all(&buf)?;
    Ok(())
}

pub fn update_rels(src: String, slide_num: i8) -> std::io::Result<i32> {
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

pub fn update_miniature_images(src: &str, ranking: &Ranking) -> io::Result<()> {
    // cover
    let cover_image = &ranking.cover;
    let cover_image_src = format!("{}/ppt/media/coverPage.png", src);
    let img_resp = get_img_resp(cover_image.as_str());

    match img_resp.status().is_success() {
        true => {
            let img_bytes = img_resp.bytes().unwrap();
            let img = image::load_from_memory(&img_bytes).unwrap();
            img.save(cover_image_src).unwrap();
        }
        false => {
            panic!("error getting cover page image");
        }
    }

    update_image(format!("{}/ppt/slides/_rels/slide7.xml.rels", src), "../media/coverPage.png", 3)?;
    Ok(())
}

pub fn update_miniature_chapter_number(src: &str, week: i32) -> io::Result<()>{
    let input_filename = format!("{}/ppt/slides/slide7.xml", src);

    // Ouvrir le fichier en lecture/écriture
    let mut file = OpenOptions::new().read(true).write(true).open(&input_filename)?;

    // Lire le contenu du fichier
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Effectuer le remplacement
    let modified_content = content.replace("<a:t>#43</a:t>", &format!("<a:t>#{}</a:t>", &week));

    // Rembobiner le curseur du fichier au début et écrire les modifications
    file.seek(std::io::SeekFrom::Start(0))?;
    file.write_all(modified_content.as_bytes())?;
    file.set_len(modified_content.len() as u64)?;

    println!("Remplacement terminé dans le fichier '{}'", &input_filename);

    Ok(())
}

pub fn update_slide(src: &str, slide_num: i8, rank: i8, rank_changement: Option<i8>, counter_slide: i8, name: &String, chapter: i16) -> std::io::Result<()> {
    let input_filename = format!("{}/ppt/slides/slide{}.xml", src, slide_num);
    let output_filename = format!("{}/ppt/slides/slide{}.xml", src, counter_slide);

    let mut input_file = File::open(input_filename)?;
    let mut input_contents = String::new();
    input_file.read_to_string(&mut input_contents)?;

    let re1 = Regex::new(r"<a:t>([+-]?\d+)</a:t>").unwrap();
    let re2 = Regex::new(r"<a:t>([a-zA-Z\s]+)</a:t>").unwrap();
    let re3 = Regex::new(r"<a:t>\(#([0-9]+)\)</a:t>").unwrap();
    let re4 = Regex::new(r#"<a:srgbClr val="BB1A00"/></a:solidFill></a:ln><a:solidFill><a:srgbClr val="FFB10B"/></a:solidFill><a:latin typeface="Bahnschrift SemiBold SemiConden""#).unwrap();
    
    let mut replacement = String::new();
    let mut color = "";
    let mut outline = "";
    let mut color_change = false;
    match rank_changement {
        Some(r) => {
            println!("{} rank_changement: {}", name, r);
            (replacement, color, outline) = match r.cmp(&0) {
                std::cmp::Ordering::Greater => (format!("<a:t>+{}</a:t>", r), "2EF729", "DF5656"),
                std::cmp::Ordering::Less => (format!("<a:t>{}</a:t>", r), "DF5656", "BB1A00"),
                std::cmp::Ordering::Equal => ("<a:t>=</a:t>".to_string(), "", ""),
            };
            color_change = r != 0;
        },
        None => replacement = r"<a:t>in!</a:t>".to_string(),
    };

    let mut modified_contents: String = "".to_string();
    if let Some(caps) = re1.captures(&input_contents) {
        let first_match = caps.get(0).unwrap().as_str();

        modified_contents = input_contents.replacen(first_match, &replacement, 1);

        println!("Remplacement de la première occurrence terminé. Le résultat a été enregistré dans '{}'", &output_filename);
    } else {
        println!("Aucune première correspondance trouvée dans le fichier.");
    }
    if let Some(caps) = re2.captures(&input_contents) {
        let first_match = caps.get(0).unwrap().as_str();

        modified_contents = modified_contents.replacen(first_match, format!("<a:t>{}</a:t>", &name).as_str(), 1);

        println!("Remplacement de la deuxième occurrence terminé. Le résultat a été enregistré dans '{}'", &output_filename);
    } else {
        println!("Aucune deuxième correspondance trouvée dans le fichier.");
    }
    if let Some(caps) = re3.captures(&input_contents) {
        let first_match = caps.get(0).unwrap().as_str();

        modified_contents = modified_contents.replacen(first_match, format!("<a:t>(#{})</a:t>", &chapter).as_str(), 1);

        println!("Remplacement de la troisième occurrence terminé. Le résultat a été enregistré dans '{}'", &output_filename);
    } else {
        println!("Aucune troisième correspondance trouvée dans le fichier.");
    }
    if color_change {
        if let Some(caps) = re4.captures(&input_contents){
            let first_match = caps.get(0).unwrap().as_str();

            modified_contents = modified_contents.replacen(first_match, format!("<a:srgbClr val=\"{}\"/></a:solidFill></a:ln><a:solidFill><a:srgbClr val=\"{}\"/></a:solidFill><a:latin typeface=\"Bahnschrift SemiBold SemiConden\"", &outline, &color).as_str(), 1);
    
            println!("Remplacement de la quatrième occurrence terminé. Le résultat a été enregistré dans '{}'", &output_filename);
        } else {
            println!("Aucune quatrième correspondance trouvée dans le fichier.");
        }
    }
     

    let mut output_file = File::create(&output_filename)?;
    output_file.write_all(modified_contents.as_bytes())?;

    Ok(())
}

pub fn update_presentation_xml(src: &str, r_id: i32) -> std::io::Result<()> {
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