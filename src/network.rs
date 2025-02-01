use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::{Reverse,max};
use anyhow::anyhow;

use crate::node::Node;

/// A simple network, represented as an adjacency list.
///
#[derive(Debug,Clone)]
pub struct SimpleNetwork<N: Node = u64>{
    pub nodes: HashSet<N>,
    adjacencies: HashMap<N, HashSet<N>>,
    pub total_edges: u64,
}

impl<N: Node> SimpleNetwork<N> {
	pub fn from_node_vec(node_vec: Vec<N>) -> Self {
		Self {
			nodes: HashSet::from_iter(node_vec.clone().into_iter()),
			adjacencies: HashMap::from_iter(node_vec.clone().into_iter().map(|n| (n,HashSet::default()))),
			total_edges: 0,
		}
	}
	
	pub fn add_edge(&mut self, from: N, to: N) -> anyhow::Result<()>{
		self.adjacencies.get_mut(&from).ok_or(anyhow!("edge fail"))?.insert(to);
		self.total_edges += 1;
		Ok(())
	}
	
	/// Computes mean square-degree
	pub fn mean_square_degree(&self) -> f64{
		(self.adjacencies.clone().into_values().fold(0,|sum,x| sum+x.len().pow(2)) as f64)/(self.nodes.len() as f64)
	}
	
	/// SSSP using BFS for simple graphs
	///
	/// Starting from a given vertex, builds a `HashMap` keyed by vertex
	/// storing the distance from source to that vertex.  If a vertex is unreachable, 
	/// it is not in the map.
	/// 
	/// Should take O(V+E) time
	pub fn sssp(&self,from: N) -> HashMap<N, i64> {
		// Map to return, and queue (min heap) for next vertices to check
		let mut dists = HashMap::new();
		let mut queue = BinaryHeap::<(Reverse<i64>,N)>::new();
		dists.insert(from,0);
		queue.push((Reverse(0),from));
		while !queue.is_empty() {
			// Pop a vertex from the queue, then check for any neighbors we haven't seen
			// The distances are all +1 from previous vertex
			if let Some((Reverse(base_dist), next)) = queue.pop(){
				if let Some(adj) = self.adjacencies.get(&next){
					adj.iter().for_each(|node| if !dists.contains_key(node) {
												dists.insert(*node,base_dist+1);
												queue.push((Reverse(base_dist+1),*node));});
				}
			}
		}
		dists
	}
	
	/// Finds the largest connected component in a simple graph by
	/// checking SSSP repeatedly from different sources until all nodes
	/// have been found.
	/// 
	/// Returns a `HashSet` containing each node from that component.
	/// 
	/// Should take O(V+E) time in total
	pub fn largest_component(&self) -> HashSet<N> {
		// track unreached nodes and current largest component
		let mut unreached_nodes = self.nodes.clone();
		let mut largest = HashMap::new();
		while !unreached_nodes.is_empty() {
			let next_src = unreached_nodes.iter().next().unwrap().clone();
			let component = self.sssp(next_src);
			// remove the found nodes, then update largest
			component.clone().into_keys().for_each(|node| {unreached_nodes.remove(&node);});
			if component.len() > largest.len(){
				largest = component;
			}
		}
		// Convert `HashMap` (from SSSP) to `HashSet`
		largest.into_keys().collect::<HashSet<N>>()
	}
	
	/// Computes max and mean shortest paths (aka diameter and mean
	/// geodesic distance respectvely) of the largest connected component.
	/// Uses SSSP from each vertex rather than APSP as it should be faster/easier
	/// on simple graphs, despite the double counting. Additionally returns the
	/// size of the largest component used.
	/// 
	/// Should take O(V^2+VE) time
	pub fn mgd_diameter(&self) -> (f64,u64,u64) {
		let component = self.largest_component();
		// double fold summing over the whole adjacency matrix, but we get to throw out each row as we use it.
		let (mgd, diameter) = component.iter().fold((0.0,0), |(sum,maximum),node| {
			self.sssp(*node).into_values().fold((sum,maximum), |(sum_part,max_part),node| {
				(sum_part+(node as f64),max(max_part,node))
			})
		});
		// no factor of two here because each path was counted twice - a little misleading name but works
		let paths = component.len()*(component.len()-1);
		(mgd/(paths as f64),diameter as u64,component.len() as u64)
	}
	
}