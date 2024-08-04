use std::{collections::HashSet};

#[derive(PartialEq, Eq, Clone, Debug)]
enum TentativeWight {
    Infinit,
    Number(u32),
}

impl Ord for TentativeWight {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other {
            TentativeWight::Infinit => match self {
                TentativeWight::Infinit => std::cmp::Ordering::Equal,
                _ => std::cmp::Ordering::Less,
            },
            TentativeWight::Number(o) => match self {
                TentativeWight::Infinit => std::cmp::Ordering::Greater,
                TentativeWight::Number(s) => s.cmp(o),
            },
        }
    }
}

impl PartialOrd for TentativeWight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type KeyType = u64;
#[derive(Clone)]
struct Edge {
    weight: u32,
    node: usize,
}

pub struct InternetOfThings {
    adjacency_list: Vec<Vec<Edge>>,
    nodes: Vec<KeyType>,
}

fn min_index(weights: &Vec<TentativeWight>, nodes: &Vec<usize>) -> usize {
    let mut min_weight = (weights[0].clone(), 0);
    for node in nodes.iter() {
        if let Some(n) = weights.get(*node) {
            if n < &min_weight.0 {
                min_weight = ((&weights[*node]).clone(), node.clone())
            }
        }
    }
    return min_weight.1;
}

impl InternetOfThings {
    fn get_node_index(&self, node: KeyType) -> Option<usize> {
        self.nodes.iter().position(|n| n == &node)
    }

    pub fn set_edges(&mut self, from: KeyType, edges: Vec<(u32, KeyType)>) {
        let edges: Vec<Edge> = edges
            .into_iter()
            .filter_map(|e| {
                if let Some(to) = self.get_node_index(e.1) {
                    Some(Edge {
                        weight: e.0,
                        node: to,
                    })
                } else {
                    None
                }
            })
            .collect();
        match self.nodes.iter().position(|n| n == &from) {
            Some(i) => self.adjacency_list[i] = edges,
            None => {
                self.nodes.push(from);
                self.adjacency_list.push(edges)
            }
        }
    }

    pub fn set_nodes(&mut self, nodes: Vec<KeyType>) {
        self.nodes = nodes;
        self.adjacency_list = vec![vec![]; self.nodes.len()];
    }

    pub fn connected(&self, from: KeyType, degree: usize) -> Option<HashSet<KeyType>> {
        self.nodes.iter().position(|n| n == &from).map(|i| {
            self.conected_rec(i, degree)
                .into_iter()
                .map(|n| self.nodes[n].clone())
                .collect()
        })
    }

    fn conected_rec(&self, from: usize, degree: usize) -> HashSet<usize> {
        if degree > 0 {
            self.adjacency_list[from]
                .iter()
                .flat_map(|e| {
                    let mut set = self.conected_rec(e.node, degree - 1);
                    set.insert(e.node);
                    set
                })
                .collect()
        } else {
            HashSet::new()
        }
    }

    pub fn shortest_path(&self, from: KeyType, to: KeyType) -> Option<(u32, Vec<KeyType>)> {
        let mut src = None;
        let mut dest = None;
        for (i, n) in self.nodes.iter().enumerate() {
            if n == &from {
                src = Some(i);
            }
            if n == &to {
                dest = Some(i);
            }
            if src.is_some() && dest.is_some() {
                break;
            }
        }
        if src.is_some() && dest.is_some() {
            let (src, dest) = (src.unwrap(), dest.unwrap());
            let mut distance: Vec<TentativeWight> = vec![TentativeWight::Infinit; self.nodes.len()];
            distance[src] = TentativeWight::Number(0);
            let mut open: Vec<usize> = (0..self.nodes.len()).into_iter().collect();
            let mut parent = vec![None; self.nodes.len()];
            let mut found = false;
            while !open.is_empty() {
                let u = min_index(&distance, &open);
                let u = open.remove(u);
                if u == dest {
                    found = true;
                    break;
                }
                let dist = distance[u].clone();
                for e in &self.adjacency_list[u] {
                    let new_distance = match dist {
                        TentativeWight::Number(n) => TentativeWight::Number(n + e.weight),
                        _ => TentativeWight::Infinit,
                    };
                    let old_distance = distance[e.node].clone();
                    if new_distance < old_distance {
                        distance[e.node] = new_distance;
                        parent[e.node] = Some(u);
                    }
                }
            }
            if found {
                let mut path = vec![];
                let mut p = parent[dest].unwrap();
                path.push(self.nodes[dest].clone());
                while p != src {
                    path.push(self.nodes[p].clone());
                    p = parent[p].unwrap();
                }
                path.push(self.nodes[src].clone());
                path.reverse();
                let cost = match distance[dest] {
                    TentativeWight::Number(n) => n,
                    _ => 0,
                };
                Some((cost, path))
            } else {
                None
            }
        } else {
            None
        }
    }
}
