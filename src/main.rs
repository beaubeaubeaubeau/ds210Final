mod functions;
mod graph;
use std::io;
use std::collections::VecDeque;

fn main() {
    use graph::Graph;
    use functions::functions;

    let file = functions::read_file("src/roadNet-TX.txt");
    let graph = Graph::create_directed(file.len(), &file);
    let mut path = VecDeque::new();
    println! ("Welcome to the Texas GPS!\n");
    loop {
        println! ("Options:\n'add' - add a new stop\n'exit'- terminate GPS\n");
        println! ("Current path: {:?}", path);
        println! ("Remaining Distance: {:?}", functions::check_distance(&path, &graph.outedges));
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        match input {
            "add" => {
                match functions::add_stop() {
                    Ok(result) => path.push_back(result),
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
