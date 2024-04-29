mod functions;
mod graph;
use std::io;

fn main() {
    use graph::Graph;
    use functions::functions;

    let file = functions::read_file("roadNet-TX.txt");
    let graph = Graph::create_directed(file.len(), &file);
    let mut roads: Vec<(usize,usize)> = vec! [];
    println! ("Welcome to the Texas Roads Tracker!\n");
    loop {
        println! ("Options:\n'add' - create a path\n'exit'- cancel\n");
        println! ("Current roads: {:?}", roads);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        match input {
            "add" => {
                match functions::add_edge() {
                    Ok(result) => roads.push(result),
                    Err(err) => println! ("{}", err),
                }
            },
            "exit" => break,
            _ => println! ("Not a valid input!\n"),
        }
        println! (" -----------------------\n");
        println! ("");
    }
}
