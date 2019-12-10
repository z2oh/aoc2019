use std::collections::{HashMap, HashSet, hash_map::Entry};

#[derive(Debug, Default)]
struct Graph {
    nodes: Vec<Node>,
}

#[derive(Debug, Default)]
struct Node {
    children: HashSet<usize>,
}

fn main() {
    let mut node_map: HashMap<String, usize> = HashMap::new();

    let mut counter = 0;
    let mut root: Option<usize> = None;
    let mut san:  Option<usize> = None;
    let mut you:  Option<usize> = None;

    let mut graph = Graph::default();

    for line in include_str!("input").lines() {
        let l: Vec<_> = line.split(')').collect();

        let p_idx = match node_map.entry(l[0].to_string()) {
            Entry::Vacant(v) => {
                let idx = counter;
                v.insert(idx);
                counter += 1;
                graph.nodes.push(Node::default());
                idx
            },
            Entry::Occupied(o) => *o.get(),
        };
        let c_idx = match node_map.entry(l[1].to_string()) {
            Entry::Vacant(v) => {
                let idx = counter;
                v.insert(idx);
                counter += 1;
                graph.nodes.push(Node::default());
                idx
            },
            Entry::Occupied(o) => *o.get(),
        };

        graph.nodes[p_idx].children.insert(c_idx);

        if l[0] == "COM" { root = Some(p_idx) };
        if l[1] == "SAN" { san  = Some(p_idx) };
        if l[1] == "YOU" { you  = Some(p_idx) };
    }

    println!("{}", dfs_count_transfers(&graph, root.unwrap(), san.unwrap(), you.unwrap()));
}

fn dfs_count_transfers(g: &Graph, root: usize, san: usize, you: usize) -> u32 {
    let (_, san_trace, you_trace) = dfs_recurse(g, root, san, you, 0, &mut vec![root]);
    let san_trace = san_trace.unwrap();
    let you_trace = you_trace.unwrap();
    let mut i = 0;
    let common = loop {
        if san_trace[i] != you_trace[i] { break i } else { i += 1 };
    };
    (san_trace.len() + you_trace.len() - common * 2) as u32
}

// Return value is (Depth, SAN trace, YOU trace)
fn dfs_recurse(
    g: &Graph,
    root: usize,
    san: usize,
    you: usize,
    depth: u32,
    trace: &mut Vec<usize>
) -> (u32, Option<Vec<usize>>, Option<Vec<usize>>) {
    let node = &g.nodes[root];
    let mut acc = depth;
    let mut san_trace = None;
    let mut you_trace = None;

    for c_idx in &node.children {
        trace.push(*c_idx);
        if *c_idx == san {
            san_trace = Some(trace.clone());
        }
        if *c_idx == you {
            you_trace = Some(trace.clone());
        }
        let (new_acc, local_san, local_you) = dfs_recurse(g, *c_idx, san, you, depth + 1, trace);
        if san_trace.is_none() { san_trace = local_san };
        if you_trace.is_none() { you_trace = local_you };
        trace.pop();
    }

    (acc, san_trace, you_trace)
}
