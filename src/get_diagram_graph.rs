struct DiagramTerm {
    id: String,
    label: String,
    is_literal: bool,
    parent: Option<String>,
}

struct DiagramEdge {
    id: String,
    label: String,
    source_id: String,
    target_id: String,
    is_strat: bool,
    is_rank: bool,
    is_pred: bool,
}