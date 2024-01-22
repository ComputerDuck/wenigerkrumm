use std::time::Instant;

use std::env;
use std::fs;

struct Vector2 {
    x: f32,
    y: f32,
}
impl Vector2 {
    fn new(a: &Point, b: &Point) -> Self {
        Vector2 {
            x: a.x - b.x,
            y: a.y - b.y,
        }
    }
    fn dot(&self, b: &Vector2) -> f32 {
        self.x * b.x + self.y * b.y
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    index: usize,
    x: f32,
    y: f32,
}
impl Point {
    fn new(index: usize, x: f32, y: f32) -> Self {
        Point { index, x, y }
    }
}

fn main() {
    let args = env::args().skip(1);

    for arg in args {
        let original: Vec<Point> = readfile(&arg);

        let mut dist_matrix: Vec<Vec<f32>> = vec![vec![0.0; original.len()]; original.len()];

        for i in 0..original.len() {
            for j in 0..original.len() {
                dist_matrix[i][j] = distance(original[i], original[j]);
            } 
        }

        let unvisited: Box<Vec<(&Point, bool)>> =
            Box::new(original.iter().map(|p| (p, false)).collect());

        // println!("running {}", arg);
        let start: Instant = Instant::now();
        let route = find_route(unvisited, &dist_matrix);
        let time = start.elapsed().as_secs_f64();

        println!("route:");
        for p in route.clone() {
            println!("{}", p.index);
        }

        let mut total: f32 = 0.;
        for i in 1..route.len() {
            total += dist_matrix[route[i].index][route[i-1].index];
        }
        println!("total distance: {}", total);
        println!("time: {}", time);
    }
}

fn find_route<'a>(
    unvisited: Box<Vec<(&'a Point, bool)>>,
    dist_matrix: &Vec<Vec<f32>>,
) -> Vec<&'a Point> {
    fn aux<'a>(
        mut start: usize,
        mut next: usize,
        mut unvisited: Box<Vec<(&'a Point, bool)>>,
        dist_matrix: &Vec<Vec<f32>>,
    ) -> Vec<&'a Point> {
        let mut route: Box<Vec<&Point>>= Box::new(vec![]);

        if start > unvisited.len() {
            return *route;
        }


        let first = unvisited[start];
        unvisited[start].1 = true;
        let second = unvisited[next];
        unvisited[next].1 = true;


        route.push(first.0);
        route.push(second.0);

        let (possible, route) = find_route_rek(route, unvisited.clone(), 2, dist_matrix);

        // unvisit previous points
        unvisited[start].1 = false;
        unvisited[next].1 = false;

        // choose next points
        next = (next + 1) % (unvisited.len()-1);
        if start == next {
            start += 1;
            next = start + 1;
        }

        // match the result
        match possible {
            true => *route.unwrap(),
            false => aux(start, next, unvisited, dist_matrix),
        }
    }

    aux(0, 1, unvisited, dist_matrix)
}

fn find_route_rek<'a>(
    mut route: Box<Vec<&'a Point>>,
    mut unvisited: Box<Vec<(&'a Point, bool)>>,
    visited: usize,
    dist_matrix: &Vec<Vec<f32>>,
) -> (bool, Option<Box<Vec<&'a Point>>>) {

    // if visited == unvisited.len() {
    //     return (true, Some(route))
    // }
    // println!("{}, {}", visited, unvisited.len());
    if unvisited.iter().find(|p| !p.1).is_none() {
        return (true, Some(route))
    }

    let pq = priority_queue(
        route[route.len() - 2],
        route[route.len() - 1],
        unvisited.clone(),
        dist_matrix,
    );
    // println!("{:?}: {:?}", route.last(), pq);
    route.iter().for_each(|p| print!("{} ", p.index));
    println!();
    // println!("{:?}", route);

    for p_pq in pq {
        route.push(p_pq);
        unvisited.get_mut(p_pq.index).unwrap().1 = true;

        let (possible, way) = find_route_rek(route.clone(), unvisited.clone(), visited+1, dist_matrix);
        if possible {
            return (true, way);
        }

        route.pop();
        unvisited.get_mut(p_pq.index).unwrap().1 = false;
    }

    (false, None)
}

fn priority_queue<'a>(
    prev: &Point,
    curr: &Point,
    unvisited: Box<Vec<(&'a Point, bool)>>,
    dist_matrix: &Vec<Vec<f32>>,
) -> Vec<&'a Point> {
    let mut queue: Vec<&Point> = vec![];

    let prev_vec = Vector2::new(curr, prev);
    let unv: Vec<&Point> = unvisited
        .iter()
        .filter(|(_, v)| !v)
        .map(|(p, _)| *p)
        .collect();

    for p in unv {
        let curr_vec = Vector2::new(p, curr); 
        if prev_vec.dot(&curr_vec) >= 0.0 {
            let curr_distance = dist_matrix[p.index][curr.index];
            let pos = queue.iter().position(|x| dist_matrix[x.index][curr.index] > curr_distance);
            match pos {
                 Some(i) => queue.insert(i, p),
                 None => queue.push(p),
            }
        }
    }

    queue
}


fn distance(point1: Point, point2: Point) -> f32 {
    ( (point2.x - point1.x).powf(2.0) + (point2.y - point1.y).powf(2.0) ).sqrt()
}

fn readfile(filename: &str) -> Vec<Point> {
    // open file
    let file = fs::read_to_string(filename).expect(&format!("Failed to open file {}", filename));

    // parse file
    let mut i = 0;
    file.lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .map(|p: Vec<f32>| {
            i += 1;
            Point::new(i - 1, p[0], p[1])
        })
        .collect()
}
