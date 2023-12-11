# Day 5

Day 5 was probably the hardest day out of the first 10, though perhaps its Part 2 could've been brute-forced with simpler code. 

We begin with a list of seeds (numbers). 

Each *seed* needs some sort of *soil* (also numbered), but these aren't necessarily the same numbers. Our puzzle input allows us to map seed numbers to soil numbers: we're given a list of three numbers: 

* soil number
* seed number 
* the length of the range that this particular mapping is good for

It took me an embarassingly long time to realise that it's destination, then source, rather than the other way around. Easily fixed though. 

There's further mappings from soil to fertilizer, to water, to light, to temperature, to humidity, to location. 

If a transform is not defined then the number is carried through. 

## Part 1

We need to trace each seed through its transformations and report the *smallest* location number from any of the input seeds. 

I did this by writing a lookup method which effectively calculates `output = destination + input - source`. 

Performing the lookups in succession for each seed and then taking the minimum gets us there. 


## Part 2

Surprise! The list of seeds is actually a list of *ranges* of seed numbers!

I decided (based on the real input using very large seed numbers) that brute forcing this would be bad and that I should instead do the range version of the transformations. 

This presents some problems because applying a transform to an input range can result in up to *three* output ranges: 

* the part of the input range which is lower than the transform range
* the intersection, transformed
* the part of the input range which is higher than the transform range


As an example, consider the seed range `0..50` and the seed-to-soil transform `(23, 10, 7)`, meaning that seed range `10..17` becomes soil range `23..30`. Our output is as follows: 

* `0..10` stays the same
* `10..17` becomes `23..30`
* `17..50` stays the same

(Yes, seeds `10..17` and `23..30` both map to soil `23..30`.)

Performing the range partition just once has several pitfalls - I have six separate test cases. 

To complicate things further there are multiple possible transforms from seed to soil, etc. We must ensure that *every* possible transform is applied, *at most once* to every part of the input. In other words, once an input range has been transformed, we can't transform it further in this stage!

This is achieved in `multi_range` by means of maintaining `out` and `todo` lists - untransformed parts of existing ranges go back into `todo` for the next transform to have a go at. 

We end up with a list of ranges, and again we report the minimum value (i.e. the minimum start-of-range) from that list. (It was not otherwise necessary to coalesce ranges.)
