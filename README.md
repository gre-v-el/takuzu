# Takuzu
## A Rust implementation of Takuzu - a binary version of sudoku

[todo: screenshot]

Takuzu is played on a square board with NxN cells, where N must be even. Cells can be either true, false, or not yet assigned 

The puzzle starts with some cells filled. The player's aim is to fill the rest of the board without breaking any rules.

## The Rules
* At most two same consecutive cells (no linear triplets) 
* Equal number of trues and falses in a row/column 
* No two equal rows
* No two equal columns