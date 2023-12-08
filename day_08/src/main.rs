use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;
use std::usize;
use num_integer;

fn parse_content(str: &str) -> (Vec<usize>, HashMap<String, (String, String)>) {
    let mut directions: Vec<usize> = Vec::new();
    let mut graph: HashMap<String, (String, String)> = HashMap::new();

    let content: Vec<&str> = str.split("\n").filter(|x| !x.is_empty()).collect();
    let (indications, graph_str) = content.split_first().unwrap();
    for char_ in indications.chars() {
        match char_ {
            'L' => directions.push(0),
            'R' => directions.push(1),
            _ => (),
        };
    }
    for line in graph_str {
        let filtered_line = String::from(*line)
            .replace(" ", "")
            .replace("(", "")
            .replace(")", "");
        let splited_line: Vec<&str> = filtered_line.split("=").collect();
        let (father, sons) = splited_line.split_first().unwrap();
        let sons: Vec<&str> = sons[0].split(",").collect();
        // println!("Father: {father} , Sons: {:?}",sons);
        graph.insert(
            String::from(*father),
            (String::from(sons[0]), String::from(sons[1])),
        );
    }
    (directions, graph)
}

fn traverse_graph(directions: &Vec<usize>, graph: &HashMap<String, (String, String)>) -> usize {
    let mut index = 0;
    let mut next = String::from("AAA");
    let mut n_steps: usize = 0;
    loop {
        if next == "ZZZ" {
            return n_steps;
        }
        let r_o_l = directions[index];
        let candidates = graph.get(&next).unwrap();
        next = match r_o_l {
            0 => candidates.0.clone(),
            1 => candidates.1.clone(),
            _ => panic!("this value of r_o_l is not considered"),
        };
        n_steps += 1;
        // loop throught the directions vec
        index += 1;
        if index == directions.len() {
            index = 0;
        }
    }
}

fn step(direction: usize, node: &String, graph: &HashMap<String, (String, String)>) -> String {
    let next_node = graph.get(node).unwrap();
    let next_node: String = match direction {
        0 => next_node.0.clone(),
        1 => next_node.1.clone(),
        _ => panic!("this value of r_o_l is not considered"),
    };
    next_node
}

fn compute_state(node:&String, index:usize)->String{
        let mut state = node.clone();
        state.push_str(index.to_string().as_str());
        state
}

#[derive(Debug)]
struct CycleInfo {
    start: usize,
    starting_node: String,
    objective_nodes: Vec<(String, usize)>,
}

impl CycleInfo {
    fn locate_objective_nodes(
        &mut self,
        directions: &Vec<usize>,
        graph: &HashMap<String, (String, String)>,
    ) -> () {
        let mut index = self.start;
        let mut node = self.starting_node.clone();
        let mut steps: usize = 0;
        let initial_status = compute_state(&node, index);
        loop {
            node = step(directions[index], &node, graph);
            if node.ends_with("Z") {
                self.objective_nodes.push((node.clone(), steps));
            }
            steps += 1;
            index += 1;
            if index == directions.len() {
                index = 0;
            }
            if initial_status == compute_state(&node, index){
                return;
            }

        }
    }
}

fn find_cycle(
    directions: &Vec<usize>,
    initial_node: &String,
    graph: &HashMap<String, (String, String)>,
) -> CycleInfo {
    let mut states: HashMap<String, (usize, usize)> = HashMap::new();
    let mut n_steps = 0_usize;
    let mut index = 0_usize;
    let mut next_node = initial_node.clone();
    loop {
        let r_o_l = directions[index];
        let candidates = graph.get(&next_node).unwrap();
        next_node = match r_o_l {
            0 => candidates.0.clone(),
            1 => candidates.1.clone(),
            _ => panic!("this value of r_o_l is not considered"),
        };
        let state = compute_state(&next_node, index+1);
        if let Some(value) = states.insert(state.clone(), (index+1, n_steps)) {
            let mut cycle =  CycleInfo {
                start: value.0,
                starting_node: next_node,
                objective_nodes: Vec::new(),
            };
            cycle.locate_objective_nodes(&directions, &graph);
            return cycle;
            
        }
        n_steps += 1;
        index += 1;
        if index == directions.len() {
            index = 0;
        }
    }
}

fn check_cicles_in_multiple_graphs(
    directions: &Vec<usize>,
    graph: &HashMap<String, (String, String)>,
) -> Vec<CycleInfo>{
    let mut cycles_info: Vec<CycleInfo> = Vec::new();
    let mut initial_nodes: Vec<String> = Vec::new();
    for graph_key in graph.keys() {
        if graph_key.ends_with("A") {
            initial_nodes.push(graph_key.clone());
        }
    }

    for node in initial_nodes{
        let info = find_cycle(&directions, &node, graph);
        cycles_info.push(info);
    }
    cycles_info
}

fn compute_minimun_steps(cycles:&Vec<CycleInfo>)->u128{
    let mut max_displacement = 0;
    // compute max_displacement
    for cycle in cycles{
        if cycle.start > max_displacement{
            max_displacement=cycle.start;
        }
    }
    let mut values:Vec<u128> = Vec::new();
    for cycle in cycles{
        for node in & cycle.objective_nodes{
            values.push( (node.1 + cycle.start + 1 ) as u128);
        }
    }
    let mut solution = values[0];
    for i in 1..values.len(){
        solution = num_integer::lcm(solution, values[i]);

    };
    solution
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let (directions, graph) = parse_content(&content);
    let n_steps = traverse_graph(&directions, &graph);
    println!("Part1 N_steps: {n_steps}");
    let cycles = check_cicles_in_multiple_graphs(&directions, &graph);
    let count = compute_minimun_steps(&cycles);
    println!("Part2 N_steps: {count}");
}
