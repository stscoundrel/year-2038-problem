# Year 2038 problem

Demonstration of year 2038 problem in Rust.

The year 2038 problem refers to maximum unix date that can be stored in i32 value. 03:14:07 UTC on 19 January 2038, the integer will overflow. Result will be incorrect time calculation, around 20:45:52 on Friday, 13 December 1901.

## The test

- Creates maximum value for i32
- Uses wrapped add to increase it by 1 without causing overflow errors
- In very simplified way, parse current year from the timestamp. Ignores constraints like leap years, leap seconds etc.

## To run

`cargo run`
