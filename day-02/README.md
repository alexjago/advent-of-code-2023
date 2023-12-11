# Day 2

We're playing a game of sorts. 

In each game, an Elf will randomly draw several cubes (either red, green, or blue) from a bag, show them to us, and then replace them, then repeat some number of times.

(So the number of cubes of each type is constant within a game, but not necessarily between a game.)


I took this day as an opportunity to use `nom` for input partsing and `strum` for Enum deserialisation. 

I represented an R/G/B quantity triple as a `Handful` and a colour as an enum, with impls and such. 

Parsing is beautiful:


    fn number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(input)
    }

    fn colour(input: &str) -> IResult<&str, Color> {
        map_res(alt((tag("red"), tag("green"), tag("blue"))), |s: &str| {
            Color::from_str(s)
        })(input)
    } 
    fn parse_line(input: &str) -> (u32, Vec<Handful>) {
        let game_tag = tag("Game ");
        let game_tot = preceded(game_tag, number);
        let dice = separated_pair(number, multispace1, colour);
        let handful = map(separated_list1(tag(", "), dice), Handful::from);
        let set_list = separated_list1(tag("; "), handful);
        let mut full_line = separated_pair(game_tot, tag(": "), set_list);
        full_line(input).finish().unwrap().1
    }

## Part 1

Suppose that we had exactly 12 red cubes, 13 green cubes, and 14 blue cubes. Of the number of games we've just seen, which would've been impossible?

To evaluate this, we can iterate through each Handful in each game, and do an element-wise comparison. If in any Handful there's too many cubes of a certain colour, the game is invalid. 

It took me annoyingly long to realise that the comparison had to be element-wise. 


## Part 2

Now instead of identifying impossible games given a constant number of cubes, we'd like to know the *fewest number of cubes of each colour* which would allow for each game. 

We do this by taking an element-wise `max` across the handfuls within each game. This was a nice opportunity to break out the `reduce` iterator method. 

We report the sum of the *power* of each game, where the power is `R * G * B` and R, G, B are our "minimum cubes needed for the game".

