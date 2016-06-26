# game_of_life
Conway's Game of Life; I'm pretty much just looking for excuses to write things in Rust

This seems like a more interesting project than tetris. We'll see how complex it gets.

## Progress

When I was skimming the [Wiki page](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) I wasn't sure how I'd implement an infinite game board. The obvious solution is to have a fixed board size that wraps around to itself, but I'm too much of a rebel to follow such trivial mores. Also, wrapping the board would interfere with certain [Game of Life patterns](https://upload.wikimedia.org/wikipedia/en/d/d1/Long_gun.gif) which rely on infinite dimensions to be accurate. 

The simplest implementation would be a vector of vectors (but 4 2d vectors would be more efficient because it is cheaper to append to 4 quadrants instead of prepend to possibly large vectors).
This is also pretty inefficient because it stores the presence and absence of each cell; the vector of vectors would be mostly empty space. There's probably a better solution.

An ideal data structure could be iterated over, would have very fast lookup times, would have quite fast insert/remove times, and could store a large range of values. The aforementioned implementation is pretty good at all of these but the last, and would start to choke if alive cells got too far away from each other. 

A hash table fits almost all of these criteria, but I think would have a fatal flaw. Assuming the table isn't absurdly large, there would be relatively frequent collisions. If an element is to be inserted but it collides with an already present element, it some collision handling algorithm will dictate another place for it to be stored (e.g. the next location). However, if the other element is removed, then lookup of the recently inserted element involves looking at the location it hashes to, realizing an element was deleted from that location, and following the collision handling algorithm until it finds the desired element or finds an empty space that an element was not deleted from. This makes it terribly unsuited to a situation like this in which elements are inserted and deleted extremely frequently.  

Some kind of tree might be better. A `O(log(n))` lookup time isn't terrible, considering the hash table could eventually exceed `O(n)` and there won't necessarily be tons of alive cells.
