//! An abstracted node.

/// Abstracts over a single node in a network.
pub trait Node:
    Default + std::fmt::Debug + std::fmt::Display + PartialEq + Eq + Copy + Clone + std::hash::Hash + Ord + PartialOrd
{
    /// The (unique) id of the node.
    fn id(&self) -> u64;
}


impl Node for u32 {
    fn id(&self) -> u64 {
		*self as u64
	}
}

impl Node for u64 {
    fn id(&self) -> u64 {
		*self
	}
}