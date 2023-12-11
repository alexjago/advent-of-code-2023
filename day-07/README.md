# Day 7

Today, we're ranking hands in Camel Cards, which is a bit like poker. 

A hand consists of five cards from `23456789TJQKA`. There are seven types of hand: 

1. Five of a kind (5 cards all of the same value)
2. Four of a kind (4 of the same value and 1 other)
3. Full House (3 of value X, 2 of value Y)
4. Three of a kind (3 of the same value, 2 of different values)
5. Two Pair (2 of value X, 2 of value Y, and 1 other)
6. Two of a kind (2 of value X and 3 others, all different)
7. High card (Ace down to 2)

When comparing the same type of hand, rather than using poker rules, we instead just consider the cards of the hand left to right. 

So `A222A` beats `2AAA2`in Camel Cards: they're both Full Houses, but the first one has a higher card in its leftmost position. 


## Part 1

I ended up refactoring this fairly dramatically in light of part 2 and again after completing part 2. I'll present the final form. 

First, though, I'll note the value of closely reading the problem description vs. the sample input. The latter contained five hands. There are seven possible hand types. In particular, I'd forgotten to implement a distinction between e.g. Full House and Three of a Kind. This meant that my code worked on the sample input but not the real input. 

I implemented type-of-hand detection with the Counter crate, and in particular its `most_common_ordered` method was very useful to get me the number of occurrences of the most common and second-most-common card. 

I ultimately ended up implementing a **score-based system:** 

* Each card in the hand `ABCDE` corresponds to two digits (base 10), with the hand type as a leading digit: `haabbccddee`. 
  * This means that e.g. `2AAA2` becomes `50012121200` 
	  * `5` because there are seven hand types with numbers `1` up to `7` by strength, and Full House is the third-strongest 
		* Then a `2` gets number `00` up to an `A` with number `12`.    
* Now we can order hands by score and things... just work out. 


The actual problem involves reporting the sum of the product of the hand-ranks and their corresponding bids, which was straightforward once the hands were ranked. 

Originally for part 1 I had defined a comparison method directly for hands rather than generating this numeric score, but I found the score method easier for part 2...


## Part 2

Plot twist! `J` actually stands for Joker. 

Now, the individual card ranks are `J23456789TQKA` such that a Joker ranks below even a 2. However, when determining *hand types* a Joker becomes whatever card helps the hand best. 

In Camel Cards (unlike poker) we don't have flushes, so a Joker should always become the same card. 

Note: a `JJJJJ` is still a five-of-a-kind, it's just the lowest ranked one (its score is `50000000000`). 

I implemented this by successively replacing `J` with *each other card in the hand,* calculating the score for that, and reporting the maximum.

At this point we have a score and we can proceed like in part 1. 
