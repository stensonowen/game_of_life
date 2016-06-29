
extern crate life;
extern crate rand;
//use rand;
//use life::*;

fn main() {

    let mut b = life::Board::new();
    let max_x = 55;
    let max_y = 25;

    for _ in 0..1000 {
        let (x,y) = rand::random::<(i32,i32)>();
        b.push_pt(
            x % max_x,
            y % max_y);
    }

    //println!("{:?}", b);

    b.print();
}
