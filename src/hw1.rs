//! Utilities for parsing data from the FB100 dataset
//!
//! The `_attr` files are text files with one header line and a data line for each
//! node. The nodes have an `id`, `status`, `gender`, `major`, `dorm`, and `year`,
//! each represented by an integer value.
//!
//! The `edges` files are text files consisting of several rows containing 2 node `id`s,
//! indicating that an edge exists between those two nodes.

use serde::Deserialize;
use crate::node::Node;

/// A record of a node
///
/// This represents a single row of the `_attr` file.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct NodeData {
    pub node_id: u64,
    status: i32,
    gender: i32,
    major: i32,
    dorm: i32,
    year: i32,
}

/// Parse a file-like object into a vector of nodedata.
///
pub fn parse_nodes<R: std::io::Read>(input: R) -> Result<Vec<NodeData>, csv::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .from_reader(input);

    rdr.deserialize().skip(1)
        // `Result` implements fromiterator, so when we collect this it will give us the first
        // error if there are any errors, or else will give us the vector of [`NodeData`]s.
        .collect()
}

/// Parse a file-like object into a vector of nodes - nodes are u64s here
///
pub fn parse_basic_nodes<R: std::io::Read>(input: R) -> Result<Vec<u64>, csv::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .from_reader(input);

    rdr.deserialize()
        // `Result` implements fromiterator, so when we collect this it will give us the first
        // error if there are any errors, or else will give us the vector of [`NodeData`]s.
        .collect()
}

/// A record of a edge
///
/// This represents a single row of the edges file.
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Edge {
    pub from: u64,
    pub to: u64,
}

/// Parse a file-like object into a vector of edges.
///
pub fn parse_edges<R: std::io::Read>(input: R) -> Result<Vec<Edge>, csv::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .from_reader(input);

    rdr.deserialize()
        // `Result` implements fromiterator, so when we collect this it will give us the first
        // error if there are any errors, or else will give us the vector of [`Edge`]s.
        .collect()
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Adjacency<N: Node = u64> {
	pub node_id: N,
	pub name: String,
	pub id_again: N,
	pub degree: u64,
	pub edges: Vec<N>,
}

pub fn parse_adjacency_list<R: std::io::Read>(input: R) -> Result<Vec<Adjacency<u64>>, csv::Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(b' ')
        .comment(Some(b'#'))
        .from_reader(input);

	let mut end_list = vec![];
    for result in rdr.deserialize() {
        let record: Adjacency<u64> = result?;
        end_list.push(record);
    }
    Ok(end_list)
        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_nodes_works() -> Result<(), csv::Error> {
        const DATA: &[u8] = b"id	status	gender	major	dorm	year
1	2	3	4	5	6";

        let out = parse_nodes(DATA)?;
        assert_eq!(
            out,
            vec![NodeData {
                node_id: 1,
                status: 2,
                gender: 3,
                major: 4,
                dorm: 5,
                year: 6,
            }],
        );

        Ok(())
    }

    #[test]
    fn multiline_node_parser() -> Result<(), csv::Error> {
        const DATA: &[u8] = b"id	status	gender	major	dorm	year
1	2	3	4	5	6
2	2	3	8	5	6
3	2	3	4	5	7";

        let out = parse_nodes(DATA)?;
        assert_eq!(
            out,
            vec![
                NodeData {
                    node_id: 1,
                    status: 2,
                    gender: 3,
                    major: 4,
                    dorm: 5,
                    year: 6,
                },
                NodeData {
                    node_id: 2,
                    status: 2,
                    gender: 3,
                    major: 8,
                    dorm: 5,
                    year: 6,
                },
                NodeData {
                    node_id: 3,
                    status: 2,
                    gender: 3,
                    major: 4,
                    dorm: 5,
                    year: 7,
                },
            ],
        );

        Ok(())
    }

    #[test]
    fn parsing_edges_works() -> Result<(), csv::Error> {
        const DATA: &[u8] = b"12	1
13	1
2	3";

        let out = parse_edges(DATA)?;
        assert_eq!(
            out,
            vec![
                Edge { from: 12, to: 1 },
                Edge { from: 13, to: 1 },
                Edge { from: 2, to: 3 },
            ],
        );

        Ok(())
    }
}
