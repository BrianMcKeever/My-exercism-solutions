pub mod graph {
    use graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    pub struct Graph {
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
        pub nodes: Vec<Node>,
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Edge {
                point_1_name: String,
                point_2_name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(point_1_name: &str, point_2_name: &str) -> Self {
                    Edge {
                        point_1_name: point_1_name.to_string(),
                        point_2_name: point_2_name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self: Edge, attrs: &[(&str, &str)]) -> Self {
                    for (a, b) in attrs {
                        self.attrs.insert(a.to_string(), b.to_string());
                    }
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Eq, Clone, Default)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        attrs: HashMap::new(),
                        name: name.to_string(),
                    }
                }
                pub fn get_attr(self: &Node, name: &str) -> Option<&str> {
                    match self.attrs.get(name) {
                        None => None,
                        Some(value) => Some(&value),
                    }
                }
                pub fn with_attrs(mut self: Node, attrs: &[(&str, &str)]) -> Self {
                    for (a, b) in attrs {
                        self.attrs.insert(a.to_string(), b.to_string());
                    }
                    self
                }
            }
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                edges: Vec::new(),
                nodes: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_edges(mut self: Graph, edges: &[Edge]) -> Self {
            for e in edges {
                self.edges.push(e.clone());
            }
            self
        }
        pub fn with_nodes(mut self: Graph, nodes: &[Node]) -> Self {
            for n in nodes {
                self.nodes.push(n.clone());
            }
            self
        }
        pub fn with_attrs(mut self: Graph, attrs: &[(&str, &str)]) -> Self {
            for (a, b) in attrs {
                self.attrs.insert(a.to_string(), b.to_string());
            }
            self
        }
        pub fn get_node(self: &Graph, name: &str) -> Option<Node> {
            for n in self.nodes.iter() {
                if n.name == name {
                    return Some(n.clone());
                }
            }
            None
        }
        pub fn get_attr(self: &Graph, name: &str) -> Option<&str> {
            match self.attrs.get(name) {
                None => None,
                Some(value) => Some(&value),
            }
        }
    }
}
