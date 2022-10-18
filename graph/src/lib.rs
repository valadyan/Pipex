use std::collections::HashMap;
use std::fs::{File, self};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::fmt::Display;

pub struct Graph <T>{
    nodes: HashMap<String, GraphNode<T>>
}

struct GraphNode<T>{
    data: T,
    links: Vec<String>
}

impl<T> GraphNode<T> {
    fn new(data: T) -> Self { Self { data, links: Vec::new() } }
}

impl<T> Graph<T> {
    pub fn new(node_count: usize) -> Self { 
        Self { nodes : HashMap::new() } 
    }

    pub fn add_node(&mut self, node_id: String, node_data: T){
        self.nodes.insert(node_id.to_string(), GraphNode::new(node_data));
    }

    pub fn add_link(&mut self, from: String, to: String) {
        match self.nodes.get_mut(&from.to_string()) {
            None =>{},
            Some(node) => node.links.push(to.to_string())
        }
    }
}

impl<T: Display> Graph<T> {
    pub fn serialize(&self, filename: &str){
        let path = Path::new(filename);
        let display = path.display();
    
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        for (id, node) in &self.nodes{
            let node_data = node.data.to_string();
            write!(file, "{id} {node_data}\n");
        }
        
        file.write(b"#\n");

        for (id, node) in &self.nodes{
            for link in &node.links{
                write!(file, "{id} {link}\n");
            }
        }
    }

    pub fn deserialize(filename: &str) -> io::Result<Graph<String>>{
        let file = fs::File::open(filename)?;
        let mut reader: io::BufReader<File> = io::BufReader::new(file);
    
        let mut _graph: Graph<String> = Self::init_graph(&mut reader);
        Self::fill_nodes(&mut _graph, &mut reader);
    
        Ok(_graph)
    }
    
    fn init_graph(reader: &mut io::BufReader<File>)-> Graph<String>{
        let mut nodes_count =String::new();
        reader.read_line(&mut nodes_count);
        let t = nodes_count.trim();
    
        Graph::new(nodes_count.parse::<usize>().unwrap())
    }
    
    fn fill_nodes(graph: &mut Graph<String>, reader: &mut io::BufReader<File>){
        // let mut buf = vec![];
        // reader.read_until(b" ", &mut buf);
        // graph.add_node(buf);

        // for line in reader.lines() {
        //     if let Ok(node_str) = line {
        //         let mut _nodeInf: Vec<&str> = node_str.split(" ").collect();
        //         let data = _nodeInf[0].to_string();
        //         let node_id = graph.add_node(data);
                
        //         if _nodeInf.len() < 2 {continue;}

        //         let links = &_nodeInf[1..].to_vec();
        //         for n in 1.._nodeInf.len() {
        //             graph.add_link(node_id, _nodeInf[n].parse::<usize>().unwrap());
        //         }
        //     }
        // }
    }


}