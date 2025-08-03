pub use definitions::{DiagramEdge, DiagramElement, DiagramTerm};

pub mod definitions {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct DiagramElement {
        pub id: String,
        pub value: Option<String>,
        pub parent: Option<String>,
        pub attributes: Option<HashMap<String, Option<String>>>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct DiagramTerm {
        pub id: String,
        pub label: Option<String>,
        pub parent: Option<String>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct DiagramEdge {
        pub id: String,
        pub label: Option<String>,
        pub source_id: Option<String>,
        pub target_id: Option<String>,
    }
}
