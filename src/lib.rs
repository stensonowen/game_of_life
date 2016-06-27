// requires nightly:
//#![feature(test)]
//extern crate test;

extern crate rand;

use std::collections::HashSet;

//should this just be a tuple?
// either   type Point = (i32,i32)
// or       Struct Point (i32,i32)
#[derive(Eq, PartialEq, Hash)]
struct Point(i32,i32);  //(x,y)


impl Point {
    fn get_neighbors(&self) -> Vec<Point> { //vs [Point;8] ?
        let Point(x,y) = *self;
        vec![   Point(x-1,y+1), Point(x  ,y+1), Point(x+1,y+1),
                Point(x-1,y  ),                 Point(x+1,y  ),
                Point(x-1,y-1), Point(x  ,y-1), Point(x+1,y-1)]
    }
}


type Life = HashSet<Point>;    //catalog of living colonies

struct Board {
    min  : Point,       //top-left corner of board
    max  : Point,       //top-right corner
    cells: Life,
}


impl Board {
    fn push(&mut self, p: &Point) {
        if p.0 < self.min.0 { self.min.0 = p.0; }
        if p.1 < self.min.1 { self.min.1 = p.1; }
        if p.0 < self.max.0 { self.max.0 = p.0; }
        if p.1 < self.max.1 { self.max.1 = p.1; }
    }

    fn blank() -> Board {
        //basic shape: 1*3 block
        let mut life = Life::new();
        life.insert(Point(-1, -1));
        life.insert(Point(-1,  0));
        life.insert(Point(-1,  1));
        life.insert(Point( 1, -1));
        life.insert(Point( 1,  0));
        life.insert(Point( 1,  1));

        Board {
            min: Point(-2,-2),
            max: Point( 2, 2),
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

