extern crate graph;
use graph::Graph;

fn main() {
    if let Ok(mut _graph) = Graph::<String>::deserialize("testGraph") {
        match _graph.bfs("1", "3") {
            None => None,
            Some(path) => Some({
                path.iter().for_each( |n| print!("{}-",n) );
            })
        };
        _graph.rm_node("3");
        _graph.serialize("testGraph_modifyed");
    }
}