pub mod graph {

    use std::collections::HashMap;

    use graph_items::{edge::Edge, node::Node};

    pub trait WithAttrs {
        fn attrs_mut(&mut self) -> &mut HashMap<String, String>;

        fn attr(&self, name: &str) -> Option<&str>;

        fn with_attrs<T: ToString>(mut self, attr: &[(T, T)]) -> Self
        where
            Self: Sized,
        {
            let attrs: HashMap<String, String> = attr
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            *self.attrs_mut() = attrs;
            self
        }
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&n| n.name() == name)
        }
    }

    impl WithAttrs for Graph {
        fn attrs_mut(&mut self) -> &mut HashMap<String, String> {
            &mut self.attrs
        }
        fn attr(&self, name: &str) -> Option<&str> {
            self.attrs.get(name).map(|s| s.as_str())
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            use crate::graph::WithAttrs;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                a: String,
                b: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new<T: ToString>(a: T, b: T) -> Self {
                    Self {
                        a: a.to_string(),
                        b: b.to_string(),
                        attrs: HashMap::new(),
                    }
                }
            }
            impl WithAttrs for Edge {
                fn attrs_mut(&mut self) -> &mut HashMap<String, String> {
                    &mut self.attrs
                }
                fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;

            use crate::graph::WithAttrs;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new<T: ToString>(name: T) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn name(&self) -> &str {
                    self.name.as_str()
                }
            }

            impl WithAttrs for Node {
                fn attrs_mut(&mut self) -> &mut HashMap<String, String> {
                    &mut self.attrs
                }
                fn attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }
    }
}
