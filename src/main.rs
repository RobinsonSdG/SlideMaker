// use serde::{Deserialize, Serialize};
// use serde_xml_rs::{from_str, to_string};
// use std::error::Error;
// use std::fs::{self, File};
// use std::io::Write;

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct   Relationships {
//     #[serde(rename = "xmlns")] 
//     xmlns: String,
//     #[serde(rename = "Relationship")] 
//     relationship: Vec<Relationship>
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct Relationship {
//     #[serde(rename = "Id")] 
//     id: String,
//     #[serde(rename = "Type")] 
//     slide_type: String,
//     #[serde(rename = "Target")] 
//     target: String
// }

use std::{fs::{File, self}, io::{BufReader, BufWriter}};

use minidom::Element;
use std::io::Write;
use regex::Regex;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let src = "DernierJetInch2";
    add_slide(src, 3)?;
    Ok(())
}


fn add_slide(src: &str, slide_num: i8) -> std::io::Result<()> {
    // 1. copier slideX.xml
    // fs::copy(format!("{}/ppt/slides/slide{}.xml", src, slide_num-1), format!("{}/ppt/slides/slide{}.xml", src, slide_num))?;
    // 2. modifier le slideX.xml:
    update_slide(src, slide_num, 2)?;
    /* 
        - La couleur du texte de variance: <a:srgbClr val="FF0000" />
        - La valeur de la variance: <a:t>-1</a:t>
        - Le texte sous l'image: <a:t>SAKAMOTO DAYS (#143)</a:t>   
    */
    // 3. copier le slideX.xml.rels
    fs::copy(format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, slide_num-1), format!("{}/ppt/slides/_rels/slide{}.xml.rels", src, slide_num))?;
    // fs::copy("TroisiemeJet/ppt/slides/_rels/slide3.xml.rels", "TroisiemeJet/ppt/slides/_rels/slide4.xml.rels");
    // 4. modifier le presentation.xml.rels (rajouter la slide et update les chiffres)
    // fs::copy(format!("{}/ppt/_rels/slide{}.xml.rels", src, slide_num-1), format!("{}/ppt/_rels/slide{}.xml.rels", src, slide_num))?;
    let r_id = update_rels(format!("{}/ppt/_rels/presentation.xml.rels", src), slide_num)?;
    // 5. modifier le presentation.xml:  ajouter le <p:sldId id="261" r:id="rIdX" /> -> incrémenté de 1 le id et mettre le bon rId
    Ok(())
}


//Possibilité de créer le nouveau fichier en même temps
fn update_slide(src: &str, slide_num: i8, rank: i8) -> std::io::Result<()> {
    let search_pattern = r"<a:t>-\d+</a:t>";
    let replace_with = "<a:t>+2</a:t>"; //rank

    let input_file_path = format!("{}/ppt/slides/slide{}.xml", src, slide_num-1);
    let output_file_path = format!("{}/ppt/slides/slide{}.xml", src, slide_num);

    let mut input_file = File::open(input_file_path)?;

    // Ouvrez le fichier de sortie en écriture
    let output_file = match File::create(output_file_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Erreur lors de la création du fichier de sortie : {}", err);
        }
    };

    // Créez un lecteur pour le fichier d'entrée
    let reader = BufReader::new(input_file);

    // Créez un écrivain pour le fichier de sortie
    let mut writer = BufWriter::new(output_file);

    // Créez une expression régulière pour la recherche
    let regex = Regex::new(search_pattern).expect("Erreur lors de la création de l'expression régulière");

    for line in reader.lines() {
        let line = line.expect("Erreur lors de la lecture de la ligne");

        // Utilisez l'expression régulière pour rechercher et remplacer dans la ligne
        let modified_line = regex.replace_all(&line, replace_with);

        // Écrivez le résultat modifié dans le fichier de sortie
        writeln!(writer, "{}", modified_line).expect("Erreur lors de l'écriture dans le fichier de sortie");
    }
    Ok(())
}


