use std::fs::read_to_string;

use petgraph::{algo, prelude::DiGraph, Graph};

fn read_map(path: &str) -> DiGraph<(usize, usize, char), i32> {
    let input = read_to_string(path).unwrap();
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut graph = DiGraph::new();
    let mut matrix = Vec::new();

    for x in 0..map.len() {
        let mut row = Vec::new();
        for y in 0..map[0].len() {
            let idx = graph.add_node((x, y, map[x][y]));
            row.push(idx);
        }
        matrix.push(row);
    }

    let get_elevation = |elevation| match elevation {
        'S' => 'a' as i32,
        'E' => 'z' as i32,
        e => e as i32,
    };

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let idx = matrix[x][y];
            let elevation = get_elevation(graph[idx].2);
            if y > 0 {
                let l_elevation = get_elevation(graph[matrix[x][y - 1]].2);
                if elevation - l_elevation >= -1 {
                    graph.add_edge(idx, matrix[x][y - 1], 1);
                }
            }

            if x < map.len() - 1 {
                let t_elevation = get_elevation(graph[matrix[x + 1][y]].2);
                if elevation - t_elevation >= -1 {
                    graph.add_edge(idx, matrix[x + 1][y], 1);
                }
            }

            if y < map[0].len() - 1 {
                let r_elevation = get_elevation(graph[matrix[x][y + 1]].2);
                if elevation - r_elevation >= -1 {
                    graph.add_edge(idx, matrix[x][y + 1], 1);
                }
            }

            if x > 0 {
                let d_elevation = get_elevation(graph[matrix[x - 1][y]].2);
                if elevation - d_elevation >= -1 {
                    graph.add_edge(idx, matrix[x - 1][y], 1);
                }
            }
        }
    }
    graph
}

fn find_shortest_path(graph: &Graph<(usize, usize, char), i32>, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let end = graph.node_indices().find(|n| graph[*n].2 == 'E').unwrap();
    let start = graph
        .node_indices()
        .find(|n| (graph[*n].0, graph[*n].1) == start)
        .unwrap();
    let result = algo::astar(
        &graph,
        start,
        |n| n == end,
        |_| 1,
        |_| 1,
    );

    if let Some(path) = result {
        Option::Some(path.1.iter().map(|&n| (graph[n].0, graph[n].1)).collect())
    } else {
        Option::None
    }
}

fn main() {
    let graph = read_map("input/task_1.txt");
    let start = *graph.node_weights().find(|(_, _, e)| *e == 'S').unwrap();
    let shortest_path = find_shortest_path(&graph, (start.0, start.1)).unwrap();
    let task_1_answer = shortest_path.len() - 1;
    println!("[Task 1] Shortest path length: {}", task_1_answer);

    let task_2_answer: ((usize, usize), usize) = graph
        .node_weights()
        .filter(|(_, _, e)| *e == 'a' || *e == 'S')
        .map(|s| ((s.0, s.1), find_shortest_path(&graph, (s.0, s.1))))
        .filter(|(_, result)| result.is_some())
        .map(|(loc, res)| (loc, res.unwrap().len() - 1))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    println!("[Task 2] Shortest path length to peak: {}", task_2_answer.1);
}
