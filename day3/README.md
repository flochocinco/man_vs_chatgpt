# Date
2023, December 8th

# Version
3.5 from webpage

# Status
compile, run but the result is not the expected one

# Prompt
Prompt 1
``` 
ok, let's try another problem:
write a rust code matching this constraint:
"The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number;"
```

Prompt 2
```
previous code doesn't work because it computes 141 instead of 4361. I think you are reading characters instead of string (i.e. figure instead of number)
```

Prompt 3
```
still wrong because you reading char instead of string!
error is around here:
sum += engine_schematic[i][j].to_digit(10).unwrap() as usize;

here should read the current number, meaning all figures until you reached a symbol or "."
```
