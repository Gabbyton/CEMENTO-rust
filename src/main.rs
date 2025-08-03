use roxmltree::Document;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct DiagramElement {
    id: String,
    value: Option<String>,
    attributes: Option<HashMap<String, Option<String>>>,
}

// #[derive(Debug, Serialize, Deserialize)]
// struct DrawioGeometry {
//     x: f64,
//     y: f64,
//     width: f64,
//     height: f64,
// }

// #[derive(Serialize, Deserialize)]
// struct DrawioDocument {
//     cells: HashMap<String, DrawioCell>,
// }

fn parse_drawio_file(xml_content: &str) {
    let document = Document::parse(xml_content).expect("Cannot parse the document");
    // get diagram first page root element
    let diagram_root = document
        .descendants()
        .find(|child| child.is_element() && child.tag_name().name() == "root")
        .expect("cannot find root element!");

    // get elements within the diagram alongside style information
    let mut diagram_elements: Vec<DiagramElement> = Vec::new();
    for element in diagram_root.children().filter(|child| child.is_element()) {
        let element_id = element.attribute("id").unwrap().to_string();
        let value = element.attribute("value").and_then(|s| Some(s.to_string()));

        let mut diagram_element = DiagramElement {
            id: element_id,
            value: value,
            attributes: None,
        };

        let styles = element.attribute("style").map(|f| f.split(';'));
        if let Some(style) = styles {
            let style_elems = style.map(|s| s.to_string()).filter(|s| !s.is_empty());
            let style_map: HashMap<String, Option<String>> = style_elems
                .map(|s| match s.split_once('=') {
                    Some((k, v)) => (k.to_string(), Some(v.to_string())),
                    None => (s, None),
                })
                .collect();
            diagram_element.attributes = Some(style_map);
        }
        diagram_elements.push(diagram_element);
    }
    println!("{:#?}", diagram_elements);
    // assign elements to term or edge structs based on type
    // for element in
}

fn main() {
    let xml_content = std::fs::read_to_string("sample.drawio").expect("cannot read drawio diagram");
    parse_drawio_file(&xml_content);
    // let drawio_document = parse_drawio_file(&xml_content)?;
    // println!("{:#?}", drawio_document);
}
