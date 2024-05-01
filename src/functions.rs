pub mod functions{
    // using Graph from the graph module, File for file-reading capabilities
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;
    use std::collections::VecDeque;
    

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

    pub fn add_stop() -> Result<usize, String> {
        println! ("\nPlease input the node #: ");
        let node = read_node();

        if node > 3843320 {
            return Err("Node outside range!".to_string());
        }

        return Ok(node);
    } 

    pub fn remove_stop(path: &mut VecDeque<usize>) -> Result<usize, String> {
        println! ("\nPlease input the node #: ");
        let node = read_node();

        if node > 3843320 {
            return Err("Node outside range!".to_string());
        }

        if let Some(index) = path.iter().position(|&x| x == node) {
            path.remove(index);
            return Ok(0);
        } else {
            return Err("Node not found in path!".to_string());
        }
    }

    pub fn check_distance(path: &VecDeque<usize>, outedges: &Vec<Vec<usize>>) -> u64 {
        if path.len() == 0 || path.len() == 1 {
            return 0;
        }

        let mut dist = 0;

        for i in 0..path.len()-1 {
            let node1 = path.get(i).unwrap();
            let node2 = path.get(i+1).unwrap();
            dist += bfs(node1, node2, &outedges)
        }

        return dist;
    }

    pub fn compute_average(path: &VecDeque<usize>, outedges: &Vec<Vec<usize>>) -> Result<f64, String> {
        println! ("\nPlease input the node #: ");
        let node = read_node();

        if node > 3843320 {
            return Err("Node outside range!".to_string());
        }
        else if !path.contains(&node) {
            return Err("Node not in path".to_string());
        }
        
        let mut sum = 0;
        for i in path {
            sum += bfs(&node, i, outedges);
        }

        return Ok((sum as f64) / ((path.len()-1) as f64))
    }

    fn bfs(start: &usize, end: &usize, outedges: &Vec<Vec<usize>>) -> u64{
        let mut distance: Vec<Option<u64>> = vec![None;outedges.len()];
        distance[*start] = Some(0);
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(*start);
        while let Some(v) = queue.pop_front() { 
            for u in outedges[v].iter() {
                if let None = distance[*u] { 
                    distance[*u] = Some(distance[v].unwrap() + 1);
                    queue.push_back(*u);
                }
            }
        }
        return distance[*end].expect("Failed to read distance");
    }

    fn read_node() -> usize {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let node: usize = input.parse().expect("Not a good number!");
        println!();
        return node;
        
    }

}