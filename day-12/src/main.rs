use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;

use regex::Regex;
use std::collections::HashMap;

#[derive(Parser)]
pub struct Opts {
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();

    let infile = read_to_string(opts.infile)?;

    println!("Part 1:\n{}", part_1(&infile));
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

fn part_1(infile: &str) -> usize {
    let mut total = 0;
    for line in infile.lines() {
        let (conds, counts) = parse_part_1(line);
        // println!("\n{line}");
        // println!("{conds:?}\t{counts:?}");
        let matches: Vec<usize> = conds_to_usizes(&conds)
            .into_iter()
            .filter(|b| match_report(*b, &counts))
            .collect();
        // for bbb in &matches {
        //     println!("\t{:064b}", bbb);
        // }
        // println!("\t{} matches", matches.len());
        total += matches.len();
    }
    total
}

/// Of course there's too many bits to fit in a u64 and you wouldn't want to do this exhaustively anyway
/// is this actually greedy?
fn part_2(infile: &str) -> usize {
    /*
    return infile
        .lines()
        .filter_map(|s| s.split_once(' '))
        // .map(|s| s.0.len())
        .map(|s| s.0.chars().filter(|&c| c == '?').count() * 5)
        .max()
        .unwrap();
    */
    // ^^ number of bits to try exhaustively. About 45. Times the number of lines (another 10 bits' worth of multiplier).

    /*
    We want to sort of peek arbitrarily forward until the next `Operational` spring
    if the next run of springs are Operational, we skip over them
    if the next run of springs are `Unknown`, call its length U...
    ... how many more Damaged springs can we match?
    need sum(D) + count(D) to be <= U (roughly)
    if previous match was Damaged (and fully used up) we will need the first Unknown to act as Operational

    If we have a mixed run of D and U that's a nightmare and a half innit

    If we work recursively we can maybe keep a lid on some of the madness?

    Every multi-run of just D and U splits the world into up to 2^|multirun| possibilities?

    For an arrangement to match, we need to have
        - all unknowns allocated
        - a matching number of D runs
        - all D runs to be a matching length
            - which implies that if a DU* chunk at the left edge can't be longer than its count, and a U*D chunk at the right edge similar

    We can split into D|U chunks?
        - if more chunks than counts, that's a fail
        - if as many chunks as counts, then all that's left to do is potentially slide U around where it sits on an edge
        - if fewer chunks than counts we may be able to split by allocating some U as O

    This isn't greedy or whatever hey, maybe not even dynamic?
    Maybe it is dynamic.

    Subproblem: if we have a D of length A, bounded by Ux and Uy, and a count of length B >= A, how many arrangements are there?
        - We're fitting 4 to ??###????, say
        - B=4, A=3, X = 2, Y = 4
        - well it can't slide further left or right than (B - A) = 1
        - So it has 2 positions
        - Now suppose A = 2: ??##????
        - Now it can slide 2 left or 2 right. ****????, ?****????, ??****??
        - OK, and now suppose A = 1 and Ux = 3 so as not to constrain: ???#????
            - could be ****???? or ?****??? or ??****?? or ???****?
        - This has consistently been 1 + B - A provided that min(Ux, Uy) >= B - A

    Now let's consider Ux = 0 (wlog, Uy = 0 also). B is *anchored*: ##????? becomes ****????. Only one arrangement. This is as greedy as it gets.

    Now let's consider e.g. B of length 3 in U (bounded on both sides by O) of length...
        - 3 -> 1
        - 4 -> 2
        - 5 -> 3
        - ... |arrangements| = U - B


    But all of these are very simple.

    Consider two single Ds in a run of eg 5 U (bounded on both sides by O)
    Now, the Ds mustn't touch, so we really have the following options
        - UDODU
        - DOODU or UDOOD
        - DOOOD (if there were 3 single Ds, this would be the only option really)


    Maybe we can work in from the outside? We know that if we have O*D on the left or DO* on the right, then we have to match the corresponding edge greedily.

    Suppose we had damages of count A, B, C to pack into U{> A+B+C+3} Then all arrangements are of the form:
            O*D{A}O+D{B}O+D{C}O*

    Subproblem: If we can take a single multichunk then we can divide and conquer a bit (multiplying the number of sub-arrangements at the end)
        If we eat multiple counts that's more recursion?


    Starts with O*D+U* and as long as |D|+|U| > leftmost we have a match (possibly with carryover)

    If a thing starts with O*U+D+ and |U| < |leftmost| then that D also locates the leftmost count (and v/v on the right)
        (Possibly splitting some U to the right)
        (# of possibilities somewhere between 0 and 1+|leftmost|-|D|)

    Starts with O*U*O and |U| < leftmost then that U is all Os (possibly a big win: # of possibilities === 1)

    O*U+O and |U| == leftmost gives us exactly two possibilities (there or not) plus a recursion for our trouble

    Starts with O*U+ ... and |U| > leftmost, that's where things get really tricky

    OK SO: it's been like 5 hours. Reddit says this is one for Dynamic. Ugh.

    Let's consider, again, the concept of multichunking, where a chunk is a {D,U}+ sequence (bounded by O or end of string)
    Our *decision*, knowing how many runs of damaged springs we have left to go, and something about our posiion in the chunk list, is how many to allocate to *this* chunk.
    Our *coordinates* are *probably* our index in the string generally and something based on our remaining runs
    Our *return* is the return from this chunk, multiplied by the highest return from our "future chunks"

    {(StrIndex, RunIndex): Arrangements}

    Nice.

    If we're at position X in the string, then we need to try and match U*D+U to the start of the string, where the run of the Us on the left is in 0..=Y
    where Y is {remaining length of string} - {sum (future |D|)} + {count(future |D|)} - |D|
    ... what? Well, all our remaining spring runs need a certain amount of space: the length of all those springs, plus a spacer in between
    ... this implies that our *next* spring-run has to be in a bounded position relative to the start of the string
    ... so we can try all of them, (immediately discarding any with an O in the way)
    ... and then that's our coordinates for memoisation, too

    */

    let longs: Vec<String> = infile.lines().map(convert_line).collect();

    return longs
        .iter()
        .filter_map(|r| r.split_once(' '))
        .map(|(s, n)| {
            (
                s,
                n.split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(s, n)| {
            // println!("");
            memo_pt2(s, &n, 0, 0, &mut HashMap::<(usize, usize), usize>::new())
        })
        .sum();
}

/// For each row, the condition records show every spring and whether it is operational (.) or damaged (#).
/// This is the part of the condition records that is itself damaged; for some springs, it is simply unknown (?) whether the spring is operational or damaged.
///
/// After the list of springs for a given row, the size of each contiguous group of damaged springs is listed in the order those groups appear in the row.
/// groups are always separated by at least one operational spring: #### would always be 4, never 2,2
///
/// Some rows have several possible arrangements
///
/// Anyway, we need
fn parse_part_1(row: &str) -> (Vec<Condition>, Vec<usize>) {
    let (springs, nums) = row.split_once(' ').unwrap();
    let springpat = Regex::new(r"(\#+)|(\.+)|(\?+)").unwrap();

    let conds = springpat
        .find_iter(springs)
        .map(|m| match &m.as_str()[..1] {
            "#" => Condition::Damaged(m.len()),
            "." => Condition::Operational(m.len()),
            "?" => Condition::Unknown(m.len()),
            _ => unimplemented!(),
        })
        .collect();

    let counts = nums
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    (conds, counts)
}

/// determine if a match
fn match_report(bits: usize, spec: &Vec<usize>) -> bool {
    // count bits

    if bits == 0 {
        return spec.is_empty();
    }
    let mut bits = bits >> bits.trailing_zeros();

    for v in spec {
        if bits == 0 {
            return false;
        }
        bits >>= bits.trailing_zeros();

        let d = bits.trailing_ones() as usize;

        if d == *v {
            bits >>= d;
            continue;
        } else {
            return false;
        }
    }

    bits.count_ones() == 0
}

/// Panics if input.len() > 64
/// Collates all the various possibilities for the conditions
/// as a bitstring ([0] matches LSB)
/// 1 = damaged, 0 = other
fn conds_to_usizes(input: &Vec<Condition>) -> Vec<usize> {
    use Condition::*;
    if input.len() > usize::BITS as usize {
        panic!("too large an input!");
    }
    let mut bitstring = 0_usize;

    let mut idx = 0;

    // Set all bits to 1 where it is known-damaged
    for c in input {
        match &c {
            Damaged(n) => {
                bitstring |= (0xffffffffffffffff >> (64 - n)) << idx;
                idx += n;
            }
            Operational(n) | Unknown(n) => {
                idx += n;
            }
        }
    }

    let mut out = vec![bitstring];
    idx = 0;

    for c in input {
        match &c {
            Unknown(n) => {
                for i in idx..(idx + n) {
                    for b in out.clone() {
                        out.push(b | 1 << i)
                    }
                }
                idx += n;
            }
            Damaged(n) | Operational(n) => idx += n,
        }
    }

    out
}

fn convert_line(line: &str) -> String {
    let (springs, nums) = line.split_once(' ').unwrap();
    format!(
        "{}?{}?{}?{}?{} {},{},{},{},{}",
        springs, springs, springs, springs, springs, nums, nums, nums, nums, nums
    )
}

/** Actually do part 2!
Huge credit to /u/pendejadas
https://www.reddit.com/r/adventofcode/comments/18ge41g/2023_day_12_solutions/kd1adh1/

- This is a dynamic programming solution (memoized)
- We parameterise our memoization over
  - the length of the remaining LHS string ('line_idx')
  - the number of runs of damaged springs yet to allocate ('counts_idx')
  - actually, we parameterise over the amount we've already used, same deal
- In a classic shenanigan, the tests pass but the actual doesn't

Ok So: the boundary conditions on this are easy to get wrong.

Suppose we have a string to process (LHS of the input) and some RHS.

We can place our first member of the RHS in the LHS at a limited number of spots.

We need to leave enough space for the remainder of the RHS, and gaps between them.

We need to not leave any known-damaged to the left of what we're placing.

And (to have a gap) we can't have a known-damaged spring immediately to our right.

Finally, we have a couple of base cases:
  - if RHS is empty then we either have one arrangement (no damaged springs) or zero arrangements
  - if LHS starts with a '.' (working spring) we can skip forward immediately.
**/
fn memo_pt2(
    line: &str,
    counts: &Vec<usize>,
    line_idx: usize,
    counts_idx: usize,
    store: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(r) = store.get(&(line_idx, counts_idx)) {
        return *r;
    };
    let l = &line[line_idx..];
    let c = &counts[counts_idx..];

    if l.starts_with('.') {
        // tail call recursion pls
        return memo_pt2(line, counts, line_idx + 1, counts_idx, store);
    }
    if c.is_empty() {
        if l.contains('#') {
            return 0;
            // illegal
        }
        return 1; // 0!, and all that
    }

    let total_damage = c.iter().sum::<usize>();

    if l.len() < total_damage + c.len() - 1 {
        return 0; // illegal
    }

    // We can have up to this much length before we have to place our next member of c (including the space that member needs)
    let spare_length: usize = l.len() - (c.iter().sum::<usize>() + c.len().saturating_sub(1));

    let mut total = 0;
    for p in 0..=spare_length {
        let left = &l[..p];
        let pane = &l[p..p + c[0]];
        let right = &l[p + c[0]..];
        if left.contains('#') {
            // overshot
            break;
        }
        if right.starts_with('#') {
            continue;
            // undershot
        }
        if !pane.contains('.') {
            if right.is_empty() {
                total += 1
            } else {
                total += memo_pt2(line, counts, line_idx + p + c[0] + 1, counts_idx + 1, store);
            }
        }
        // println!("{left}\t{pane}\t{}\t{c:?}\t{total}", &l[p + c[0]..]);
    }
    store.insert((line_idx, counts_idx), total);
    total
}

#[derive(Debug, PartialEq, Eq)]
/// usize is length of contiguous springs with that condition
enum Condition {
    Operational(usize),
    Damaged(usize),
    Unknown(usize),
}

#[cfg(test)]
mod test {
    use super::*;

    const _ALL_KNOWN: &str = r"#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";

    const EXAMPLE_1: &str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_parse_1() {
        use Condition::*;
        assert_eq!(
            parse_part_1("???.### 1,1,3"),
            (vec![Unknown(3), Operational(1), Damaged(3)], vec![1, 1, 3]),
        );
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 21);
    }

    #[test]
    fn part_2_5x() {
        assert_eq!(&convert_line(".# 1"), ".#?.#?.#?.#?.# 1,1,1,1,1");
    }

    #[test]
    fn memo_equiv() {
        assert_eq!(
            memo_pt2("?###????????", &vec![3, 2, 1], 0, 0, &mut HashMap::new()),
            10
        )
    }

    #[test]
    /// Base case needs to consider what happens when the RHS is anchored
    /// All the analysis was good for something?
    fn memo_right_anchor() {
        assert_eq!(
            memo_pt2("....???##?", &vec![3], 0, 0, &mut HashMap::new()),
            2
        );
    }

    #[test]
    fn test_pt2_print() {
        assert_eq!(part_2(".#.#. 1,1"), 1)
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 525152);
    }
}
//

// 2249347309805 is too high for part 2
