 # Day 9

Today's problem was based around successive differences.

Given a sufficiently long sequence `P(k), P(k+1), P(k+2), ...` where `P` is a polynomial, we can take the difference between successive terms in `P` to get `P'(k), P'(k+1), P'(k+2), ...`, then the difference between *those* terms to get `P''(k), P''(k+1), P''(k+2), ...`. Once the differences are all zeroes, we can stop. 

The Triangle Numbers `P(x) = (x)(x-1)/2 = 0.5*x^2 - 0.5*x + 0` provide a nice demonstration:

    0   1    3    6   10   15   21   ...
      1    2   3    4    5    6   ...
         1   1    1    1   1   ...
           0    0   0    0  ...

Our input is several sequences (the top row of the diagram). 

## Part 1

For Part 1 we need to get the *next* value in each sequence, added to the next value in each successive difference. Our puzzle answer is the sum of the next terms of each sequence. 

The solution was fairly straightforward. 

I represented the data as a map from the "level" to a list of values (`P` is level 0, `P'` is level 1, `P''` is level 2, etc). This was built up by the direct method of iterating over each sequence and storing the differences in a new list for the next level. Once a list contains only zeroes, we can move on. 

Next, we iterate across the levels, highest to lowest. Keeping a running tally, we add on each level's last value to get its next value. Once we hit level 0, that's our subtotal. 

## Part 2

Now we have to extrapolate backwards rather than forwards. This is almost identical to part 1, except now that we will be *subtracting* our running tally from the *first* value in each level to get the new value of the running tally. 

Again, add up the subtotals from each input sequence, and we're done. 


## Other approaches

The general form of this is to construct a polynomial from the sequence (either by library function or manually, perhaps with successive differences as a sub-task). 

Then the polynomial can be evaluated at any desired point. 

