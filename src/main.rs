use std::fs::{read_dir, File};
use std::time::Instant;
use std::path::Path;
use anyhow::anyhow;

use hw5352::{hw1::parse_edges, hw1::parse_nodes, output::to_csv, SimpleNetwork};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    w_hw_num: u32,

    #[arg(short, long)]
    name: String,

    #[clap(long, short, action)]
    folder: bool,

    #[clap(long, short, action)]
    degree: bool,

    #[clap(long, short, action)]
    length: bool,
}

/// Call with: cargo run [--release] -- [HW1] [options]
/// Can also use cargo run -- --help to view full auto-generated
/// options list as they might not all be here
/// HW1 Options:
///		[-n]  			file/folder name (no extension or "_attr")
///		[-f]			specifies to run on every file within target folder, rather than on a single data set
///		[-d]			compute degree measures
///		[-l]			compute distance measures
fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let args = Args::parse();

    if args.w_hw_num == 1 {
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
            let partial_time = split.elapsed();
            let elapsed = now.elapsed();
            println!("Split: {:.2?}, Total: {:.2?}", partial_time, elapsed);
            
        }
    }

    let elapsed = now.elapsed();
    println!("Completed in: {:.2?}", elapsed);
    Ok(())
}