fn update_rels(src: String, slide_num: i8) -> std::io::Result<i32> {
    let data = std::fs::read_to_string(&src)?;
    let mut root: Element = data.parse().unwrap();

    let mut id = 1;
    for _ in root.children() {
        id += 1
    }

    let newcd = Element::builder("Relationship", "")
        .attr("Id", format!("rId{}", id))
        .attr("Type", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide")
        .attr("Target", format!("slides/slide{}.xml", slide_num))
        .build();
    root.append_child(newcd);

    let mut buf = Vec::<u8>::new();
    root.write_to(&mut buf).unwrap();
    // println!("{}", String::from_utf8(buf).unwrap());

    let mut file = File::create(&src)?;
    file.write_all(&buf)?;
    Ok(id)
}





//     let xml: String = fs::read_to_string(src)?.parse()?;
   
//     print!("{}", xml);
//     // Utiliser ça pour modifier le presentation.xml.rels
//     let relationships: Relationships = match from_str(xml.as_str()) {
//         Ok(r) => r,
//         Err(e) => panic!("{}", e)
//     };

//     let reserialized_item = to_string(&relationships).unwrap();
//     assert_eq!(xml, reserialized_item);


//     // for relationship in relationships.Relationship {
//     //     println!("{:?}", relationship)
//     // }



//     // let mut file = File::create("res.xml")?;
//     // file.write_all(reserialized_item.as_bytes())?;

//     // for relationship in relationships.Relationship {
//     //     println!("{:?}", relationship)
//     // }

//     /*
//     Je pense qu'il faut une autre strat pour les slideX.xml
//     Les fichiers sont beaucoup trop grand avec beaucoup trop de params.
//     Mais il faut changer que quelques éléments:
//         - La couleur du texte de variance: <a:srgbClr val="FF0000" />
//         - La valeur de la variance: <a:t>-1</a:t>
//         - Le texte sous l'image: <a:t>SAKAMOTO DAYS (#143)</a:t>    
//     */
//     // add_slide();
//     Ok(())
// }



// fn update_rels(src: &str, slide_num: i8) -> Result<(), Box<dyn Error>> {
//     let xml: String = fs::read_to_string(src)?.parse()?;
   

//     // Utiliser ça pour modifier le presentation.xml.rels
//     let mut relationships: Relationships = match from_str(xml.as_str()) {
//         Ok(r) => r,
//         Err(e) => panic!("{}", e)
//     };

//     for relationship in relationships.relationship {
//         println!("{:?}", relationship)
//     }

//     let mut id = format!("rId{}", slide_num);

//     let new_rel = Relationship {
//         id: id,
//         target: format!("slides/slide{}.xml", slide_num),
//         slide_type: "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide".to_string(),
//     };

//     // let mut new_rels = Relationships { Relationship: vec![] };

//     // for mut relationship in relationships.Relationship {
//     //     let rel_id: String = relationship.Id.chars().filter(|c| c.is_digit(10)).collect();
//     //     match rel_id.parse::<i8>() {
//     //         Ok(i) => {
//     //             if i > slide_num {
//     //                 relationship.Id = format!("rId{}", i + 1);
//     //             }
//     //             new_rels.Relationship.push(relationship);
//     //         },
//     //         Err(e) => panic!("relationship.Id: {} err: {}", relationship.Id, e)
//     //     };
//     // }

//     // new_rels.Relationship.push(new_rel);

//     // let reserialized_item = match to_string(&relationships) {
//     //     Ok(s) => s,
//     //     Err(e) => panic!("to_string err: {}", e),
//     // };

//     // // fs::remove_file(src)?;
//     // let mut file = File::create(src)?;
//     // file.write_all(reserialized_item.as_bytes())?;

//     Ok(())
// }


// /* Test */

// // use std::fs::{self, File};
// // use std::error::Error;
// // use serde::{Deserialize, Serialize};
// // use serde_xml_rs::{from_str, to_string};
// // use std::io::Read;
// // use std::io::Write;

// // #[derive(Debug, Serialize, Deserialize, PartialEq)]
// // struct Item {
// //     name: String,
// //     source: String,
// // }

// // fn main() -> Result<(), Box<dyn Error>> {
// //     let xml = "test.xml";
// //     // let mut file = File::open(xml)?;
// //     // let mut src = String::new();
// //     // file.read_to_string(&mut src)?;
// //     let src: String = fs::read_to_string(xml)?.parse()?;
// //     // let src = r#"<?xml version="1.0" encoding="UTF-8"?><Item><name>Banana</name><source>Store</source></Item>"#;
// //     let should_be = Item {
// //         name: "Banana".to_string(),
// //         source: "Store".to_string(),
// //     };

// //     let item: Item = from_str(src.as_str()).unwrap();
// //     assert_eq!(item, should_be);

// //     let reserialized_item = to_string(&item).unwrap();
// //     assert_eq!(src, reserialized_item);

// //     let mut file = File::create("res.xml")?;
// //     file.write_all(reserialized_item.as_bytes())?;

// //     Ok(())
// // }

// extern crate serde;
// extern crate serde_xml_rs;

// use serde::{Deserialize, Serialize};
// use serde_xml_rs::{from_str, to_string};

// #[derive(Debug, Deserialize, Serialize)]
// struct Relationship {
//     #[serde(rename = "Id")]
//     id: String,
//     #[serde(rename = "Type")]
//     type_: String,
//     #[serde(rename = "Target")]
//     target: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct Relationships {
//     relationships: Vec<Relationship>,
// }

// fn main() {
//     let xml_str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
// <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
//     <Relationship Id="rId8"
//         Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles"
//         Target="tableStyles.xml" />
//     <Relationship Id="rId3"
//         Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide"
//         Target="slides/slide2.xml" />
//     <!-- Ajoutez les autres relations ici -->
// </Relationships>"#;

//     // Désérialisez le document XML sans les espaces de noms
//     let relationships: Relationships = from_str(xml_str).unwrap();
//     println!("{:#?}", relationships);

//     // Pour sérialiser, utilisez to_string()
//     let serialized_xml = to_string(&relationships).unwrap();
//     println!("{}", serialized_xml);
// }

