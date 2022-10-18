extern crate graph;
use graph::Graph;

fn main() {
    let mut gr = Graph::<&str>::new(3);
    gr.add_node("asc");
    gr.add_node("cc");
    gr.add_node("");
    gr.add_link(1,2);
    gr.add_link(3,2);

    gr.serialize("testgr")
    // if let Ok(_graph) = Graph::<String>::deserialize("testGraph") {
    //     _graph.serialize("asdf");
    // }
}
