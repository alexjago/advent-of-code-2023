# Day 11

Today, we're expanding galaxies.

Our input is an ASCII-art diagram: `#` is a galaxy, `.` is empty space. 

Due to reasons, we must expand space between galaxies: when a column or row is empty, it should be duplicated:

    #...#
    ..#..
    .....
    ...#.

becomes...

    #....#
    ...#..
    ......
    ......
    ....#.

There are several things to do here. 

Firstly, we read the input and convert to a set of 2D positions marking out the galaxies. 

Then, we iterate over our set - fully, not sparsely - to identify empty rows and columns. 

There's a trick to the expansion: suppose a galaxy is to the right of three empty columns. This means, post-expansion, it will be 3 columns further right! (Same principle if it's below empty rows.)

This allows us to model the expansion just by updating galaxy positions according to how many empty rows and columns they are down or to the right of, rather than drawing a whole new grid.

Finally, we need to iterate over each pair of galaxies, identify the Manhattan distance between them, and report the sum of all those distances. 

Some of the 2D iterations were surpringly challenging due to some complexities around `flat_map` and the borrow checker. I ended up doing those more imperatively. 

## Part 2

Now instead of just duplicating empty columns and rows, we need to have 1 million of them!

Thankfully, our expansion trick still works: if a galaxy is to the right of three empty columns, then post-expansion it will be 2,999,997 columns further right (3 columns, times 1 million expansion factor, minus the three original columns).

This means that our datastructure - our set of galaxies - is exactly the same size as it was in part 1. The values in it are just bigger numbers.

We report the same sum-of-paired-Manhattan-distances as in part 1. 

This was quite a satisfying part 2. All I needed to do was refactor the code slightly to allow for a variable expansion factor and to extract out the pair-distance code, add the tests for the new examples, and the result was correct first try. 
