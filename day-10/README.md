# Day 10

Today, our problem features an animal in a pipe. 

The pipes are laid on the surface and our input is ASCII art: north-south `|`, east-west `-`, the four cardinal corners NE `L`, NW `J`, SE `F`, SW `7`, an empty space `.` and our starting position `S` (which is actually a pipe piece).

For part 1, we need to find the farthest point in the pipe from our starting position. We are given that there is a loop of pipes, such that there are no extraneous junctions. However, there are lots of other bits of pipe, which we'll need to ignore. 

This is helpful because we can identify which piece `S` is by examining its four neighbours - exactly two of them should try and link up. Manual review of my real puzzle input confirmed this to be the case. 

## Loading the input

I implemented an adjacency map: given a position, report the neighbouring positions which are connected. 

I did this for the entire map, though in retrospect I could've probably combined this with part 1 proper. 

Points are a 2-element array of integer coordinates. Some free functions provide for addition & subtraction (unused) and multiplication & division by a scalar factor.

## Part 1 

Given the starting coordinates and the adjacency map, we needed to find the point in the pipe-loop furthest from the starting position. 

This seemed like an obvious job for some sort of flood fill, which I implemented with the usual breadth-first-search approach. The output is a map of position to manhattan distance from the starting point, which also happens to be the step count for the flood fill. The maximum value in the map is our answer.

BFS was actually a bit overkill here, since each tile in the loop has exactly two neighbours, it would've been sufficient to go around the loop and then halve the length.

## Part 2

Now I needed to find the area *enclosed* by the pipe-loop. This had a complication in that a zero-width gap between pipes counts for a connection to the outside! 

    F----7
    |F--7|
    ||OO||
    |L7FJ|
    L-JL-J
    OOOOOO
  
In the above diagram, every `O` identifies an "outside" tile. Even the `OO` in the third row counts as outside, because if you could squeeze between the pipes south, you could get to the other `O`s at the southern edge. 

  
When I reviewed the subreddit the following day I saw that many people had implemented a neat scanline-type approach (start on the outside, cross a pipe, now you're on the inside, cross to outside again...) but I did something a bit more basic. 

I scaled up the map by 2x, inserting new lengths of pipe as needed:

    F---------7
    |.........|
    |.F-----7.|
    |.|.....|.|
    |.|.O.O.|.|
    |.|.....|.|
    |.L-7.F-J.|
    |...|.|...|
    L---J.L---J
    O.O.O.O.O.O

Each `O` above corresponds to one in the previous diagram and each `.` is a non-pipe tile. 

Now we can run a flood fill from the known-outside spots, and then to scale back down we can count *only* the tiles which correspond to an original space (in my implementation, their X and Y coordinates are both even).

Since non-loop pipe should count as empty space for this, I performed the scale-up by repeating my BFS approach from part 1's flood fill. This had the nice property that only loop tiles were in the output. 

Now, rather than distances, my output was the new, scaled-up adjacency map. Given two connected neighbours in 1x space, we can create their new in-between neighbour in 2x space at the average of their two positions. It's necessary to include the in-between neighbours to keep the loop connected!

Finally, we identify the bounds of the grid, identify any non-loop tiles on the edge (which are our starting "outside" positions) and run a flood fill. This time we don't care about whether neighbours are pipe-connected or not, just if they're adjacent and not part of the pipe-loop. 

After a few mis-steps with (lack of) bounds checking and repeatedly putting the same items in the queue, it all worked! 
