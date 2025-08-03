use ammonia::{Builder, clean};
use roxmltree::Document;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct DiagramElement {
    id: String,
    value: Option<String>,
    parent: Option<String>,
    attributes: Option<HashMap<String, Option<String>>>,
}

#[derive(Debug)]
struct DiagramTerm {
    id: String,
    label: Option<String>,
    parent: Option<String>,
}

#[derive(Debug)]
struct DiagramEdge {
    id: String,
    label: Option<String>,
    source_id: Option<String>,
    target_id: Option<String>,
}

fn clean_term(term: String) -> String {
    let clean_term = Builder::new().tags(HashSet::new()).clean(&term).to_string();
    let clean_term = clean_term.replace("\"", "");
    return clean_term.trim().to_string();
}

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

fn parse_diagram_terms(diagram_terms: Vec<&DiagramElement>) -> Vec<DiagramTerm> {
    let mut parsed_diagram_terms: Vec<DiagramTerm> = Vec::new();
    for term in diagram_terms {
        let term_parent = term
            .parent
            .to_owned()
            .and_then(|parent_id| match &*parent_id {
                "1" => None,
                _ => Some(parent_id),
            });
        let diagram_term = DiagramTerm {
            id: term.id.to_owned(),
            label: term.value.to_owned().map(|label| clean_term(label)),
            parent: term_parent,
        };
        parsed_diagram_terms.push(diagram_term);
    }
    parsed_diagram_terms
}

fn get_diagram_edges(diagram_elements: &Vec<DiagramElement>) -> Vec<&DiagramElement> {
    let diagram_edges: Vec<&DiagramElement> = diagram_elements
        .iter()
        .filter(|element| {
            element.attributes.as_ref().is_some_and(|attrs| {
                attrs.contains_key("source")
                    || attrs.contains_key("target")
                    || attrs.contains_key("edgeLabel")
            })
        })
        .collect();
    diagram_edges
}

fn parse_diagram_edges(diagram_edges: Vec<&DiagramElement>) -> Vec<DiagramEdge> {
    let mut parsed_diagram_edges: Vec<DiagramEdge> = Vec::new();
    for edge in diagram_edges {
        let (source_id, target_id) = edge.attributes.as_ref().map_or((None, None), |attrs| {
            (attrs.get("source").cloned(), attrs.get("target").cloned())
        });
        let diagram_edge: DiagramEdge = DiagramEdge {
            id: edge.id.to_owned(),
            label: edge.value.to_owned().map(|label| clean_term(label)),
            source_id: source_id.flatten(),
            target_id: target_id.flatten(),
        };
        parsed_diagram_edges.push(diagram_edge);
    }
    parsed_diagram_edges
}

fn main() {
    let xml_content = std::fs::read_to_string("sample.drawio").expect("cannot read drawio diagram");
    let diagram_elements = parse_drawio_file(&xml_content);
    let diagram_terms = get_diagram_terms(&diagram_elements);
    let diagram_edges = get_diagram_edges(&diagram_elements);
    let diagram_terms = parse_diagram_terms(diagram_terms);
    let diagram_edges = parse_diagram_edges(diagram_edges);
    println!("{:#?}", diagram_terms);
    println!("{:#?}", diagram_edges);
}
