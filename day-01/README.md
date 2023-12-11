# Day 1

## Part 1

We have a list of strings like so:

>     1abc2
>     pqr3stu8vwx
>     a1b2c3d4e5f
>     treb7uchet

The *first* and *last* digit on each line form a two-digit number. Our puzzle result is the sum, across all lines, of this two-digit number.

I solved this by throwing a regex at the problem to match individual digits, then extracting the first and last digit (which might be the same).

Put the digits together, parse the integer, sum and done. 

## Part 2

Here's some more sample input:

>     two1nine
>     eightwothree
>     abcone2threexyz
>     xtwone3four
>     4nineeightseven2
>     zoneight234
>     7pqrstsixteen


Surprise! Some of the digits are spelled out with letters. We still need the sum of the two-digit numbers. But on e.g. the top line, under Part 1 rules its number would've been `11` whereas now it's `29`.

Again, we can throw a regex at the problem to match the first digit in the line and then parse it...

    (\d)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)

Hang on, what's this in the input?

>     nineight

Digits can overlap. **Digits can overlap!** 

The `Regex` crate doesn't report overlapping matches. Apparently that's the spec or something. 

However, we don't actually need overlapping matches in the general case. We only need the *very last match*. In what will become a recurring principle of "if it works, it's not stupid"...

    (\d)|(enin)|(thgie)|(neves)|(xis)|(evif)|(ruof)|(eerht)|(owt)|(eno)

... we can take this regex of reversed digit names, match it to the *reversed* input line, and then reverse the match to parse the digit. 

Put our two-digit numbers together, sum and done. 
