use std::fs::{read_dir, File};
use std::time::Instant;
use std::path::{Path,PathBuf};
use anyhow::anyhow;
use std::collections::{HashSet,HashMap};
use rand::Rng;


use hw5352::{hw1::parse_edges, hw1::parse_nodes, hw1::AttrNode, hw1::parse_basic_nodes, hw1::parse_attr_nodes, hw1::parse_adjacency_list, output::to_csv, SimpleNetwork, statistic::remove_attrs, statistic::infer_attrs, statistic::remove_edges, statistic::jaccard_scores, statistic::dp_scores, statistic::sp_scores, statistic::roc};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    w_hw_num: u32,

    #[arg(short, long, default_value_t=format!(""))]
    name: String,

    #[clap(long, short, action)]
    folder: bool,

    #[clap(long, short, action)]
    degree: bool,

    #[clap(long, short, action)]
    length: bool,
    
    #[clap(long, short, action)]
    attr_inf: bool,
    
    #[clap(long, short, action)]
    edge_inf: bool,
    
    #[clap(long, short, action)]
    cnull_model: bool,
    
    #[clap(long, short, action)]
    strogatz: bool,
}
//TODO: add documentation
//good luck reading it until then

/// Call with: cargo run [--release] -- [HW1] [options]
/// Can also use cargo run -- --help to view full auto-generated
/// options list as they might not all be here
/// Universal Options:
///		[-n]  			file/folder name (no extension or "_attr")
///		[-f]			specifies to run on every file within target folder, rather than on a single data set
/// W0 Options:			(watts-strogatz)
/// W1 Options:
///		[-d]			compute degree measures
///		[-l]			compute distance measures
/// 	[-c]			config model rewiring, gather stats at each step
/// W2 Options:
/// 	[-c]			generate null model for C and mgd using config model
/// 	[-a]			delete and infer node attributes
/// 	[-e]			edge inference
/// W3 Options:			(adjacency list; harmonic centrality)

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let args = Args::parse();
    
    if args.w_hw_num == 0 {
		let max_nodes = 100;
		let node_list: Vec<u64> = (0..max_nodes).collect();
		let mut network_base: SimpleNetwork<u64> = SimpleNetwork::from_node_vec(node_list.clone());
		for i in &node_list {
			network_base.add_edge(*i,(*i+1)%max_nodes );
			network_base.add_edge(*i,(*i+2)%max_nodes );

		}
		let p_vals = 0..101;
		for p in p_vals {
			//rewire edges to new connections
			let mut network = network_base.clone();
			for i in &node_list{
				if rand::thread_rng().gen_range(1..101) <= p{
					network.rewire_edge(*i,(*i+1)%max_nodes);
				}
				if rand::thread_rng().gen_range(1..101) <= p{
					network.rewire_edge(*i,(*i+2)%max_nodes);
				}
			}
			//betweenness centrality
			let btn_c = network.betweenness_centrality();
			let name = "WStest";
			let btn_vec = btn_c.into_values().collect::<Vec<u64>>();
			let data_file = File::options()
                    .append(true)
                    .create(true)
                    .open("src/output/hw2_ws_data.csv")?;
                    
            to_csv(&name, &[(p as f64)/100.0], &btn_vec, data_file)?;
            println!(".");
			
		}
		
    } else if args.w_hw_num == 1 {
        let mut node_data = vec![];
        let mut edge_data = vec![];
        let mut filename_nodes = format!("src/data/{}_attr.txt", args.name);
		let mut filename_edges = format!("src/data/{}.txt", args.name);
		// filepath management, just getting all filenames from the folder
		// should be in same position in vecs, unless something fails weirdly
		// does assume that any *_attr.txt file has a corresponding *.txt
        if args.folder {
            let foldername = &format!("src/data/{}", args.name);
            let paths = read_dir(foldername).unwrap();

            for entry in paths {
				let path_buf = entry?.path();
				let path_str = path_buf.to_str().ok_or(anyhow!("non-utf8 path"))?;
                if path_str.contains("_attr.txt") {
					let edge_str = path_str.replace("_attr.txt",".txt");
                    node_data.push(path_str.to_owned());
                    edge_data.push(edge_str);
                }
            }
        } else{
			//dummy values so the for loop happens
			node_data.push(filename_nodes.clone());
			edge_data.push(filename_edges.clone());
		}
        for i in 0..node_data.len() {
			let split = Instant::now();
			if args.folder {
				filename_nodes = node_data[i].clone();
				filename_edges = edge_data[i].clone();
			}
			println!("Opening {}",filename_nodes.clone());
            println!("Opening {}",filename_edges.clone());
            let nodes_file = File::open(filename_nodes.clone())?;
            let edges_file = File::open(filename_edges.clone())?;
            
            let path = Path::new(&filename_edges);
            let name = path.file_stem().unwrap().to_str().unwrap();

            let nodes = parse_nodes(nodes_file)?;
            let edges = parse_edges(edges_file)?;
            //dbg!(nodes.clone());
            //dbg!(edges.clone());

            let node_list: Vec<u64> = nodes.into_iter().map(|n| n.node_id).collect();
            let mut network: SimpleNetwork<u64> = SimpleNetwork::from_node_vec(node_list);
            for edge in edges {
                network.add_edge(edge.from, edge.to)?;
            }
            //dbg!(network.clone());
            //The fast stuff - mean degree and mean squared degree
            if args.degree {
                let degree_file = File::options()
                    .append(true)
                    .create(true)
                    .open("src/output/hw1_degree_data.csv")?;
                //dbg!("output one open");

                let mean_degree = (network.total_edges as f64) / (network.nodes.len() as f64);
                let mean_square_degree = network.mean_square_degree();

                //output to hw1_degree_data.csv
                //also recording number of nodes and edges for sanity checks
                to_csv(&name, &[mean_degree, mean_square_degree], &[network.total_edges/2,network.nodes.len() as u64],degree_file)?;
                println!(".");
            }
            //mgd, diameter, and largest connected component
			//This takes a long time to run - about 5.5 hours for the facebook100 dataset
            if args.length {
                let distance_file = File::options()
                    .append(true)
                    .create(true)
                    .open("src/output/hw1_distance_data.csv")?;
                //dbg!("output two open");

                let (mgd,diameter,size) = network.mgd_diameter();

                //output to hw1_distance_data.csv
                to_csv(&name, &[mgd], &[diameter, size], distance_file)?;
                println!(".");
            }
            
            if args.cnull_model {
				let steps = 10*network.total_edges;
                let config_berkeley_1 = File::options()
                    .append(true)
                    .create(true)
                    .open("src/output/hw2_berkeley_data.csv")?;
                let (mgd_actual,_,_) = network.mgd_diameter();
				let cluster_actual = network.cluster_coeff();
				//output to hw2_p4_data.csv
				//dbg!(network.clone());
               	to_csv(&name, &[cluster_actual,mgd_actual], &[0], config_berkeley_1)?;
               	let ratio = f64::powf(steps as f64, 1.0 / 25.0);
               	let measure_steps = (1..101).map(|x| ratio.powi(x) as u64).collect::<Vec<u64>>();
				for i in 0..steps {
					//double edge swap on network
					network.double_edge_swap()?;
					
					if measure_steps.contains(&i){
						let config_berkeley = File::options()
		                    .append(true)
		                    .create(true)
		                    .open("src/output/hw2_berkeley_data.csv")?;
						
						//measure C and mgd
						let (mgd,_,_) = network.mgd_diameter();
						let cluster = network.cluster_coeff();
						//output to hw2_p4_data.csv
						//dbg!(network.clone());
	               		to_csv(&name, &[cluster,mgd], &[i], config_berkeley)?;
	               		dbg!(".");
               		}
               	}
                println!(".");
			}
            
            let partial_time = split.elapsed();
            let elapsed = now.elapsed();
            println!("Split: {:.2?}, Total: {:.2?}", partial_time, elapsed);
        }
    } else if args.w_hw_num == 2 {
		//Need to check if 1 or 2 files, then read in (basic data)
        let mut node_data = vec![];
        let mut edge_data = vec![];
        let mut filename_nodes = format!("src/data/{}_nodes.txt", args.name);
		let mut filename_edges = format!("src/data/{}.txt", args.name);
		// filepath management, just getting all filenames from the folder
		// should be in same position in vecs, unless something fails weirdly
		// does assume that any *_attr.txt file has a corresponding *.txt
        if args.folder {
            let foldername = &format!("src/data/{}", args.name);
            let paths = read_dir(foldername).unwrap();

            for entry in paths {
				let path_buf = entry?.path();
				let path_str = path_buf.to_str().ok_or(anyhow!("non-utf8 path"))?;
				
				if let Some(extension) = path_buf.extension(){
	                if extension == "txt" && !path_str.contains("_nodes.txt") {
						let node_str = &path_str.replace(".txt","_nodes.txt");
						let mut candidate = PathBuf::new();
						candidate.set_file_name(&node_str);
						if candidate.exists() {
							node_data.push(node_str.to_owned());
						} else {
	 					   node_data.push(format!(""));
						}
	
	                    edge_data.push(path_str.to_owned());
	                }
                }
            }
        } else{
			//dummy values so the for loop happens
			node_data.push(filename_nodes.clone());
			edge_data.push(filename_edges.clone());
		}
        for i in 0..node_data.len() {
			let split = Instant::now();
			if args.folder {
				filename_nodes = node_data[i].clone();
				filename_edges = edge_data[i].clone();
			}

    		
            
            //doing hw3 node attribute inference
			if args.attr_inf {
				println!("Opening {}",filename_edges.clone());
	            let edges_file = File::open(filename_edges.clone())?;
	            
	            let path = Path::new(&filename_edges);
	            let name = path.file_stem().unwrap().to_str().unwrap();
	            
	            let edges = parse_edges(edges_file)?;
	
				println!("Opening {}",filename_nodes.clone());
				let nodes_file = File::open(filename_nodes.clone())?;
				let mut nodes = parse_attr_nodes(nodes_file)?;
				nodes.sort_by(|a,b| a.node_id.cmp(&b.node_id));
	            //dbg!(nodes.clone());
	            //dbg!(edges.clone());
	            
	            let alpha_unit = 50.0;
	            for alpha in 0..50 {
					let alpha_split = Instant::now();
					for rep in 0..50 {
						let nodes_redacted = remove_attrs(nodes.clone(),(alpha as f64)/alpha_unit);
						//dbg!(nodes_redacted.clone());
						let mut network: SimpleNetwork<AttrNode> = SimpleNetwork::from_node_vec(nodes_redacted.clone());
		            	for edge in &edges {
							let from_node = nodes_redacted.iter().find(|&x| x.node_id == edge.from);
							let to_node = nodes_redacted.iter().find(|&x| x.node_id == edge.to);
							if from_node.is_some() && to_node.is_some() {
								network.add_edge(*from_node.unwrap(), *to_node.unwrap())?;
							}
		            	}
		            	//dbg!(network.clone());
		            	
		            	let num_observed = ((nodes.len() as f64)*(alpha as f64)/alpha_unit) as usize;
		            	let mut baseline;
		            	if num_observed > 0{
							baseline = nodes_redacted.iter().map(|&x| x.attr).collect::<Vec<i32>>();
						} else {
							baseline = nodes.iter().map(|&x| x.attr).collect::<Vec<i32>>();
						}
						baseline.sort();
						baseline.dedup();
		            	
		            	let mut inferred = infer_attrs(network,baseline);
		            	let num_guessed = inferred.len() - ((nodes.len() as f64)*(alpha as f64)/alpha_unit) as usize;
		            	inferred.sort_by(|a,b| a.node_id.cmp(&b.node_id));
		            	let num_wrong = inferred.iter().zip(nodes.clone().iter()).filter(|(a,b)| a.attr != b.attr).count();
		            	let acc = 1.0 - (num_wrong as f64)/(num_guessed as f64);
		            	
		            	//dbg!(inferred.clone());
		            	let config_c_mgd = File::options()
	                    .append(true)
	                    .create(true)
	                    .open("src/output/hw3_p1.csv")?;
						
						
						//output
						//dbg!(network.clone());
	                	to_csv(&name, &[(alpha as f64)/alpha_unit, acc], &[], config_c_mgd)?;
					}
					let alpha_time = alpha_split.elapsed();
            		let elapsed = now.elapsed();
		           	println!("alpha={} done in: {:.2?}, Total: {:.2?}", alpha, alpha_time, elapsed);
				}
	            
	            
			}
			//hw3 edge inference
			else if args.edge_inf {
				println!("Opening {}",filename_edges.clone());
	            let edges_file = File::open(filename_edges.clone())?;
	            
	            let path = Path::new(&filename_edges);
	            let name = path.file_stem().unwrap().to_str().unwrap();
	            
	            let edges = parse_edges(edges_file)?;

				println!("Opening {}",filename_nodes.clone());
				let nodes_file = File::open(filename_nodes.clone())?;
				let nodes = parse_attr_nodes(nodes_file)?;
				
				
				let mut full_network: SimpleNetwork<AttrNode> = SimpleNetwork::from_node_vec(nodes.clone());
		        for edge in &edges {
					let from_node = nodes.iter().find(|&x| x.node_id == edge.from);
					let to_node = nodes.iter().find(|&x| x.node_id == edge.to);
					if from_node.is_some() && to_node.is_some() {
						full_network.add_edge(*from_node.unwrap(), *to_node.unwrap())?;
					}
		   		}
	            
	            let edges_as_pairs = full_network.edgelist();
	            
	            //dbg!(nodes.clone());
	            //dbg!(edges.clone());
	            let alpha_unit = 20.0;
	            for alpha in 0..20 {
					let alpha_split = Instant::now();
					for rep in 0..50 {
						let edges_redacted = remove_edges(edges.clone(),(alpha as f64)/alpha_unit);
						//dbg!(nodes_redacted.clone());
						let mut network: SimpleNetwork<AttrNode> = SimpleNetwork::from_node_vec(nodes.clone());
		            	for edge in &edges_redacted {
							let from_node = nodes.iter().find(|&x| x.node_id == edge.from);
							let to_node = nodes.iter().find(|&x| x.node_id == edge.to);
							if from_node.is_some() && to_node.is_some() {
								network.add_edge(*from_node.unwrap(), *to_node.unwrap())?;
							}
		            	}
		            	//dbg!(network.clone());
		            	
		            	//let score_split = Instant::now();
		            	let j_scores = jaccard_scores(network.clone());
		            	let d_scores = dp_scores(network.clone());
		            	let s_scores = sp_scores(network.clone());
		            	//dbg!(score_split.elapsed());
		            	//dbg!(j_scores.len());
		            	//dbg!(j_scores.clone());
		            	//dbg!(d_scores.clone());
		            	//dbg!(s_scores.clone());
		            	let j_roc = roc(j_scores,edges_as_pairs.clone());
		            	let d_roc = roc(d_scores,edges_as_pairs.clone());
		            	let s_roc = roc(s_scores,edges_as_pairs.clone());
		            	//dbg!(score_split.elapsed());

		            	
		            	if alpha == 16 && rep == 0 {
							let j_roc_file = File::options()
	                    	.append(true)
	                    	.create(true)
	                    	.open("src/output/hw3_jroc.csv")?;
						
							//output
							//dbg!(network.clone());
							for item in &j_roc {
	                			to_csv(&name, &[item.0,item.1], &[], j_roc_file.try_clone()?)?;
	                		}
	                		let d_roc_file = File::options()
	                    	.append(true)
	                    	.create(true)
	                    	.open("src/output/hw3_droc.csv")?;
						
							//output
							//dbg!(network.clone());
							for item in &d_roc {
	                			to_csv(&name, &[item.0,item.1], &[], d_roc_file.try_clone()?)?;
	                		}
	                		let s_roc_file = File::options()
	                    	.append(true)
	                    	.create(true)
	                    	.open("src/output/hw3_sroc.csv")?;
						
							//output
							//dbg!(network.clone());
							for item in &s_roc {
	                			to_csv(&name, &[item.0,item.1], &[], s_roc_file.try_clone()?)?;
	                		}
						}
						
						let mut j_auc = 0.0;
						let mut d_auc = 0.0;
						let mut s_auc = 0.0;
						//dbg!(j_roc.clone());
						//dbg!(d_roc.clone());
						//dbg!(s_roc.clone());
						
						for i in 1..j_roc.len(){
							j_auc += j_roc[i].0*(j_roc[i].1 - j_roc[i-1].1);
							d_auc += d_roc[i].0*(d_roc[i].1 - d_roc[i-1].1);
							s_auc += s_roc[i].0*(s_roc[i].1 - s_roc[i-1].1);
						}
		            	//dbg!(score_split.elapsed());
		            	
		            	//dbg!(inferred.clone());
		            	let config_c_mgd = File::options()
	                    .append(true)
	                    .create(true)
	                    .open("src/output/hw3_p2.csv")?;
						
						
						//output
						//dbg!(network.clone());
	                	to_csv(&name, &[(alpha as f64)/alpha_unit,j_auc,d_auc,s_auc], &[], config_c_mgd)?;
					}
					let alpha_time = alpha_split.elapsed();
            		let elapsed = now.elapsed();
		           	println!("alpha={} done in: {:.2?}, Total: {:.2?}", alpha, alpha_time, elapsed);
				}
	            
			}
            //config model 1000x C and mgd
            else if args.cnull_model {
                //dbg!("output two open");
                println!("Opening {}",filename_edges.clone());
	            let edges_file = File::open(filename_edges.clone())?;
	            
	            let path = Path::new(&filename_edges);
	            let name = path.file_stem().unwrap().to_str().unwrap();
	            
	            let edges = parse_edges(edges_file)?;
	
	            let nodes;
				if filename_nodes != "" {
					println!("Opening {}",filename_nodes.clone());
					let nodes_file = File::open(filename_nodes.clone())?;
					nodes = parse_basic_nodes(nodes_file)?;
				} else {
					nodes = edges.clone().iter().flat_map(|edge| [edge.from, edge.to]).collect::<Vec<u64>>();
				}
	            //dbg!(nodes.clone());
	            //dbg!(edges.clone());
                
	            let mut network: SimpleNetwork<u64> = SimpleNetwork::from_node_vec(nodes);
	            for edge in edges {
	                network.add_edge(edge.from, edge.to)?;
	            }
	            //dbg!(network.clone());
                
                let reps = 1000;
                let init = 10*network.total_edges;
				let swaps = network.total_edges;
                //baseline data
                let config_c_mgd_one = File::options()
                    .append(true)
                    .create(true)
                    .open("src/output/hw2_p4_data.csv")?;
                let (mgd_actual,_,_) = network.mgd_diameter();
				let cluster_actual = network.cluster_coeff();
				//output to hw2_p4_data.csv
				//dbg!(network.clone());
               	to_csv(&name, &[cluster_actual,mgd_actual], &[], config_c_mgd_one)?;
               	for _ in 0..init {
					//double edge swap on network
					network.double_edge_swap()?;
				}
				for _ in 0..reps {
					let crg_split = Instant::now();
					for _ in 0..swaps {
						//double edge swap on network
						network.double_edge_swap()?;
					}
					let config_c_mgd = File::options()
                    .append(true)
                    .create(true)
                    .open("src/output/hw2_p4_data.csv")?;
					
					//measure C and mgd
					let (mgd,_,_) = network.mgd_diameter();
					let cluster = network.cluster_coeff();
					//output to hw2_p4_data.csv
					//dbg!(network.clone());
                	to_csv(&name, &[cluster,mgd], &[], config_c_mgd)?;
                	let crg_time = crg_split.elapsed();
            		let elapsed = now.elapsed();
            		println!("CRG in: {:.2?}, Total: {:.2?}", crg_time, elapsed);
					
				}
                println!(".");
			}
                        
            //Watts-Strogatz + betweenness centrality
            
            
            
            let partial_time = split.elapsed();
            let elapsed = now.elapsed();
            println!("Split: {:.2?}, Total: {:.2?}", partial_time, elapsed);
        }
    } else if args.w_hw_num == 3 {
		// read in (adjacency list)
        let mut adj_data = vec![];
        let mut filename = format!("src/data/{}.txt", args.name);
		// filepath management, just getting all filenames from the folder
		// should be in same position in vecs, unless something fails weirdly
		// does assume that any *_attr.txt file has a corresponding *.txt
        if args.folder {
            let foldername = &format!("src/data/{}", args.name);
            let paths = read_dir(foldername).unwrap();

            for entry in paths {
				let path_buf = entry?.path();
				let path_str = path_buf.to_str().ok_or(anyhow!("non-utf8 path"))?;
				
				if let Some(extension) = path_buf.extension(){
	                if extension == "txt"{
	                    adj_data.push(path_str.to_owned());
	                }
                }
            }
        } else{
			//dummy values so the for loop happens
			adj_data.push(filename.clone());
		}
        for i in 0..adj_data.len() {
			let split = Instant::now();
			if args.folder {
				filename = adj_data[i].clone();
			}

            println!("Opening {}",filename.clone());
            let adj_file = File::open(filename.clone())?;
            
            let path = Path::new(&filename);
            let name = path.file_stem().unwrap().to_str().unwrap();
            
            let mut adjs = parse_adjacency_list(adj_file)?;
            for adj in &mut adjs {
				if(adj.degree == 0) {
					adj.edges = vec![];
				} else {
					adj.edges = adj.edges.clone().into_iter().enumerate()
      								.filter(|&(i, x)| (i + 1) % 2 != 0)
     								.map(|(i, x)| x)
     								.collect::<Vec<u64>>();
     			}
			}
            
			
			let mut network: SimpleNetwork<u64> = SimpleNetwork::from_adj_list(adjs.clone());
			
			let reps = 1000;
			let init = 10*network.total_edges;
			let swaps = network.total_edges;
			
			
            let mut full_stats = HashMap::new();
            let baselines = network.harmonic_centrality();
            for (&node,r) in baselines.iter() {
				if let Some(source) = adjs.iter().find(|&adj| adj.node_id == node){
					let medici_real = File::options()
                    .append(true)
                    .create(true)
                    .open("src/output/hw2_medici_data.csv")?;
					let node_name = source.name.clone();
					to_csv(&node_name, &[*r], &[], medici_real)?;
				}
				full_stats.insert(node,vec![]);
			}
			
			for _ in 0..init {
				network.double_edge_swap();
			}
			for i in 0..reps {
				for j in 0..swaps {
					network.double_edge_swap();
				}
				let rand_data = network.harmonic_centrality();
	            for (node,r) in rand_data.iter() {
					if let Some(set) = full_stats.get_mut(node) {
						set.push(*r);
					}
				}
			}
			
			for (&node,set) in full_stats.iter() {
					if let Some(source) = adjs.iter().find(|&adj| adj.node_id == node){
						let medici_rand = File::options()
	                    .append(true)
	                    .create(true)
	                    .open("src/output/hw2_medici_dist.csv")?;
						let node_name = source.name.clone();
						to_csv(&node_name, &set, &[], medici_rand)?;
					}
				}
			
            let partial_time = split.elapsed();
            let elapsed = now.elapsed();
            println!("Split: {:.2?}, Total: {:.2?}", partial_time, elapsed);
        }
	}

    let elapsed = now.elapsed();
    println!("Completed in: {:.2?}", elapsed);
    Ok(())
}
