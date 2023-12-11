# Day 4

Let's play scratchies!

We have a scratchcard, which is two lists of numbers. The left list says which numbers are winners. The right is a list of candidates. 

## Part 1

If we have one or more winning candidates, our score for the scratchcard is `2^(x-1)` where `x` is the number of winnng candidates. If we have no winning candidates, we score zero. 

Our puzzle result is the sum of the scores across all the scratchcards. 

This was fairly simple to implement. Iterate over lines (cards), split cards up, parse numbers, compute the size of the intersection, tally up. 

## Part 2

Turns out we read the rules wrong! Instead, we get *more scratchcards* according to how many winning numbers we have. 

> If card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.

Each new copy acts like the original, e.g. if card 11 caused us to win an extra copy of 12, 13 and 14, and we had three extra copies of 11 from previous cards, now we have four extra copies each of 12, 13 and 14. 

Our puzzle result is the total number of scratchcards, including our original allocation. 

To implement this, I maintained a map of copy counts. When processing a card, I could look up how many copies of that card there were. Then when "winning more copies" of future cards, I could simply add however-many copies there were of this card.
