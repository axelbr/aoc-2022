use std::{fs::read_to_string, cmp::max};

type Map = Vec<Vec<i32>>;


fn load_map(path: &str) -> Map {
    let input = read_to_string(path).unwrap();
    let mut map: Map = Vec::new();
    for line in input.lines() {
        let row: Vec<i32> = line.chars().map(|c| c.to_string().parse().unwrap()).collect();
        map.push(row);
    }
    return map;
}

fn transpose(map: &Map) -> Map {
    let mut columns: Map = (0..map.len()).map(|_| Vec::new()).collect();
    for row in map {
        for (j, elem) in row.iter().enumerate() {
            columns[j].push(*elem);
        }
    }
    columns
}

fn count_visible(map: &Map) -> usize {
    let mut visible = 0;
    let cols = map[0].len();
    let mut visible_map: Vec<Vec<bool>> = (0..map.len()).map(|_| vec![false; cols]).collect();

    for i in 0..map.len() {
        let mut l_max = -1;
        for j in 0..map[i].len() {
            if map[i][j] > l_max {
                l_max = map[i][j];
                visible_map[i][j] = true;
            }
        }
        let mut r_max = -1;
        for j in (0..map[i].len()).rev() {
            if map[i][j] > r_max {
                r_max = map[i][j];
                visible_map[i][j] = true;
            }
        }
    }

    for j in 0..map[0].len() {
        let mut t_max = -1;
        for i in 0..map.len() {
            if map[i][j] > t_max {
                t_max = map[i][j];
                visible_map[i][j] = true;
            }
        }

        let mut b_max = -1;
        for i in (0..map.len()).rev() {
            if map[i][j] > b_max {
                b_max = map[i][j];
                visible_map[i][j]= true;
            }
        }
    }


    visible = visible_map.iter()
    .map(|row| row.iter()
        .map(|&b| if b {1} else {0} as usize)
        .sum::<usize>()
    )
    .sum();
    visible
}

fn scenic_score(map: &Map, position: (usize, usize)) -> usize {
    let (row, col) = position;
    let mut i = (row as i32) - 1;
    let mut score = 0;
    let mut total_score = 1;


    while i >= 0 && map[i as usize][col] < map[row][col] {
        score += 1;
        i -= 1;
    }
    if i >= 0 {
        score += 1;
    }

    let mut i = row + 1;
    total_score *= max(score, 1);
    score = 0;
    while i < map.len() && map[i][col] < map[row][col] {
        score += 1;
        i += 1;
    }
    if i < map.len() {
        score += 1;
    }


    let mut j = (col as i32) - 1;
    total_score *= max(score, 1);
    score = 0;
    while j >= 0 && map[row][j as usize] < map[row][col] {
        score += 1;
        j -= 1;
    }
    if j >= 0 {
        score += 1;
    }


    let mut j = col+1;
    total_score *= max(score, 1);
    score = 0;
    while j < map[0].len() && map[row][j] < map[row][col] {
        score += 1;
        j += 1;
    }
    if j < map[0].len() {
        score += 1;
    }

    total_score * max(score, 1)

}

fn main() {
    let map = load_map("./input/task_1.txt");
    let visible = count_visible(&map);
    println!("[Task 1] Number of visible trees: {}", visible);

    let mut max_scenic_score = 0;
    for i in 1..map.len()-1 {
        for j in 1..map[i].len()-1 {
            let score = scenic_score(&map, (i,j));
            if max_scenic_score < score {
                max_scenic_score = score;
            }
        }
    }
    println!("[Task 2] Max scenic score: {}", max_scenic_score);
}
