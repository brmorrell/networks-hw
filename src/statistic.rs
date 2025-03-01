//TODO: add documentation
use crate::hw1::{AttrNode,Edge};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::SimpleNetwork;
use std::time::Instant;

pub fn remove_attrs(nodes: Vec<AttrNode>, alpha: f64) -> Vec<AttrNode> {
	let num_observed = ((nodes.len() as f64)*alpha) as usize;
	
	let mut numbers: Vec<usize> = (0..nodes.len()).collect();
 	numbers.shuffle(&mut rand::thread_rng());
 	let usable_numbers = &numbers[0..num_observed];
 	
 	let temp = nodes.clone();
 	let new_nodes = temp.iter().enumerate().map(|(i,node)| {
			if usable_numbers.contains(&i) {
				*node
			} else {
				AttrNode {node_id: node.node_id, attr: -1}
			}
		});
	new_nodes.collect()
	
}

pub fn infer_attrs(network: SimpleNetwork<AttrNode>, baseline: Vec<i32>) -> Vec<AttrNode> {
	let mut guessed_nodes: Vec<AttrNode> = network.nodes.clone().iter().map(|&x| x).collect();
	
	for node in &mut guessed_nodes {
		if node.attr == -1 {
			if let Some(neighbors) = network.get_adjs(*node){
				let mut counts: HashMap<i32,usize> = HashMap::new();
				neighbors.iter().map(|&x| x.attr).filter(|&x| x >= 0).for_each(|x| {*counts.entry(x).or_default() += 1;});
				
				//take most common, break ties randomly
				if counts.len() > 0 {
					let max_cnt = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(_, v)| v);
					if max_cnt.is_some() {
						let guesses: Vec<i32> = counts.iter().filter(|(_,&v)|  v == *max_cnt.unwrap()).map(|(&k, _)| k).collect();
						node.attr = guesses[rand::thread_rng().gen_range(0..guesses.len())]
					}
				} else {
					node.attr = baseline[rand::thread_rng().gen_range(0..baseline.len())];
				}
			}
		}
	}
	
	guessed_nodes
}

pub fn remove_edges(edges: Vec<Edge>, alpha: f64) -> Vec<Edge> {
	let num_observed = ((edges.len() as f64)*alpha) as usize;
	
	let mut numbers: Vec<usize> = (0..edges.len()).collect();
 	numbers.shuffle(&mut rand::thread_rng());
 	let usable_numbers = &numbers[0..num_observed];
 	
 	let temp = edges.clone();
 	let new_edges = temp.iter().enumerate().filter(|(i,_)| usable_numbers.contains(&i))
 											.map(|(_,edge)| *edge);
	new_edges.collect()
	
}


pub fn jaccard_scores(network: SimpleNetwork<AttrNode>) -> HashMap<(AttrNode, AttrNode), f64> {
	let pairs = network.get_nonedges();
	let mut scores = HashMap::new();
	for (src,dst) in pairs {
		//dbg!("doing a pair");
		if let Some(src_nbrs) = network.get_adjs(src){
			if let Some(dst_nbrs) = network.get_adjs(dst){
				let union_size = src_nbrs.union(dst_nbrs).collect::<HashSet<_>>().len();
				let inter_size = src_nbrs.intersection(dst_nbrs).collect::<HashSet<_>>().len();
				//random noise - 1/n will never reorder scores
				let noise = rand::thread_rng().gen::<f64>()/(network.nodes.len() as f64);
				if union_size > 0 {
					scores.insert((src,dst),(inter_size as f64)/(union_size as f64) + noise);
				} else {
					scores.insert((src,dst),noise);
				}
			}
		}
	}
	scores
}


pub fn dp_scores(network: SimpleNetwork<AttrNode>) -> HashMap<(AttrNode, AttrNode), f64> {
	let pairs = network.get_nonedges();
	let mut scores = HashMap::new();
	for (src,dst) in pairs {
		if let Some(src_nbrs) = network.get_adjs(src){
			if let Some(dst_nbrs) = network.get_adjs(dst){
				let src_k = src_nbrs.len();
				let dst_k = dst_nbrs.len();
				//random noise - 1/2 will never reorder scores
				let noise = rand::thread_rng().gen::<f64>()/2.0;
				scores.insert((src,dst),(src_k*dst_k) as f64 + noise);
			}
		}
	}
	scores
}

pub fn sp_scores(network: SimpleNetwork<AttrNode>) -> HashMap<(AttrNode, AttrNode), f64> {
	let pairs = network.get_nonedges();
	//precompute apsp
	let paths = network.apsp();
	
	let mut scores = HashMap::new();
	for (src,dst) in pairs {
		//random noise - 1/n will never reorder scores
		let noise = rand::thread_rng().gen::<f64>()/(network.nodes.len() as f64);
		let path_length = paths[src.node_id as usize][dst.node_id as usize];
		if path_length >= 0{
			scores.insert((src,dst),1.0/(path_length as f64) + noise);
		} else {
			scores.insert((src,dst),noise);
		}
	}
	scores
}

pub fn roc(scores: HashMap<(AttrNode, AttrNode), f64>, edges: HashSet<(AttrNode,AttrNode)>) -> Vec<(f64,f64)> {
	let mut scores_aug = vec!();
	let mut curve = vec!();
	let mut T = 0;
	let mut F = 0;
	//let split = Instant::now();
	for (pair,score) in scores {
		if edges.contains(&pair) {
			scores_aug.push((pair,score,true));
			T+=1;
		} else {
			scores_aug.push((pair,score,false));
			F+=1;
		}
	}
	//dbg!(split.elapsed());
	scores_aug.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
	let mut running_T = 0;
	let mut running_F = 0;
	curve.push((0.0,0.0));
	//dbg!(split.elapsed());
	for i in 0..scores_aug.len() {
		if scores_aug[i].2{
			running_T+=1;
		} else {
			running_F+=1;
		}
		let tpr = if T > 0 {(running_T as f64)/(T as f64)} else {0.0};
		let fpr = if F > 0 {(running_F as f64)/(F as f64)} else {0.0};
		curve.push((tpr,fpr));
	}
	//dbg!(split.elapsed());
	curve
}





