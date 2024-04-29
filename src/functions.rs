pub mod functions{
    // using Graph from the graph module, File for file-reading capabilities
    use crate::graph::Graph;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;

    

    pub fn read_file(path: &str) -> Vec<(usize, usize)> {
        // takes a a string path and reads the associated .txt file. returns the length of the file and a vector of (usize, usize) tuples
        let mut result: Vec<(usize, usize)> = Vec::new();
        let file = File::open(path).expect("Could not open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        for line in buf_reader {
            let line_str = line.expect("Error reading");
            let v: Vec<&str> = line_str.trim().split('\t').collect();
            let from = v[0].parse::<usize>().unwrap();
            let to = v[1].parse::<usize>().unwrap();
            result.push((from, to));
        }
        return result;
    }

    pub fn add_edge() -> Result<(usize,usize), String> {
        println! ("\nPlease input the start node: ");
        let node1 = read_node();
        println! ("\nPlease input the end node: ");
        let node2 = read_node();

        if node1 < 0 || node2 < 0 {
            return Err("Nodes can't be negative!".to_string());
        }
        return Ok((node1, node2));
    } 

    fn read_node() -> usize {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let node: usize = input.parse().expect("Not a good number!");
        return node;
        
    }
}