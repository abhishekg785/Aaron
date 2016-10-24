/*
* DOM data structure
* DOM is a tree of nodes, it has zero or more children
* it has other attrs and methods , but we can ignore most of them now
*/

use std::collections::{HashMap, HashSet};
pub type AttrMap = HashMap<String, String>;

pub struct Node {
	//data common to all nodes
	pub children: Vec<Node>,

	//data specific to each node type
	pub node_type : NodeType,
}

pub enum NodeType {	
	Text(String),
	Element(ElementData),
}

pub struct ElementData {
	pub tag_name : String,
	pub attributes : AttrMap,
}

// cons fn to make it easy to create new nodes
pub fn text(data: String) -> Node {
	Node { children:Vec::new(), node_type: NodeType::Text(data)}
}

pub fn elem(name: String, attrs: AttrMap, children:Vec<Node>) -> Node {
	Node {
		children: children,
		node_type: NodeType::Element(ElementData {
			tag_name: name,
			attributes: attrs
		})
	}
}