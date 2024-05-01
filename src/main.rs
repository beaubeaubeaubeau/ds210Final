mod functions;
mod graph;
use std::io;
use std::collections::VecDeque;

fn main() {
    //use functions and graph modules
    use graph::Graph;
    use functions::functions;

    //read file roadNet-TX.txt, which returns a vector of (usize, usize) tuples
    let file = functions::read_file("src/roadNet-TX.txt");
    //convert tuples into an adjacency list
    let graph = Graph::create_directed(file.len(), &file);
    //create VecDeque path for quick modification from front and back
    let mut path = VecDeque::new();

    println! ("Welcome to the Texas GPS!\n");
    //loop until otherwise specified
    loop {
        println! ("Options:\n'add' - add a new stop\n'next' - arrived at the next stop\n'remove' - remove a stop\n'avg' - compute the average distance of a stop in the path from all other stops\n'exit' - terminate GPS\n");
        println! ("Current path: {:?}", path);
        // add a semi-interactive current location output, for when the path isn't empty
        if path.len() > 0 {
            println! ("               ^");
            println! ("          Currently here\n");
        }
        // additionally, print the current distance of the nodes in the pat from each other, in sequence
        println! ("Remaining Distance: {:?}\n", functions::check_distance(&path, &graph.outedges));
        
        //ask for input based on options provided
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        // match statement depending on the input provided. invalid input will trigger another loop and a "Not a valid input" line
        match input {
            // if input is "add", call add_stop and perform different actions based on the return value
            "add" => {
                match functions::add_stop() {
                    Ok(result) => path.push_back(result),
                    Err(err) => println! ("{}", err),
                }
            },
            // if input is "next", pop out the node from the front of the path. 'let _' is because the program wouldn't work if the return value
            // of next wasn't ()
            "next" => {
                let _ = path.pop_front();
            },
            // if input is "remove", call remove_stop and perform different actions based on return value
            "remove" => {
                match functions::remove_stop(&mut path, ) {
                    Ok(result) => continue,
                    Err(err) => println! ("{}", err),
                }
            }
            // same as "add" and "remove" for "avg", this time calling compute_average
            "avg" => {
                match functions::compute_average(&mut path, &graph.outedges) {
                    Ok(result) => println! ("{}", result),
                    Err(err) => println! ("{}", err),
                }
            }
            // terminate the loop if the input is "break"
            "exit" => break,
            _ => println! ("Not a valid input!\n"),
        };
        println! (" -----------------------");
    }
}

// test functions for check_distance, which implements the breadth-first search. wasn't sure how to test for the other functions
// because they all require user input
#[test]
fn test_check_distance() {
    use functions::functions;
    use graph::Graph;

    let file = functions::read_file("src/roadNet-TX.txt");
    let graph = Graph::create_directed(file.len(), &file);

    let mut vec1 = VecDeque::new();

    let mut vec2 = VecDeque::new();
    vec2.push_back(0);
    vec2.push_back(1);

    let mut vec3 = VecDeque::new();
    vec3.push_back(134);
    vec3.push_back(217);
    vec3.push_back(489);

    assert_eq! (functions::check_distance(&vec1, &graph.outedges), 0);
    assert_eq! (functions::check_distance(&vec2, &graph.outedges), 1);
    assert_eq! (functions::check_distance(&vec3, &graph.outedges), 140);
}

#[test]
fn test_check_distance() {
    use functions::functions;
    use graph::Graph;

    let file = functions::read_file("src/roadNet-TX.txt");
    let graph = Graph::create_directed(file.len(), &file);

    let mut vec1 = VecDeque::new();

    let mut vec2 = VecDeque::new();
    vec2.push_back(0);
    vec2.push_back(1);

    let mut vec3 = VecDeque::new();
    vec3.push_back(134);
    vec3.push_back(217);
    vec3.push_back(489);

    assert_eq! (functions::check_distance(&vec1, &graph.outedges), 0);
    assert_eq! (functions::check_distance(&vec2, &graph.outedges), 1);
    assert_eq! (functions::check_distance(&vec3, &graph.outedges), 140);
}
