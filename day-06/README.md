# Day 6

Today we're racing tiny model boats!

The general principle is that the race has a time limit, `A`, and a distance to beat, `D`. At the start of the race, you can hold down a button for `0 ≤ t ≤ A` milliseconds, which increases the boat's speed by `1 ms per ms` that the button is held down. 

Hold down the button for too little time and your boat will be slow. Hold for too long and your boat will be fast but starting very late.

There's a happy medium.

## Part 1

For Part 1 we are given a list of times and corresponding distances-to-beat for several races. We need to count the number of ways to succeed in each race (number of whole milliseconds the button can be held down) and report the product of those counts. 

The input here was given in column-major form, and parsing was quite straightforward. 

I chose to implement a function which, given a total time, returned a list of distances, indexed by `t`. Generating this list for each total time, filtering out the too-short distances, and then taking the product of the counts, sufficed.


## Part 2

Surprise! There's actually just one, very long race - what we thought were spaces between existing races were kerning issues. 

At this point I turned to calculus. We can represent this problem as finding `t` for `(A - t) * t > D`. 

Rewrite as `(A - t) * t - D = 0` and we can use the quadratic formula. Then the integer values of `t` at or on the inside of the roots are the bounds of the button-hold times. 

I actually didn't fully do this in code in Rust, since it was just two equations and doing numeric conversions is painful. Rather, I turned to `bc`. It took a couple of submission attempts to get the rounding right, but a gold star is a gold star.  
