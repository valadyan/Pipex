use std::collections::HashMap;
use std::fs::{File, self};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::fmt::{Display};
use std::error::Error;

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

impl<T: Display> Graph<T> {
    pub fn new() -> Self { 
        Self { nodes : HashMap::new() } 
    }

    pub fn add_node(&mut self, node_id: &str, node_data: T){
        self.nodes.insert(node_id.to_string(), GraphNode::new(node_data));
    }

    pub fn add_link(&mut self, from: &str, to: &str) {
        match self.nodes.get_mut(&from.to_string()) {
            None =>{},
            Some(node) => node.links.push(to.to_string())
        }
    }

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
        let reader: io::BufReader<File> = io::BufReader::new(file);
    
        let mut graph = Graph::<String>::new();
        let mut read_links = false;

        for line_res in reader.lines() {
            match line_res {
                Err(mes) => return Err(mes),
                Ok(line) => {
                    Self::is_reader_read_links(&mut read_links, &line);

                    if read_links {
                        Self::parse_links(&mut graph, &line);
                    } else {
                        Self::parse_nodes(&mut graph, &line);
                    }
                }
            }
        }

        Ok(graph)
    }

    fn is_reader_read_links(read_links: &mut bool, file_line: &str){
        if !*read_links && file_line.starts_with("#") {
            *read_links = true;
        }
    }

    fn parse_nodes(graph: &mut Graph<String>, node_info: &str){
        let node_id_bound = node_info.find(char::is_whitespace).expect("incorrect graph file");
        let node_id = &node_info[..node_id_bound];
        let mut node_data = "";
        if node_id_bound < node_info.len() {
            node_data = &node_info[node_id_bound+1..];
        } 
        graph.add_node(node_id, node_data.to_owned());
    }
    
    fn parse_links(graph: &mut Graph<String>, link_info: &str){
        let from_to: Vec<&str> = link_info.split_whitespace().collect();
        if from_to.len() == 2 {
            graph.add_link(from_to[0], from_to[1]);
        }
    }
}