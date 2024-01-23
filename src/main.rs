use std::time::Instant;

use std::env;
use std::fs;

/// represents a two dimensional Vector
struct Vector2 {
    /// displacement in x direction
    x: f32,
    /// displacement in y direction
    y: f32,
}
impl Vector2 {
    /// Creates a new Vector from two Points
    ///
    /// # Arguments
    ///
    /// * `a` - reference to an initial Point (initial position)
    /// * `b` - reference to a final Point (displacement position)
    ///
    /// # Example 
    /// ```
    /// let p1 = Point::new(0,0,0);
    /// let p2 = Point::new(1,1,1);
    ///
    /// let vector = Vector2::from(p1, p2);
    /// ```
    fn from(a: &Point, b: &Point) -> Self {
        Vector2 {
            x: a.x - b.x,
            y: a.y - b.y,
        }
    }
    /// Calculates the dot product from itself and another Vector
    ///
    /// # Arguments
    ///
    /// * `b` - reference to a Vector to calculate the dot product from
    ///
    /// # Example
    ///
    /// ```
    /// let p1 = Point::new(0,0.,0.);
    /// let p2 = Point::new(0,0.,1.);
    /// let p3 = Point::new(0,1.,0.);
    ///
    /// let vector1 = Vector2::from(p1, p2);
    /// let vector2 = Vector2::from(p1, p3);
    ///
    /// assert!(vector1.dot(&vector), 0);
    /// ```
    fn dot(&self, b: &Vector2) -> f32 {
        self.x * b.x + self.y * b.y
    }
}

#[derive(Debug, Clone, Copy)]
/// represents a point on a two-dimensional grid
struct Point {
    /// index of the point
    index: usize,
    /// x position
    x: f32,
    /// y position
    y: f32,
}
impl Point {
    // creates a new point from an index and two coordinates
    //
    // # Arguments 
    // * `index` - index of the point
    // * `x` - x position of the point
    // * `y` - y position of the point
    //
    // # Example 
    // ```
    // let point = Point::new(0,1.,1.);
    // ```
    fn new(index: usize, x: f32, y: f32) -> Self {
        Point { index, x, y }
    }
}

fn main() {
    let args = env::args().skip(1);

    for arg in args {
        // read each point in the file into a Vector of points
        let original: Vec<Point> = readfile(&arg);

        // generate a matrix of the distances between all the points
        let mut dist_matrix: Vec<Vec<f32>> = vec![vec![0.0; original.len()]; original.len()];
        for i in 0..original.len() {
            for j in 0..original.len() {
                dist_matrix[i][j] = distance(original[i], original[j]);
            } 
        }

        // generate list of unvisited points, which have a boolean indicating wether they have been
        // visited or not
        let unvisited: Vec<(&Point, bool)> =
            original.iter().map(|p| (p, false)).collect();

        println!("calculating route. This may take a few seconds!");

        // timer start
        let start: Instant = Instant::now();

        // run nearest neighbour algorithm
        let route = find_route(unvisited, &dist_matrix);

        // stop time
        let time = start.elapsed().as_secs_f64();

        // PRINTING THE RESULTS
        // print the found route
        println!("route: ");
        for p in route.clone() {
            println!("    {}: {} {}", p.index, p.x, p.y);
        }

        // calculate distance of route
        let mut total: f32 = 0.;
        for i in 1..route.len() {
            total += dist_matrix[route[i].index][route[i-1].index];
        }
        println!("total distance: {}km", total);
        println!("time: {}s", time);
    }
}

