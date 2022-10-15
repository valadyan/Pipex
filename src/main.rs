extern crate graph;

use graph::Graph;

use std::collections::btree_map::Iter;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    if let Ok(_graph) = deserialize_graph("testGraph"){
        _graph.nodes;//serialize("asdf");
    }
}

fn deserialize_graph(filename: &str) -> io::Result<Graph<&str>>{
    let file = File::open(filename)?;
    let mut reader: io::BufReader<File> = io::BufReader::new(file);

    let mut _graph: Graph<&str> = init_graph(&mut reader);
    fill_graph(&mut _graph, &mut reader)

    Ok(_graph)
}

fn init_graph(reader: &mut io::BufReader<File>)-> Graph<&'static str>{
    let mut nodes_count =String::new();
    reader.read_line(&mut nodes_count);
    nodes_count = nodes_count.trim().to_string();

    Graph::new(nodes_count.parse::<usize>().unwrap())
}

fn fill_graph(graph: &mut Graph<&str>, reader: &mut io::BufReader<File>){
    for line in reader.lines() {
        if let Ok(node_str) = line {
            // print!("{}", node_str);
            let mut _nodeInf: Vec<&str> = node_str.split(" ").collect();
            let data = _nodeInf[0];
            let node_id = graph.add_node(data);

            for n in 1.._nodeInf.len() {
                graph.add_link(node_id, _nodeInf[n].parse::<usize>().unwrap());
            }
        }
    }
}
