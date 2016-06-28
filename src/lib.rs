// requires nightly:
//#![feature(test)]
//extern crate test;

extern crate rand;
extern crate terminal_size;

use std::collections::{HashSet,HashMap};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Point(pub i32, pub i32);  //(x,y)


impl Point {
    fn from_clone(p: &Point) -> Point {
        //generate duplicate
        //handy in avoiding ownership/lifetime issues
        Point(p.0,p.1)
    }
    fn neighbors(&self) -> Vec<Point> { //vs [Point;8] ?
        //all neighboring points to this one
        let Point(x,y) = *self;
        vec![   Point(x-1,y+1), Point(x  ,y+1), Point(x+1,y+1),
                Point(x-1,y  ),                 Point(x+1,y  ),
                Point(x-1,y-1), Point(x  ,y-1), Point(x+1,y-1)]
    }
    pub fn min() -> Point {
        Point(std::i32::MAX,std::i32::MAX)
    }
    pub fn max() -> Point {
        Point(std::i32::MIN,std::i32::MIN)
    }
}


type Cells = HashSet<Point>;    //catalog of living colonies

#[derive(Clone, PartialEq, Debug)]
pub struct Board {
    min  : Point,       //top-left corner of board
    max  : Point,       //top-right corner
    alive: Cells,
}


impl Board {

    pub fn new() -> Board {
        Board {
            min:    Point::min(),
            max:    Point::max(),
            alive:  Cells::new(),
        }
    }

    pub fn push(&mut self, p: &Point) {
        if p.0 < self.min.0 { self.min.0 = p.0; }
        if p.1 < self.min.1 { self.min.1 = p.1; }
        if p.0 > self.max.0 { self.max.0 = p.0; }
        if p.1 > self.max.1 { self.max.1 = p.1; }
        self.alive.insert(Point::from_clone(p));
    }

    pub fn iterate(self) -> Board {
        //update everything that has to be updated
        // keep track of which cells live on to the next tick
        let mut new_board = Board {
            min:    Point::min(),
            max:    Point::max(),
            alive:  Cells::with_capacity(self.alive.len()),
        };
        // keep track of which dead cells might become alive
        let mut dead_neighbors = HashMap::with_capacity(self.alive.len()*8);
        //check on each living cell
        for cell in &self.alive {
            let neighbors: Vec<Point> = cell.neighbors();
            let mut live_neighbors = 0u8;
            for p in &neighbors {
                if self.alive.contains(&p) {
                    live_neighbors += 1;
                } else {
                    let count = dead_neighbors
                        .entry(Point::from_clone(p))
                        .or_insert(0u8);
                    *count += 1;
                }
            }
            //alive cell does not die to either over- or under-population 
            if live_neighbors == 2 || live_neighbors == 3 {
                new_board.push(cell);
            }
        }
        for (p, &count) in &dead_neighbors {
            //new cell is born at an empty point, as if by reproduction
            if count == 3 {
                new_board.push(p);
            }
        }
        
        new_board
    }

    pub fn sample() -> Board {
        //basic shape: 2 1x3 blocks
        let mut cells = Cells::new();
        cells.insert(Point(-1, -1));
        cells.insert(Point(-1,  0));
        cells.insert(Point(-1,  1));
        cells.insert(Point( 1, -1));
        cells.insert(Point( 1,  0));
        cells.insert(Point( 1,  1));

        Board {
            min:    Point(-1,-1),
            max:    Point( 1, 1),
            alive:  cells,
        }

    }


