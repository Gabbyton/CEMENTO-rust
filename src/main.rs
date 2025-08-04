mod draw_io;
mod templates;

use draw_io::diagram_draw::draw_diagram;
use draw_io::diagram_read::read_diagram;
use petgraph::visit::EdgeRef;
fn main() {
    let graph = read_diagram("cco.drawio");
    // for edge in graph.edge_references() {
    //     println!("{:#?}", graph.node_weight(edge.source()));
    //     println!("{:#?}", graph.node_weight(edge.target()));
    // }
    draw_diagram();
}
