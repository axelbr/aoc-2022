use std::{
    cmp::{max, min},
    fs::{read_to_string, File},
    io::Write
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32
}
type Line = Vec<Point>;
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
#[derive(Clone)]
struct Map {
    data: Vec<Vec<char>>,
    offset: i32
}
impl Map {
    fn from_lines(lines: &Vec<Line>, limited_depth: bool) -> Map {
        let max_x = lines
        .iter()
        .map(|l| l.iter().map(|p| p.x).max().unwrap())
        .max()
        .unwrap()
        + 1;
        let max_y = lines
            .iter()
            .map(|l| l.iter().map(|p| p.y).max().unwrap())
            .max()
            .unwrap()
            + 3;
        let mut map: Vec<Vec<char>> = (0..max_x).map(|_| vec!['.'; max_y as usize]).collect();
        for line in lines {
            for pair in line.windows(2) {
                let (a, b) = (&pair[0], &pair[1]);
                for x in min(a.x, b.x)..max(a.x, b.x)+1 {
                    map[x as usize][a.y as usize] = '#';
                }
                for y in min(a.y, b.y)..max(a.y, b.y)+1 {
                    map[a.x as usize][y as usize] = '#';
                }
            }
        }
        if limited_depth {
            for x in 0..max_x {
                map[x as usize][(max_y-1) as usize] = '#';
            }
        }
        Map { data: map, offset: 0}
    }
    fn move_sand(&mut self, current_pos: &Point, limited_width: bool) -> Option<Point> {
        let (x, y) = (current_pos.x, current_pos.y);
        let map = &mut self.data;
        if map[(x - self.offset) as usize][y as usize] == 'o' {
            return None;
        }
        if y as usize >= map[0].len() - 1 {
            return Option::None;
        } else if map[(x - self.offset) as usize][y as usize + 1] == '.' {
            return Option::Some(Point::new(x, y + 1));
        } else if x == 0 && limited_width {
            return Option::None;
        } else if x as i32 == self.offset && !limited_width {
            self.offset -= 1;
            let mut new_line = vec!['.'; map[0].len()-1];
            new_line.push('#');
            map.insert(0, new_line);
            return self.move_sand(current_pos, limited_width);
        } else if map[(x - self.offset) as usize - 1][y as usize + 1] == '.' {
            return Option::Some(Point::new(x - 1, y + 1));
        } else if x >= map.len() as i32 + self.offset - 1 && limited_width {
            return Option::None;
        } else if x >= map.len() as i32 + self.offset - 1 && !limited_width {
            let mut new_line = vec!['.'; map[0].len()-1];
            new_line.push('#');
            map.push(new_line);
            return self.move_sand(current_pos, limited_width);
        } else if map[(x - self.offset) as usize + 1][y as usize + 1] == '.' {
            return Option::Some(Point::new(x + 1, y + 1));
        } else {
            return Option::Some(Point::new(x, y));
        }
    }

    fn fill_with_sand(&mut self, start: &Point, limited_width: bool) {
        let mut pos = Option::Some(*start);
        while pos.is_some() {
            let mut did_move = true;
            pos = Option::Some(*start);
            while did_move && pos.is_some() {
                let new_pos = self.move_sand(&pos.unwrap(), limited_width);
                if new_pos.is_some() && pos.unwrap() == new_pos.unwrap() {
                    did_move = false;
                }
                pos = new_pos;
                
            }
            if pos.is_some() {
                let (x,y) = (pos.unwrap().x, pos.unwrap().y);
                self.data[(x - self.offset) as usize][y as usize] = 'o';
            }
        }
    }
}
fn parse_lines(path: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    for l in read_to_string(path).unwrap().lines() {
        let line: Line = l
            .split(" -> ")
            .map(|p| p.split_once(",").unwrap())
            .map(|(x, y)| Point::new(x.parse().unwrap(), y.parse().unwrap()))
            .collect();
        lines.push(line)
    }
    lines
}
fn display_map(map: &Vec<Vec<char>>) {
    let mut file = File::create("output.txt").unwrap();
    let mut output = String::new();
    let mut transposed: Vec<Vec<char>> = (0..map[0].len()).map(|_| Vec::new()).collect();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            transposed[y].push(map[x][y]);
        }
    }
    for y in 0..transposed.len() {
        for x in 0..transposed[0].len() {
            output.push(transposed[y][x]);
        }
        output.push('\n');
    }
    file.write_all(output.as_bytes()).unwrap();
}

fn main() {
    let lines = parse_lines("./input/task_1.txt");
    let mut map = Map::from_lines(&lines, false);
    let start = Point::new(500, 0);
    map.fill_with_sand(&start, true);
    let task_1_answer = map.data.iter()
    .flat_map(|e| e)
    .filter(|&e| *e == 'o')
    .count();
    println!("[Task 1] Number of sand corns: {}", task_1_answer);

    let mut map = Map::from_lines(&lines, true);
    map.fill_with_sand(&start, false);
    let task_2_answer = map.data.iter()
    .flat_map(|e| e)
    .filter(|&e| *e == 'o')
    .count();
    println!("[Task 2] Number of sand corns: {}", task_2_answer);
}