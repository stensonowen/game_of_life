// requires nightly:
#![feature(test)]
extern crate test;

extern crate rand;

use std::collections::HashSet;

//should this just be a tuple?
// either   type Point = (i32,i32)
// or       Struct Point (i32,i32)
#[derive(Eq, PartialEq, Hash)]
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

    fn get_neighbors2(&self) -> [(i32,i32); 8] {
        //on stack instead
        [(self.x-1, self.y+1),
         (self.x  , self.y+1),
         (self.x+1, self.y+1),
         (self.x-1, self.y  ),
         (self.x+1, self.y  ),
         (self.x-1, self.y-1),
         (self.x  , self.y-1),
         (self.x+1, self.y-1)]
    }
}


type Life = HashSet<Point>;    //catalog of living colonies

struct Board {
    min  : Point,       //top-left corner of board
    max  : Point,       //top-right corner
    cells: Life,
}


impl Board {
    fn push1(&mut self, p: &Point) {
        self.min = Point {
            x: std::cmp::min(self.min.x, p.x),
            y: std::cmp::min(self.min.y, p.y),
        };
        self.max = Point {
            x: std::cmp::max(self.max.x, p.x),
            y: std::cmp::max(self.max.y, p.y),
        };
    }

    fn push2(&mut self, p: &Point) {
        if p.x < self.min.x { self.min.x = p.x; }
        if p.y < self.min.y { self.min.y = p.y; }
        if p.x < self.max.x { self.max.x = p.x; }
        if p.y < self.max.y { self.max.y = p.y; }


    }

    fn blank() -> Board {
        //basic shape: 1*3 block
        let mut life = Life::new();
        life.insert(Point{x:-1, y:-1});
        life.insert(Point{x:-1, y: 0});
        life.insert(Point{x:-1, y: 1});
        life.insert(Point{x: 1, y:-1});
        life.insert(Point{x: 1, y: 0});
        life.insert(Point{x: 1, y: 1});

        Board {
            min: Point { x:-2, y:-2},
            max: Point { x: 2, y: 2},
            cells: life
        }

    }


        

}


#[cfg(test)]
mod tests {

    //use super::*;
    use rand;
    use super::Point;
    use super::Board;

    #[test]
    fn it_works() {

    }

    use test::Bencher;
    //benchmarks require nightly
    #[bench]
    fn gn1(b: &mut Bencher) {
        let (x,y) = rand::random::<(i32,i32)>(); 
        let p = Point{x:x, y:y};
        b.iter(|| {
            let _ = p.get_neighbors();
        });
    }
    #[bench]
    fn gn2(b: &mut Bencher) {
        let (x,y) = rand::random::<(i32,i32)>(); 
        let p = Point{x:x, y:y};
        b.iter(|| {
            let _ = p.get_neighbors2();
        });
    }
    //#[bench]
    fn bm_push1(b: &mut Bencher) {
        //65 ns/iter +/- 6
        let mut brd = super::Board::blank();
        b.iter(|| {
            let (x,y) = rand::random::<(i32,i32)>(); 
            brd.push1(&Point{x:x,y:y});
        });
    }
    //#[bench]
    fn bm_push2(b: &mut Bencher) {
        //63 ns/iter +/- 2
        let mut brd = super::Board::blank();
        b.iter(|| {
            let (x,y) = rand::random::<(i32,i32)>(); 
            brd.push2(&Point{x:x,y:y});
        });
    }

}

