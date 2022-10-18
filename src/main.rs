extern crate graph;
use graph::Graph;

fn main() {
    if let Ok(_graph) = Graph::<String>::deserialize("testGraph") {
        _graph.serialize("asdf");
    }
}