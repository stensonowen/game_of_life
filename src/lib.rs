use std::collections::HashSet;

struct Point {
    x: i32,
    y: i32,
}


impl Point {
    fn get_neighbors(&self) -> Vec<Point> {
        [(-1, 1), ( 0, 1), ( 1, 1),
         (-1, 0),          ( 1, 0),
         (-1,-1), ( 0,-1), ( 1,-1),]
             .iter()
             .map(|&(dx,dy)| Point{x: self.x+dx, y: self.y+dy})
             .collect()
    }
}


//type Board = std::collections::HashSet<Point>;
struct Board {
    cells: HashSet<Point>,
}

impl Board {

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
