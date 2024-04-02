use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}
/*
This is a bit like the following Python code:

class Fighter:
    def __init__(self, name):
        self.name = name
*/
impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}


/// Calculates and prints the closeness centrality of each node in the graph.
/// 
/// The "closeness centrality (or closeness) of a node is a measure of centrality in a network, 
/// calculated as the reciprocal of the sum of the length of the shortest paths between the node 
/// and all other nodes in the graph. Thus, the more central a node is, the closer it is to all 
/// other nodes." - [Wikipedia](https://en.wikipedia.org/wiki/Closeness_centrality)
fn closeness_centrality(graph: &UnGraph<&Fighter, f32>, nodes: &[NodeIndex]) {

    //For each node, find the shortest path to all other nodes
    for node in nodes.iter() {
        let mut total_length = 0.0_f32;
        for other_node in nodes.iter() {
            if node == other_node {
                continue;
            }

            //Find the length of the shortest path between the two nodes
            let path = petgraph::algo::astar(&graph, *node, |n| n == *other_node, |e| *e.weight(), |_| 0.0);
            match path {
                Some((length, _)) => total_length += length,
                None => (),
            }

        }

        //Normalized form which represents the average length of all shortest paths
        let closeness = (nodes.len() - 1) as f32 / total_length;

        // Print the closeness centrality of the node
        println!("The closeness centrality of {} is {:.2}", &graph[*node].name, closeness);
    }
}


/// Calculates and prints the betweenness centrality of each node in the graph.
/// 
/// The "betweenness centrality" measures the number of times a node lies on the shortest path between other nodes. 
/// This measure shows which nodes are 'bridges' between nodes in a network. It does this by identifying all 
/// the shortest paths and then counting how many times each node falls on one. 
/// 
/// For more information see the UCLA Social Sciences exercise on [Betweenness centrality](https://www.sscnet.ucla.edu/soc/faculty/mcfarland/soc112/cent-ans.htm#:~:text=To%20calculate%20betweenness%20centrality%2C%20you,two%20nodes%20of%20the%20pair.)
fn betweenness_centrality(graph: &UnGraph<&Fighter, f32>, nodes: &[NodeIndex]) {

    //Create a vector to store the centrality score of each node
    let mut score: Vec<u8> = vec![0; nodes.len()];

    //For all pairs of nodes, calculate the shortest path between them
    //Since this is an undirected graph, we can ignore B->A if we have already calculated A->B
    for i in 0..nodes.len()-1 {
        for j in i+1..nodes.len() {

            //Find all paths between nodes[i] and nodes[j]
            let paths: Vec<Vec<NodeIndex>> = petgraph::algo::all_simple_paths(graph, nodes[i], nodes[j], 0, None).collect();
            
            //Find the path with the shortest length
            let min_length = paths.iter().map(|path| path.len()).min().unwrap();
            
            //If there are no paths with intermediate nodes, skip
            if min_length < 2 {
                continue;
            }
            
            //Find all the paths with the shortest length
            let min_paths: Vec<&Vec<NodeIndex>> = paths.iter().filter(|path| path.len() == min_length).collect();

            //Increment the centrality score of each the intermediate nodes in the shortest paths
            for path in min_paths {
                for node in path.iter().skip(1).take(min_length-2) {
                    score[node.index()] += 1;
                }
            }
        }
    }

    //Standardize the scores with a denominator of (n-1)(n-2)/2
    let denominator = (nodes.len()-1)*(nodes.len()-2)/2;
    for (i, &node) in nodes.iter().enumerate() {

        let name = &graph[node].name;
        let centrality = score[i] as f32 / denominator as f32;
        println!("The betweenness centrality of {} is {:.2}", name, centrality);
    }
}

/// Calculates and prints the degree centrality of each node in the graph.
/// 
/// Degree centrality assigns an importance score based simply on the number of 
/// links held by each node.
fn degree_centrality(graph: &UnGraph<&Fighter, f32>, nodes: &[NodeIndex]) {
    
    for node in nodes.iter() {

        let degree = graph.edges_directed(*node, Direction::Outgoing).count() as f32;
        let centrality = degree / (nodes.len() - 1) as f32;

        println!("The degree centrality of {} is {:.2}", &graph[*node].name, centrality);
    }
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    degree_centrality(&graph, &fighter_nodes);
    betweenness_centrality(&graph, &fighter_nodes);
    closeness_centrality(&graph, &fighter_nodes)
}
