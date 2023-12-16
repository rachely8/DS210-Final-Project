//Name:Rachel Young 
mod reader;
mod analysis;

fn main() {
    let file_path = "facebook_combined.txt";
    let n: usize = 4039; //Taken from dataset statistics 
    let edges = reader::read_file(file_path);
    let graph = analysis::Graph::create_undirected(n, &edges);

    //calculate the average distance between all nodes 
    let avg_distance = analysis::compute_average_distance(&graph);
    println!("The average distance between all pairs of nodes: {:.2}\n", avg_distance);

    //calculate degree centralities
    let network_centrality = analysis::compute_degree_centrality(&graph);

    //sort centralities in descending order 
    let mut sorted_nodes: Vec<_> = network_centrality.iter().collect();
    sorted_nodes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    //return the 10 nodes with the greatest centrality 
    println!("Top 10 users with the greatest degree centrality:");
    for (i, (&node, &centrality_value)) in sorted_nodes.iter().take(10).enumerate() {
        println!("{}. User {}:   Degree Centrality = {} friends", i + 1, node, centrality_value);
    }  
} 


//tests that the distance is greater than 0.0
#[test]
fn test_average_distance() {
    let test_n: usize = 10;
    let test_edges:  Vec<(usize, usize)> = vec![(0,1),(0,2),(1,2),(2,4),(2,3),(4,3),(4,5),(5,6),(4,6),(6,8),(6,7),(8,7),(1,9)];
    let test_graph = analysis::Graph::create_undirected(test_n,&test_edges);
    let test_avg_distance = analysis::compute_average_distance(&test_graph);
    assert!(test_avg_distance > 0.0,"Average cannot be calculated");
}

//tests if the centrality values were calculated correctly 
#[test]
fn test_degree_centrality() {
    let test_edges_2:  Vec<(usize, usize)> = vec![(0,1),(0,2),(1,2),(2,4),(2,3),(4,3),(4,5),(5,6),(4,6),(6,8),(6,7),(8,7),(1,9)];
    let graph1 = analysis::Graph::create_undirected(10, &test_edges_2);
    let test_centrality = analysis::compute_degree_centrality(&graph1);

    // Check that the degree centrality values calculated are the expected values. 
    assert_eq!(test_centrality[&0], 2.0);
    assert_eq!(test_centrality[&1], 3.0);
    assert_eq!(test_centrality[&2], 4.0);
    assert_eq!(test_centrality[&3], 2.0);
    assert_eq!(test_centrality[&4], 4.0);
    assert_eq!(test_centrality[&5], 2.0);
    assert_eq!(test_centrality[&6], 4.0);
    assert_eq!(test_centrality[&7], 2.0);
    assert_eq!(test_centrality[&8], 2.0);
    assert_eq!(test_centrality[&9], 1.0);
}






