
extern crate life;
extern crate rand;
//use rand;
//use life::*;

fn main() {

    let mut b = life::Board::new();
    let max_x = 55;
    let max_y = 20;

    for _ in 0..2 {
        let (x,y) = rand::random::<(i32,i32)>();
        let p = life::Point(
            x % max_x,
            y % max_y);
        b.push(&p);
    }

    println!("{:?}", b);

    b.print();
}
