use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::{Reverse,max};
use anyhow::anyhow;
use rand::Rng;


use crate::hw1::Adjacency;
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
	
	pub fn from_adj_list(adj_list: Vec<Adjacency<N>>) -> Self {
		Self {
			nodes: HashSet::from_iter(adj_list.clone().into_iter().map(|adj| adj.node_id)),
			adjacencies: HashMap::from_iter(adj_list.clone().into_iter().map(|adj| (adj.node_id,adj.edges.into_iter().collect::<HashSet<N>>()))),
			total_edges: adj_list.clone().into_iter().fold(0, |cnt, adj| cnt + adj.degree),
		}
	}
	
	pub fn add_edge(&mut self, from: N, to: N) -> anyhow::Result<()>{
		//dbg!(from,to);
		let from_set = self.adjacencies.get_mut(&from).ok_or(anyhow!("edge fail"))?;
		if !from_set.contains(&to) {
			self.total_edges += 1;
		}
		from_set.insert(to);
		
		let to_set = self.adjacencies.get_mut(&to).ok_or(anyhow!("edge fail"))?;
		if !to_set.contains(&from) {
			self.total_edges += 1;
		}
		to_set.insert(from);
		Ok(())
	}
	
	pub fn remove_edge(&mut self, from: N, to: N) -> anyhow::Result<()>{
		let from_set = self.adjacencies.get_mut(&from).ok_or(anyhow!("edge not present"))?;
		if from_set.contains(&to) {
			self.total_edges -= 2;
		}
		from_set.remove(&to);
		self.adjacencies.get_mut(&to).ok_or(anyhow!("edge not present"))?.remove(&from);
		Ok(())
	}
	
	//randomly rewire specified edge, maintaining from point
	pub fn rewire_edge(&mut self, from: N, to: N) -> anyhow::Result<()> {
		let from_set = self.adjacencies.get_mut(&from).ok_or(anyhow!("edge not present"))?;
		if from_set.contains(&to) {
			let mut new_dest = to;
			while from_set.contains(&new_dest){
				let rand_idx = rand::thread_rng().gen_range(0..self.nodes.len()) as usize;
				new_dest = self.nodes.clone().into_iter().collect::<Vec<N>>()[rand_idx];
			}
			from_set.remove(&to);
			from_set.insert(new_dest);
			self.adjacencies.get_mut(&to).ok_or(anyhow!("edge not present"))?.remove(&from);
			self.adjacencies.get_mut(&new_dest).ok_or(anyhow!("edge not present"))?.remove(&from);

		}
		Ok(())
	}
	
	/// Performs a random double edge swap on the graph
	/// 
	/// Ensures that the resulting graph is still a valid simple graph, and that 
	/// the degree of each node remains the same.
	pub fn double_edge_swap(&mut self) -> anyhow::Result<()>{
		let mut rewired = false;
		let mut flat_edges = vec![];
		self.adjacencies.clone().iter().for_each(|(node_1, edges)| {
			edges.clone().iter().for_each(|node_2| {
				//dbg!((*node_1,*node_2));
				flat_edges.push((*node_1,*node_2))
			})
		});
		//dbg!(flat_edges.len());
		//dbg!(self.total_edges);
		//dbg!("trying swap");
		while !rewired {
			//dbg!("trying again");
			let choice_1 = rand::thread_rng().gen_range(0..self.total_edges) as usize;
			let choice_2 = rand::thread_rng().gen_range(0..self.total_edges) as usize;
			let (u,v) = flat_edges[choice_1];
			let (x,y) = flat_edges[choice_2];
			if let Some(u_adj) = self.adjacencies.get(&u){
				if let Some(v_adj) = self.adjacencies.get(&v){
					if !u_adj.contains(&x) && !v_adj.contains(&y) && u != x && v != y{
						self.remove_edge(u,v)?;
						self.remove_edge(x,y)?;
						self.add_edge(u,x)?;
						self.add_edge(v,y)?;
						rewired = true;
					}
				}
			}
		}
		Ok(())
	}
	
	/// Computes mean square-degree
	pub fn mean_square_degree(&self) -> f64{
		(self.adjacencies.clone().into_values().fold(0,|sum,x| sum+x.len().pow(2)) as f64)/(self.nodes.len() as f64)
	}
	
	pub fn mean_degree_empirical(&self) -> f64{
		(self.adjacencies.clone().into_values().fold(0,|sum,x| sum+x.len()) as f64)/(self.nodes.len() as f64)
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
	
	//may need APSP for some stats?
	
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
	
	/// Computes the clustering coefficient for the graph
	pub fn cluster_coeff(&self) -> f64 {
		let (triads,triangles) = self.adjacencies.clone().into_values().fold((0,0),|(triad_count,triangle_count),edges| {
			(triad_count + edges.len()*(edges.len()-1), edges.clone().iter().fold(triangle_count,|prev_count,node_1| {
				if let Some(next_steps) = self.adjacencies.get(node_1) {
					prev_count + edges.intersection(next_steps).count()
				} else {
					prev_count
				}
			}))
		});
		(triangles as f64)/(triads as f64)
	}
	
	pub fn harmonic_centrality(&self) -> HashMap<N,f64> {
		let mut result = HashMap::new();
		for node in self.nodes.clone().iter() {
			let total = self.sssp(*node).into_values().fold(0.0,|prev,d| if(d != 0){prev + (1.0/(d as f64))} else {prev});
			result.insert(*node,total/(self.nodes.len() as f64-1.0));
		}
		result
	}
	
	//also return paths
	pub fn sssp_verbose(&self,from: N) -> HashMap<N, Vec<N>> {
		// Map to store in-progress paths, from all nodes
		let mut paths = HashMap::new();
		for node in self.nodes.clone() {
			paths.insert(node, vec![]);
		}
		// Map to return, and queue (min heap) for next vertices to check
		let mut dists = HashMap::new();
		let mut queue = BinaryHeap::<(Reverse<i64>,N)>::new();
		dists.insert(from,vec![]);
		queue.push((Reverse(0),from));
		while !queue.is_empty() {
			// Pop a vertex from the queue, then check for any neighbors we haven't seen
			// The distances are all +1 from previous vertex
			if let Some((Reverse(base_dist), next)) = queue.pop(){
				if let Some(adj) = self.adjacencies.get(&next){
					adj.iter().for_each(|node| if !dists.contains_key(node) {
												if let Some(pvec) = paths.get_mut(&node){
													pvec.push(next);
												}
												if let Some(pvec) = paths.get(&node){
													dists.insert(*node,pvec.clone());

												}
												queue.push((Reverse(base_dist+1),*node));});
				}
			}
		}
		dists
	}
	
	pub fn betweenness_centrality(&self) -> HashMap<N,u64> {
		let mut result = HashMap::new();
		for node in self.nodes.clone() {
			result.insert(node, 0);
		}
		//check per node's sssp
		for node in self.nodes.clone().iter() {
			self.sssp_verbose(*node).into_values().for_each(|path| path.iter()
												.for_each(|i| if let Some(count) = result.get_mut(&i){
																		*count = &*count+1;
																		}));
		}
		result
		
	}
	
}