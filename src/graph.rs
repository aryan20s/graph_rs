use std::collections::{HashMap, HashSet, VecDeque};

use macroquad::math::Vec2;

use crate::node::GNode;

pub struct Graph {
    nodes: HashMap<u64, GNode>,
    edges: HashMap<u64, Vec<u64>>,
    is_directed: bool,
}

#[derive(Clone)]
pub struct GraphTraversal {
    pub to_visit: VecDeque<u64>,
    pub just_visited: Option<u64>,
    pub visited: HashSet<u64>,
}

impl Graph {
    pub fn new(is_directed: bool) -> Graph {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            is_directed,
        }
    }

    pub fn add_node(&mut self, data: u64, pos: Vec2) {
        if self.nodes.contains_key(&data) {
            return;
        }

        self.nodes.insert(
            data,
            GNode {
                data,
                pos,
                speed: Vec2::new(0.0, 0.0),
            },
        );
        self.edges.insert(data, vec![]);
    }

    pub fn add_edge(&mut self, src: u64, dest: u64) {
        if self.nodes.contains_key(&src) && self.nodes.contains_key(&dest) {
            self.edges.get_mut(&src).unwrap().push(dest);
            if !self.is_directed {
                self.edges.get_mut(&dest).unwrap().push(src); // if undirected, add opposite edge too
            }
        }
    }

    pub fn start_iterate(&self, src: u64) -> GraphTraversal {
        let mut to_visit = VecDeque::new();
        to_visit.push_back(src);

        GraphTraversal {
            to_visit,
            just_visited: None,
            visited: HashSet::new(),
        }
    }

    fn iterate_bfs_dfs(
        &self,
        cur_traverse: GraphTraversal,
        is_dfs: bool,
    ) -> Option<GraphTraversal> {
        let mut ret = cur_traverse.clone();

        let mut vert = None;
        while !ret.to_visit.is_empty() {
            // pop nodes until we find one thats not already visited
            let vert_check = if is_dfs {
                ret.to_visit.pop_back()
            } else {
                ret.to_visit.pop_front()
            };

            if !ret.visited.contains(&vert_check.unwrap()) {
                vert = vert_check;
                break; // if found, break
            }
        }

        if vert == None {
            return None;
        }

        let vert = vert.unwrap();

        ret.just_visited = Some(vert);
        ret.visited.insert(vert);

        for edge in self.edges.get(&vert).unwrap() {
            if !ret.visited.contains(edge) {
                ret.to_visit.push_back(*edge);
            }
        }

        Some(ret)
    }

    pub fn iterate_bfs(&self, cur_traverse: GraphTraversal) -> Option<GraphTraversal> {
        self.iterate_bfs_dfs(cur_traverse, false)
    }

    pub fn iterate_dfs(&self, cur_traverse: GraphTraversal) -> Option<GraphTraversal> {
        self.iterate_bfs_dfs(cur_traverse, true)
    }

    pub fn get_nodes(&self) -> Vec<&GNode> {
        self.nodes.values().collect()
    }

    pub fn get_nodes_mut(&mut self) -> Vec<&mut GNode> {
        self.nodes.values_mut().collect()
    }

    pub fn get_node_mut(&mut self, id: u64) -> Option<&mut GNode> {
        self.nodes.get_mut(&id)
    }

    pub fn get_node(&self, id: u64) -> Option<&GNode> {
        self.nodes.get(&id)
    }

    pub fn get_edges(&self) -> &HashMap<u64, Vec<u64>> {
        &self.edges
    }
}
