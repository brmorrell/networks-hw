//TODO: add documentation
use crate::hw1::{AttrNode,Edge};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::SimpleNetwork;
use std::time::Instant;
use crate::node::Node;

pub fn log_likelyhood_DCSBM(mixing_matrix: &Vec<Vec<usize>>, grp_degrees: &Vec<usize>) -> f64 {
	let mut log_likelyhood = 0.0;
	for r in 0..mixing_matrix.len() {
		let kr = grp_degrees[r];
		for s in 0..mixing_matrix.len() {
			let ks = grp_degrees[s];
			let wrs = mixing_matrix[r][s];
			if wrs != 0 {
				log_likelyhood += (wrs as f64)*(((wrs as f64)/(kr as f64)/(ks as f64)).ln());
			}
		}
	}
	log_likelyhood
}

pub fn compute_mixing_matrix<N:Node>(G: &SimpleNetwork<N>, c: usize, partition: &HashMap<N,(usize,bool)>) -> Vec<Vec<usize>> {
	let mut matrix = vec![vec![0; c]; c];
	for node in &G.nodes {
		if let Some((grp,_)) = partition.get(&node){
			if let Some(adjs) = G.get_adjs(*node){
				for neighbor in adjs {
					if let Some((grp2,_)) = partition.get(neighbor){
						matrix[*grp][*grp2] += 1;
					}
				}
			}
		}
	}
	matrix
}

//can make this faster if we don't actually move each node
pub fn makeAMove<N:Node>(G: &SimpleNetwork<N>, partition: &HashMap<N,(usize,bool)>, num_groups: usize) -> (f64,(N,usize)) {
	let mut max_likelyhood = 0.0;
	let mut best_move = (partition.clone().into_keys().collect::<Vec<N>>()[0],0);
	let mut test_partition = partition.clone();
	for (node,(grp,frozen)) in partition {
		if !frozen {
			//compute changes to mixing matrix by group
			for dst in 0..num_groups {
				if dst != *grp {
					//calculate log-likelyhood of moving node i to grp
					test_partition.insert(*node, (dst,false));
					let mixing_matrix = compute_mixing_matrix(G, num_groups, &test_partition);
					let mut grp_degrees = vec![];
					for grp in &mixing_matrix {
						grp_degrees.push(grp.iter().sum());
					}
					let log_likelyhood = log_likelyhood_DCSBM(&mixing_matrix,&grp_degrees);
					if max_likelyhood == 0.0 || log_likelyhood > max_likelyhood {
						max_likelyhood = log_likelyhood;
						best_move = (*node,dst);
					}
				}
			}
			test_partition.insert(*node, (*grp,false));
		}
	}
	
	(max_likelyhood, best_move)
}

pub fn runOnePhase<N:Node>(G: &SimpleNetwork<N>, mut partition: HashMap<N,(usize,bool)>, num_groups: usize) -> (HashMap<N,(usize,bool)>, f64, bool, Vec<f64>){
	let mut mixing_matrix = compute_mixing_matrix(G, num_groups, &partition);
	let mut grp_degrees = vec![];
	for grp in &mixing_matrix {
		grp_degrees.push(grp.iter().sum());
	}
	
	let mut likelyhoods = vec![];
	let mut z0_likelyhood = log_likelyhood_DCSBM(&mixing_matrix,&grp_degrees);
	let mut halt = true;
	let mut best_partition = partition.clone();
	let mut max_likelyhood = z0_likelyhood;
	likelyhoods.push(z0_likelyhood);
	
	for t in 0..G.nodes.len() {
		let (next_likelyhood,(node,dst)) = makeAMove(G,&partition, num_groups);
		partition.insert(node, (dst,true));
		if let Some(neighborhood) = G.get_adjs(node) {
			let k_node = neighborhood.len();
			if let Some((grp,_)) = partition.get(&node) {
				grp_degrees[*grp] -= k_node;
				grp_degrees[dst] += k_node;
				for neighbor in neighborhood {
					if let Some((neighbor_grp,_)) = partition.get(&neighbor) {
						mixing_matrix[*grp][*neighbor_grp] -= 1;
						mixing_matrix[*neighbor_grp][*grp] -= 1;
						mixing_matrix[dst][*neighbor_grp] += 1;
						mixing_matrix[*neighbor_grp][dst] += 1;
					}
				}
			}
		}
		likelyhoods.push(next_likelyhood);
		if next_likelyhood > max_likelyhood {
			halt = false;
			max_likelyhood = next_likelyhood;
			best_partition = partition.clone();
		}
	}
	
	(best_partition, max_likelyhood, halt, likelyhoods)
}

pub fn fitDCSBM<N:Node>(G: SimpleNetwork<N>, c: usize, T: usize) -> (HashMap<N,(usize,bool)>, f64, Vec<f64>){
	//pick random initial partition z0
	let mut rng = rand::thread_rng();
	let mut partition = HashMap::new();
	for node in &G.nodes {
		partition.insert(*node, (rng.gen_range(0..c), false));
	}
	let mut likelyhoods = vec![];
	
	for p in 0..T {
		let (new_partition, phase_likelyhood, halt, mut phase_likelyhoods) = runOnePhase(&G, partition.clone(), c);
		for (node,(grp,frozen)) in &new_partition {
			partition.insert(*node,(*grp,false));
		}
		
		likelyhoods.append(&mut phase_likelyhoods);
		if halt || p == T-1 {
			return (new_partition, phase_likelyhood, likelyhoods);
		}
	} 
	//This should never happen
	(partition,0.0,likelyhoods)
	
}





