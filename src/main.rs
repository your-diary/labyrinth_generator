use rand::Rng;
use std::collections::HashSet;

type WallMap = Vec<Vec<bool>>;

const IS_DEBUG_MODE: usize = 0;

fn debug_print<T: std::fmt::Debug>(t: &T) {
    if (IS_DEBUG_MODE != 0) {
        println!("{:?}", t);
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

fn print_map(m: &WallMap) {
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            let s = if (m[i][j]) { "■" } else { " " };
            if (j == 0) {
                print!("{}", s);
            } else {
                print!(" {}", s);
            }
        }
        println!();
    }
}

fn print_map_csv(m: &WallMap) {
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            let s = if (m[i][j]) { 1 } else { 0 };
            if (j == 0) {
                print!("{}", s);
            } else {
                print!(",{}", s);
            }
        }
        println!();
    }
}

//Checks if we can dig into the direction `d` from the current position `p`.
//
//`true` is returned iff the following two conditions are met.
//1. The new position `p + d` is wall.
//2. Three of the four adjacent positions around `p + d` are walls.
fn is_digable(p: &Point, d: &Direction, m: &WallMap) -> bool {
    let mut i = p.x;
    let mut j = p.y;

    let h = m.len();
    let w = m[0].len();

    //calculates the new position `p + d`
    match d {
        Direction::Up => i -= 1,
        Direction::Down => i += 1,
        Direction::Left => j -= 1,
        Direction::Right => j += 1,
    };

    if !m[i][j] {
        return false;
    }
    //When the number of the adjacent points is less than `4`.
    if !((0 < i && i < h - 1) && (0 < j && j < w - 1)) {
        return false;
    }

    let adjacent_points = vec![
        Point::new(i - 1, j),
        Point::new(i + 1, j),
        Point::new(i, j - 1),
        Point::new(i, j + 1),
    ];

    let mut wall_count = 0;
    for p in adjacent_points {
        if (m[p.x][p.y]) {
            wall_count += 1;
        }
    }

    (wall_count == 3)
}

fn main() {
    let h = 10; //height
    let w = 20; //width

    //contains information about if or not each position is wall
    //Initially, all of the positions are walls.
    //Additionally, we reserve the boundaries ((0, 0), (0, 1), (0, 2), etc.) as walls.
    let mut m: WallMap = vec![vec![true; w]; h]; //is wall

    //for performance
    //When none of the four directions is dig-able, a point is lazily removed from this set.
    let mut points_to_be_scanned: HashSet<Point> = HashSet::new();
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            points_to_be_scanned.insert(Point::new(i, j));
        }
    }

    let mut rng = rand::thread_rng();

    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    let mut p = Point::new(1, 1);
    m[p.x][p.y] = false;

    loop {
        debug_print(&p);

        let possible_directions: Vec<Direction> = directions
            .iter()
            .copied()
            .filter(|d| is_digable(&p, d, &m))
            .collect();

        if (possible_directions.is_empty()) {
            points_to_be_scanned.remove(&p);
            if (points_to_be_scanned.is_empty()) {
                break;
            }
            let index = rng.gen_range(0..points_to_be_scanned.len());
            p = *points_to_be_scanned.iter().nth(index).unwrap();
        } else {
            let index = rng.gen_range(0..possible_directions.len());
            match possible_directions[index] {
                Direction::Up => p.x -= 1,
                Direction::Down => p.x += 1,
                Direction::Left => p.y -= 1,
                Direction::Right => p.y += 1,
            }
            m[p.x][p.y] = false;
        }
    }

    print_map(&m);
    println!();
    print_map_csv(&m);
}
