# Day 3

This puzzle is about gear ratios. Apparently. 

Our puzzle input is an ASCII-art diagram of "part numbers" (digits) and "symbols" (anything other than a digit or a `.`).

## Part 1

We need to report the sum of all part numbers which are 8-adjacent to a symbol.

This took surprisingly long! 

I parsed with regex, matching `([0123456789]+|[^0123456789.])` -- if a match then parsed as an integer, it was a number, if it didn't, it was a symbol. 

I recorded coordinates in either case, with numbers having both a start and end position. 

It took me frustratingly long to realise that the `end` position `Regex` reports is exclusive (so that you can do `start..end` and have things work as expected in indexing).

Then for each number, I iterated over all adjacent positions and checked if they contained a symbol. If so it got added to the total.

    xxxxx
    x123x
    xxxxx

Due to a misinterpretation of the discussion (and before fixing the regex `end` issue) I thought that part numbers needed to be deduplicated. They do not. This cost me a considerable number of submission attempts and my code contains comments relating to various combinations of `end` and deduplication as a result!

## Part 2

Now the only symbol we care about is the gear: `*`. Specifically, it's a gear when adjacent to exactly two numbers and its ratio is the product of those numbers. 

Our puzzle output is the sum of those products. 

This inverts the iteration nesting from Part 1: we had numbers and we were looking for symbols around them, now we have symbols and are looking for numbers around them. 