    pub fn print(&self) {
        //TODO: use braille
        // :YcmRestartServer ?
        use terminal_size::{Width, Height, terminal_size};

        //let board_width = self.max.
        let Point(min_x, min_y) = self.min;
        let Point(max_x, max_y) = self.max;
        let board_width     = max_x.checked_sub(min_x);
        let board_height    = max_y.checked_sub(min_y);

        if board_width.is_none() || board_height.is_none() 
                || board_height.unwrap() > std::u16::MAX as i32 
                || board_width.unwrap()  > std::u16::MAX as i32 {
            println!("Board is too big to print;");
            return;
        }
        let board_width  =  board_width.unwrap() as u16;
        let board_height = board_height.unwrap() as u16;

        let term_size = terminal_size();
        if term_size.is_none() {
            println!("Failed to fetch terminal size;");
            return;
        }
        let (Width(term_width), Height(term_height)) = term_size.unwrap();

        assert!(term_width  >  board_width);
        assert!(term_height > board_height);

        let x_buf = (term_width  -  board_width) / 2;
        let y_buf = (term_height - board_height) / 2;

        let mut s = String::with_capacity(term_width as usize
                                        * term_height as usize);

        //print top spaces
        for _ in 1..y_buf {         s.push_str("\n");   }

        //print top line
        for _ in 1..x_buf {         s.push_str(" ");    }
        s.push_str("+");
        for _ in 0..board_width+1 { s.push_str("-");    }
        s.push_str("+\n");

        //print remainder of board (including vertical lines
        //for y in 0..board_height {
        for y in self.min.1 .. self.max.1+1 {
            for _ in 1..x_buf {     s.push_str(" ");    }
            s.push_str("|");
            //for x in 0..board_width {
            for x in self.min.0 .. self.max.0+1 {
                let p = Point(x as i32, y as i32);
                if self.alive.contains(&p) {
                    s.push_str("X");
                } else {
                    s.push_str(" ");
                }
            }
            s.push_str("|\n");
        }

        //print bottom line
        for _ in 1..x_buf {         s.push_str(" ");    }
        s.push_str("+");
        for _ in 0..board_width+1 { s.push_str("-");    }
        s.push_str("+\n");
            
        for _ in 1..y_buf {         s.push_str("\n");   }


        //braille version:
        //assert!(2.0 * term_width  as f64 >  board_width);
        //assert!(4.0 * term_height as f64 > board_height);
        println!("board min: ({}, {})", self.min.0, self.min.1);
        println!("board max: ({}, {})", self.max.0, self.max.1);
        println!("board width: {}", board_width);
        println!("board heiht: {}", board_height);
        println!("term width:  {}", term_width);
        println!("term height: {}", term_height);

        
        print!("{}", s);


    }
        

}


#[cfg(test)]
mod tests {

    //use super::*;
    //use rand;
    use super::Point;
    use super::Board;
    use super::Cells;


    #[test]
    fn pattern_block() {
        //2x2 square should stay the same
        let mut b = Board::new();
        b.push(&Point(0,0));
        b.push(&Point(0,1));
        b.push(&Point(1,1));
        b.push(&Point(1,0));
        assert!(b.clone() == b.iterate());
    }

    #[test]
    fn pattern_loaf() {
        //https://en.wikipedia.org/wiki/File:Game_of_life_loaf.svg
        //static ~4 x ~4 pattern
        let mut b = Board::new();
        b.push(&Point(1,3));
        b.push(&Point(2,2));
        b.push(&Point(3,1));
        b.push(&Point(4,2));
        b.push(&Point(4,3));
        b.push(&Point(2,4));
        b.push(&Point(3,4));
        assert!(b.clone() == b.iterate());
    }

    #[test]
    fn pattern_blinker() {
        //https://en.wikipedia.org/wiki/File:Game_of_life_blinker.gif
        //a 1x3 block that rotates 90 degrees every 1 iteration
        //also test min/max behavior
        let mut b1 = Board::new();
        b1.push(&Point(0,-1));
        b1.push(&Point(0, 0));
        b1.push(&Point(0, 1));
        //verify b1's min+max
        assert!(b1.min == Point( 0,-1));
        assert!(b1.max == Point( 0, 1));
        let b2 = b1.clone().iterate();
        //verify b2's min+max
        assert!(b2.min == Point(-1, 0));
        assert!(b2.max == Point( 1, 0));
        //verify third iteration identical to first
        assert!(b1 == b2.iterate());
    }

        
    use test::Bencher;
    use rand;
    #[bench]
    fn iter(ben: &mut Bencher) {
        //10k cells takes 561 microseconds/iter
        let mut b = Board::sample();
        let max = 1000;
        for _ in 0..10000 {
            let (x,y) = rand::random::<(i32,i32)>();
            b.push(&Point(x%max, y%max));
        }
        ben.iter(|| b.iter2());
    }

    //use test::Bencher;
    //benchmarks require nightly
    //#[bench]
    //fn iter(bencher: &mut test::Bencher) {
    //    bencher.iter(|| {
    //        let mut b = Board::sample();
    //        for _ in 0..10 {
    //            b.iterate();
    //        }
    //    });
    //}
    //#[bench]
    //fn iter_(bencher: &mut test::Bencher) {
    //    bencher.iter(|| {
    //        let mut b = Board::sample();
    //        for _ in 0..10 {
    //            b = b.iterate_();
    //        }
    //    });
    //}

}

