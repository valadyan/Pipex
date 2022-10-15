use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Graph <T>{
    nodes: Vec<GraphNode<T>>
}

impl<T> Graph<T> {
    pub fn new(node_count: usize) -> Self { 
        Self { nodes:  Vec::<GraphNode<T>>::with_capacity(node_count)} 
    }

    pub fn add_node(&mut self, data: T) -> usize {
        self.nodes.push(GraphNode::<T>::new(data, Vec::new()));
        self.nodes.len()
    }

    pub fn add_link(&mut self, from: usize, to: usize) {
        self.nodes[from].links.push(to);
    }

    pub fn add_links(&mut self, from: usize, links: &mut Vec<usize>) {
        self.nodes[from].links.append(links);
    }

    pub fn serialize(&mut self, filename: &str){
        let path = Path::new(filename);
        let display = path.display();
    
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        file.write(&self.nodes.len().to_le_bytes());
        file.write(b"\n");

        for node in &self.nodes{
            file.write(node.data.to_le_bytes());
            file.write(b"\n");

            for link in node.links{
                file.write(link.to_le_bytes());
                file.write(b" ");
            }

            file.write(b"\n");
        }

        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

struct GraphNode<T>{
    data: T,
    links: Vec<usize>
}

impl<T> GraphNode<T> {
    fn new(data: T, links: Vec<usize>) -> Self { Self { data, links } }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
