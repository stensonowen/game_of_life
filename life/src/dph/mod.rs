//Dynamic Perfect Hashing: https://en.wikipedia.org/wiki/Dynamic_perfect_hashing

mod bucket;
use dph::bucket::Bucket;

pub mod dph_table {
    use dph::bucket::Bucket;
    pub const FOO: u32 = 42;
    //pub const BAR: Bucket = Bucket{x: true};

    fn foo() {
        let a : Bucket;
    }

}
