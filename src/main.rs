
extern crate life;
extern crate rand;
extern crate terminal_size;
//use life::*;

const ITER: u32 = 1;

fn main() {

    let board_width:  i32;
    let board_height: i32;

	//use terminal_size::{Width, Height, terminal_size};
	//let size = terminal_size();
	//if let Some((Width(w), Height(h))) = size {
    //    //subtract to add border
    //    //divide by 2 because modulo can be + or -
    //    board_width  = (w as i32 - 6) / 2;
    //    board_height = (h as i32 - 4) / 2;
	//} else {
    //    board_width = 50;
    //    board_height = 25;
	//}

    board_width = 20;
    board_height = 10;

    let mut b = life::Board::new();

    for _ in 0..100 {
        let (x,y) = rand::random::<(i32,i32)>();
        b.push_pt(
            x % board_width,
            y % board_height);
    }

    //println!("{:?}", b);
    //b.print();

    println!("Started with {} alive", b.size());

    for _ in 0..ITER {
        b = b.iterate();
    }

    println!("Finished with {} alive after {} iterations", b.size(), ITER);

    for _ in 0..40 {

        b = b.iterate();
        std::thread::sleep(std::time::Duration::from_millis(400));
        b.print()
    }
    /*

    let b_ = b.clone();
    let mut count = 0;


    loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
        b = b.iterate();
        b.print();
        count += 1;

        if count > 300 {
            break;
        }
    }

    std::thread::sleep(std::time::Duration::from_secs(5));
    b_.print();
    */

}


/*
 *
 *       X
 *        X
 *     XXX
 */
