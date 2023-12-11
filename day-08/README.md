# Day 8

Today's problem involved executing some finite state machines and reporting the number of execution steps. 

The input consisted of an initial line `{L, R}*` (a sequence of `L` and `R` characters), with subsequent lines of the form `AAA = (BBB, CCC)`. 

This denotes a state machine where node `AAA` transitions to node `BBB` on an `L`, and to `CCC` on an `R`.

The sequence of `L` and `R` is repeated as needed. 

## Part 1

Parsing was straightforward, just some string splits to extract node names and put it into a map. 

For Part 1 we needed to start at node `AAA`, run the state machine until reaching node `ZZZ` and report the number of steps (state-machine transitions). 

This was quite straightforward: 

* have a current node (initially `AAA`) 
* iterate over the `{L,R}` line 
* break if the current node is `ZZZ`
* update the current node based on input and map
* increment the step count

## Part 2

Now, we need to execute a number of machines in parallel: we start at `11A, 22A, ... xxA` and end at `11Z, 22Z, ... xxZ`. We're done when *every* machine is in its `xxZ` node simultaneously. (They loop...)

I first tried simulating this directly, and it seemed to be taking a while. 

Then I made the lucky guess that each machine was in a simple loop (`xxZ` goes back to `xxA`), which meant that I could run each machine independently and then take the Least Common Multiple of their individual cycle times. It worked!

Actually, the most frustrating thing about the whole process was dealing with the `num` crate rather than writing LCM from scratch. Rust-Analyzer was reporting an issue that `cargo check` wasn't (the `lcm` method being unavailable). Adding the crate at workspace *and* individual project level seemed to do the trick, so it's probably R-A's fault? 

 
