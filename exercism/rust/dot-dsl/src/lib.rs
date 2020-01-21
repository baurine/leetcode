pub mod graph {

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    fn covert_attrs(attrs: &[(&str, &str)]) -> HashMap<String, String> {
        let mut new_attrs = HashMap::new();
        for item in attrs {
            new_attrs.insert(String::from(item.0), String::from(item.1));
        }
        new_attrs
    }

    pub mod graph_items {
        use std::collections::HashMap;

        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: super::HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attrs: super::HashMap::new(),
                    }
                }

                // pub fn with_attrs<'a>(&'a mut self, attrs: &[(&str, &str)]) -> &'a mut Self {
                //     for item in attrs {
                //         self.attrs
                //             .insert(String::from(item.0), String::from(item.1));
                //     }
                //     self
                // }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: crate::graph::covert_attrs(attrs),
                        // must put it in the end
                        ..self
                    }
                }

                pub fn get_attr(&self, attr_key: &str) -> Option<&str> {
                    // self.attrs.get(attr_key).map(|x| (*x).as_str())
                    // self.attrs.get(attr_key).map(|x| x.as_str())
                    // self.attrs.get(attr_key).map(|x| (*x).as_ref())
                    self.attrs.get(attr_key).map(|x| x.as_ref())
                }
            }
        }

        pub mod edge {

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                node_start: String,
                node_end: String,
                attrs: super::HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node_start: &str, node_end: &str) -> Self {
                    Edge {
                        node_start: String::from(node_start),
                        node_end: String::from(node_end),
                        attrs: super::HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: crate::graph::covert_attrs(attrs),
                        ..self
                    }
                }
            }
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

        pub fn with_nodes(self, nodes: &Vec<Node>) -> Self {
            Graph {
                nodes: nodes.clone(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &Vec<Edge>) -> Self {
            Graph {
                edges: edges.clone(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: crate::graph::covert_attrs(attrs),
                ..self
            }
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&item| item.name == name)
        }
    }
}