/// calculates the find_route. Searches for route recursively from every possible starting point
///
/// # Arguments
/// * `unvisited` - list of all points and wether they have been visited
/// * `dist_matrix` -  matrix of all point distances
fn find_route<'a>(
    mut unvisited: Vec<(&'a Point, bool)>,
    dist_matrix: &Vec<Vec<f32>>,
) -> Vec<&'a Point> {
    // starting points
    let mut start: usize = 0;
    let mut next: usize = 1;

    // initialize empty route
    let mut route: Vec<&Point> = vec![];

    // check every possible combination of starting points
    while start < unvisited.len() {
        route.clear();

        // visit first two points
        let first = unvisited[start];
        unvisited[start].1 = true;
        let second = unvisited[next];
        unvisited[next].1 = true;

        route.push(first.0);
        route.push(second.0);

        // find route recursively and return when a route has been found
        if find_route_rek(&mut route, &mut unvisited, dist_matrix) {
            break;
        }
        
        // unvisit previous points
        unvisited[start].1 = false;
        unvisited[next].1 = false;

        // choose next points
        next = (next + 1) % (unvisited.len()-1);
        if start == next {
            start += 1;
            next = start + 1;
        }

    }

    route
}

/// try all possible routes recursively
///
/// # Arguments
/// * `route` - reference to current route
/// * `unvisited` - list of all points and wether they have been visited
/// * `dist_matrix` -  matrix of all point distances
fn find_route_rek<'a, 'b: 'a>(
    route: &mut Vec<&'a Point>,
    unvisited: &mut Vec<(&'b Point, bool)>,
    dist_matrix: &Vec<Vec<f32>>,
) -> bool {
    // if all points have been visited, return true (route found)
    if route.len() == unvisited.len() {
        return true;
    }

    // get queue of prioritized points (closest points)
    let pq = priority_queue(
        route[route.len() - 2],
        route[route.len() - 1],
        unvisited,
        dist_matrix,
    );

    for p_pq in pq {
        // visit point
        route.push(p_pq);
        unvisited.get_mut(p_pq.index).unwrap().1 = true;

        // find next points
        if find_route_rek(route, unvisited, dist_matrix) {
            return true;
        }

        // unvisit point
        route.pop();
        unvisited.get_mut(p_pq.index).unwrap().1 = false;
    }

    false
}

/// find all visitable points (angle > 90deg) and sort by closest
///
/// # Arguments
/// * `prev` - point before current point
/// * `curr` - current point
/// * `unvisited` - list of all points and wether they have been visited
/// * `dist_matrix` -  matrix of all point distances
fn priority_queue<'a>(
    prev: &Point,
    curr: &Point,
    unvisited: &Vec<(&'a Point, bool)>,
    dist_matrix: &Vec<Vec<f32>>,
) -> Vec<&'a Point> {
    // calculate vector between previous and current point
    let prev_vec = Vector2::from(curr, prev);
    // get all unvisited points in a Vector
    let unv: Vec<&Point> = unvisited
        .iter()
        .filter(|(_, v)| !v)
        .map(|(p, _)| *p)
        .collect();

    let mut queue: Vec<&Point> = vec![];
    for p in unv {
        // calulate vector between current and next point to check angle
        let curr_vec = Vector2::from(p, curr); 
        // check wether angle is more or equals to 90deg
        if prev_vec.dot(&curr_vec) >= 0.0 {
            let curr_distance = dist_matrix[p.index][curr.index];
            // sort point into queue dependent on distance
            let pos = queue.iter().position(|x| dist_matrix[x.index][curr.index] > curr_distance);
            match pos {
                 Some(i) => queue.insert(i, p),
                 None => queue.push(p),
            }
        }
    }

    queue
}

/// calculate distance between two points
///
/// # Arguments
/// * `a` - first point
/// * `b` - second point
fn distance(a: Point, b: Point) -> f32 {
    ( (b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0) ).sqrt()
}

/// read file and parse it to a Vector of points
///
/// # Arguments
/// * `filename` - name of the example file to parse
fn readfile(filename: &str) -> Vec<Point> {
    // open file
    let file = fs::read_to_string(filename).expect(&format!("Failed to open file {}", filename));

    // parse file
    let mut i = -1;
    file.lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .map(|p: Vec<f32>| {
            i += 1;
            Point::new(i as usize, p[0], p[1])
        })
        .collect()
}
