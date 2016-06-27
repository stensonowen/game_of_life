//Bucket: interior hash table in DPH

pub struct Cell {
    x: i32,
    y: i32,
}

pub struct Bucket {
    //multiplier and increment
    hash:   (i32, i32),         //u32 better?
    size:   usize,
    table:  Vec<Option<Cell>>,
}

impl Bucket {

    fn new() -> Bucket {
        Bucket{
            hash : (33, 1),
            size : 4,
            table: vec![]
        }
    }

    fn lcg(&self, c: &Cell) -> usize {
        //https://en.wikipedia.org/wiki/Linear_congruential_generator
        //self.hash is composed of the multiplier and the seed respectively
        //the increment is each of the values of the Cell, as in djb2
        //  http://www.cse.yorku.ca/~oz/hash.html
        //used to hash cell coordinates into a vector index
        let mut hash: i32 = self.hash.1;        //5381
        hash = hash.wrapping_mul(self.hash.0)   //33
            .wrapping_add(c.x);
        hash = hash.wrapping_mul(self.hash.1)   //33
            .wrapping_add(c.y);
        (hash % self.size as i32) as usize
    }
}

