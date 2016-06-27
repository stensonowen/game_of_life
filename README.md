# game_of_life
Conway's Game of Life; I'm pretty much just looking for excuses to write things in Rust

This seems like a more interesting project than tetris. We'll see how complex it gets.

In order to make it slightly different from other similar projects, the plan is for it to be optimized for large boards and lots of iterations rather than simplicity and pretty printing.

## Progress

When I was skimming the [Wiki page](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) I wasn't sure how I'd implement an infinite game board. The obvious solution is to have a fixed board size that wraps around to itself, but I'm too much of a rebel to follow such trivial mores. Also, wrapping the board would interfere with certain [Game of Life patterns](https://upload.wikimedia.org/wikipedia/en/d/d1/Long_gun.gif) which rely on infinite dimensions to be accurate. 

The simplest implementation would be a vector of vectors (but 4 2d vectors would be more efficient because it is cheaper to append to 4 quadrants instead of prepend to possibly large vectors).
This is also pretty inefficient because it stores the presence and absence of each cell; the vector of vectors would be mostly empty space. There's probably a better solution.

I had initially worried about a hash table implementation because the frequent deletions could require long probes to look up. I started playing with a specialized implementation of [Dynamic Perfect Hashing](https://en.wikipedia.org/wiki/Dynamic_perfect_hashing), which would be an interesting way to solve the problem. 

Instead I started poking through the std::collections::Hashmap code and saw that it used [Robin Hood Hashing](https://en.wikipedia.org/wiki/Hash_table#Robin_Hood_hashing) (which I should have noticed when skimming the docs), which maintains a low variation of probe length. (More interesting reading [here](http://www.sebastiansylvan.com/post/robin-hood-hashing-should-be-your-default-hash-table-implementation/).) This should minimize cache misses and cap the number of addresses to inspect before concluding an entry has been deleted. 
To be relatively safe I threw a bunch of operations at a hashmap to see if they slowed. The [results](res_3.png) are interesting: despite occasional spikes (presumably when resizing the table or something), the minimum time doesn't really change. Therefore the Rust hash table implementation is not subject to the shortcoming I assumed it was! (Note: 1 'iteration' consisted of a few thousand random insert/deletes.)

