use std::{fs::read_to_string};

use petgraph::Graph;
use priority_queue::PriorityQueue;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    time_left: usize,
    moving_duration: (usize, usize),
    next_player: Player,
    position: (usize, usize),
    valve_states: Vec<bool>,
    actions: Vec<Action>,
    flow: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    Open(usize)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Player {
    Me,
    Elephant
}

struct ValveSystem {
    valves: Vec<String>,
    distance_matrix: Vec<Vec<usize>>,
    flow_rates: Vec<usize>,
    is_two_player: bool
}

impl ValveSystem {

    fn get_index(valves: &Vec<String>, name: &str) -> usize {
        valves.iter().enumerate()
        .find(|(i,v)| *v == name)
        .map(|(i, _)| i)
        .unwrap()
    }

    fn parse(path: &str, is_two_player: bool) -> ValveSystem {
        let input = read_to_string(path).unwrap();
        let mut valves: Vec<String> = input.lines()
        .map(|l| l.split_whitespace().nth(1).unwrap().to_string())
        .collect();
        valves.sort();

        let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); valves.len()];
        let mut flow_rates = vec![0; valves.len()];

        for line in input.lines() {
            let items: Vec<&str> = line.split_whitespace().collect();
            
            let node_index = Self::get_index(&valves, items[1]);
            let rate : usize = items[4].strip_prefix("rate=").unwrap().strip_suffix(";").unwrap().parse().unwrap();
            let neighbours: Vec<usize> = items[9..items.len()].iter()
            .map(|n| n.strip_suffix(",").or(Some(n)).unwrap().to_string())
            .map(|n| Self::get_index(&valves, &n))
            .collect();

            adjacency_list[node_index] = neighbours;
            flow_rates[node_index] = rate;
        }

        let distance_matrix = Self::compute_distance_matrix(&adjacency_list);

        ValveSystem { 
            valves,
            flow_rates,
            distance_matrix,
            is_two_player
        }
    }

    fn compute_distance_matrix(adjacency_list: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut edges = Vec::new();
        for (u, neighbours) in adjacency_list.iter().enumerate() {
            for &v in neighbours {
                edges.push((u as u32,v as u32));
            }
        }

        let graph = Graph::<(), i32>::from_edges(edges);
        let mut distances: Vec<Vec<usize>> = Vec::new();
        for start in graph.node_indices() {
            let min_dists = petgraph::algo::dijkstra(
                &graph,
                start, 
                None,
                |_| 1
            );
            let mut distances_from_node: Vec<usize> = (0..graph.node_count()).map(|_| usize::MAX).collect();
            for (node, distance) in min_dists.iter() {
                if start != *node {
                    distances_from_node[node.index()] = *distance as usize + 1;
                }
            }
            distances.push(distances_from_node);
        }
        distances
    }

    fn expand(&self, state: &State, action: Action) -> State {
        let mut valve_states = state.valve_states.clone();
        let Action::Open(target) = action;
        valve_states[target] = true;
        let mut actions = state.actions.clone();
        actions.push(Action::Open(target));
        
        let (cur_pos, elephant_pos) = state.position;
        let (time_wait, elephant_time_wait) = state.moving_duration;

        let mut moving_duration = (0,0);
        let mut position = (0, 0);
        let mut flow = state.flow;
        let mut time_left = state.time_left;


        let next_player = match state.next_player {
            Player::Me => {
                let duration = self.distance_matrix[cur_pos][target];
                position = (target, elephant_pos);
                flow += (time_left-duration) * self.flow_rates[target];
                if duration < elephant_time_wait {
                    moving_duration = (0, elephant_time_wait-duration);
                    time_left -=  duration;    
                } else {
                    moving_duration = (duration-elephant_time_wait, 0);
                    time_left -= elephant_time_wait;
                   
                }
                match moving_duration {
                    (0, 0) => Player::Elephant,
                    (0, _) => Player::Me,
                    (_, 0) => Player::Elephant,
                    (_, _) => panic!("invalid case")
                }
            },
            Player::Elephant => {
                let duration = self.distance_matrix[elephant_pos][target];
                flow += (time_left-duration) * self.flow_rates[target];
                position = (cur_pos, target);
                if duration < time_wait {
                    moving_duration = (time_wait-duration, 0);
                    time_left -=  duration;
                } else {
                    moving_duration = (0, duration-time_wait);
                    time_left -= time_wait;
                 
                }
                match moving_duration {
                    (0, 0) => Player::Me,
                    (0, _) => Player::Me,
                    (_, 0) => Player::Elephant,
                    (_,_) => panic!("invalid case")
                }
            }  
        };

        State {
            valve_states,
            next_player,
            moving_duration,
            flow,
            position,
            time_left,
            actions
        }
    }

    fn legal_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = vec![];
        let pos = match state.next_player {
            Player::Me => state.position.0,
            Player::Elephant => state.position.1
        };

        for (valve, cost) in self.distance_matrix[pos].iter().enumerate() {
            if valve != state.position.0 && valve != state.position.1 && *cost <= state.time_left && !state.valve_states[valve] && self.flow_rates[valve] > 0 {
                actions.push(Action::Open(valve));
            }
        }
        actions
    }
    
    fn heuristic(&self, state: &State) -> i32 {
        let mut flow: i32 = 0;
        let open_valves: Vec<usize> = (0..self.valves.len()).filter(|&v| !state.valve_states[v]).collect();
        let time_left = state.time_left;
        for valve in open_valves {
            flow += (time_left * self.flow_rates[valve]) as i32;
        }
        flow
    }


    fn compute_plan(&self, initial_valve: &str, time_left: usize) -> State {
        let current_valve = Self::get_index(&self.valves, initial_valve);
        let moving_duration = if self.is_two_player {
            (0, 0)
        } else {
            (0, usize::MAX)
        };

        let initial = State {
            time_left,
            position: (current_valve, current_valve),
            valve_states: vec![false; self.valves.len()],
            actions: Vec::new(),
            flow: 0,
            moving_duration,
            next_player: Player::Me
        };

        let mut best_state = initial.clone();
        let mut queue = PriorityQueue::new();
        queue.push(initial, i32::MIN);
       
        while let Some((current_state, _)) = queue.pop() {
            
            for action in self.legal_actions(&current_state) {
                let next_state = self.expand(&current_state, action);
                let priority = (next_state.flow as i32) + self.heuristic(&next_state);
                if priority >= best_state.flow  as i32 {
                    queue.push(next_state, priority);
                }
            }

            //println!("best={}, Queue: {}", best_state.flow, queue.len());
            if current_state.flow > best_state.flow {
                best_state = current_state.clone();
            }
        }
        
        best_state
    }
}

fn main() {
    let system = ValveSystem::parse("./input/task_1.txt", false);
    let state = system.compute_plan("AA", 30);
    println!("[Task 1] Total max pressure released within 30 min.: {}", state.flow);

    let system = ValveSystem::parse("./input/task_1.txt", true);
    let state = system.compute_plan("AA", 26);
    println!("[Task 2] Total max pressure released within 26 min.: {}", state.flow);
}
