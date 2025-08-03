use roxmltree::Document;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Deref,
};

#[derive(Debug)]
struct DiagramElement {
    id: String,
    value: Option<String>,
    parent: Option<String>,
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

fn parse_drawio_file(xml_content: &str) -> Vec<DiagramElement> {
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
        let parent = element
            .attribute("parent")
            .and_then(|s| Some(s.to_string()));

        let mut diagram_element = DiagramElement {
            id: element_id,
            value: value,
            parent: parent,
            attributes: None,
        };
        let mut extra_attributes: HashMap<String, Option<String>> = HashMap::new();
        let default_attributes: HashSet<&str> = HashSet::from(["id", "value", "parent", "style"]);
        let all_attributes: HashSet<&str> = element.attributes().map(|e| e.name()).collect();
        let other_attributes = all_attributes.difference(&default_attributes);

        for attr in other_attributes {
            extra_attributes.insert(
                String::from(*attr),
                element.attribute(*attr).and_then(|s| Some(s.to_string())),
            );
        }

        // add the values within the styles attribute and add them as object attributes
        let styles = element.attribute("style").map(|f| f.split(';'));
        if let Some(style) = styles {
            let style_elems = style.map(|s| s.to_string()).filter(|s| !s.is_empty());
            let style_map: HashMap<String, Option<String>> = style_elems
                .map(|s| match s.split_once('=') {
                    Some((k, v)) => (k.to_string(), Some(v.to_string())),
                    None => (s, None),
                })
                .collect();
            // diagram_element.attributes = Some(style_map);
            extra_attributes.extend(style_map);
        }
        diagram_element.attributes = Some(extra_attributes);
        diagram_elements.push(diagram_element);
    }
    diagram_elements
}

fn get_diagram_terms(diagram_elements: &Vec<DiagramElement>) -> Vec<&DiagramElement> {
    let diagram_terms: Vec<&DiagramElement> = diagram_elements
        .iter()
        .filter(|element: &&DiagramElement| {
            element.attributes.as_ref().is_some_and(|attrs| {
                !attrs.contains_key("edgeLabel")
                    && attrs
                        .get("vertex")
                        .is_some_and(|vertex| *vertex == Some("1".to_string()))
            })
        })
        .collect();
    diagram_terms
}

fn main() {
    let xml_content = std::fs::read_to_string("sample.drawio").expect("cannot read drawio diagram");
    let diagram_elements = parse_drawio_file(&xml_content);
    let diagram_terms = get_diagram_terms(&diagram_elements);
}
