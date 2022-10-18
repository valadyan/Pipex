extern crate graph;
use graph::Graph;

fn main() {
    let mut gr = Graph::<&str>::new(3);
    gr.add_node(1.to_string(), "asc");
    gr.add_node(2.to_string(), "cc");
    gr.add_node(3.to_string(), "");
    gr.add_link(1.to_string(),2.to_string());
    gr.add_link(3.to_string(),2.to_string());

    gr.serialize("testgr")
    // if let Ok(_graph) = Graph::<String>::deserialize("testGraph") {
    //     _graph.serialize("asdf");
    // }
}
