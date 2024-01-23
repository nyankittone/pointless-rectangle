# What is this?

this is a simple program that computes things relating to rectangles, namely the area, and whether
or not it can fit within another rectangle. This may or may not only exclusively exist because I 
was doing some examples from the Rust Book.

# how to use?

+ build with `cargo build`
+ when run, pass in parameters for a rectangle's length and height (e.g. `cargo run -- $length $height`)
+ The first thing the program does is display the rectangle's length and height, and then its area
+ The program is interactive; you're expected to type in a length and height for a different
rectangle (length first, height second)
+ if you type a valid line of input, it will make a lil rectangle from it, print out its area, and
then compare with the other rectangle to see if the new one can "fit" within the old one
+ if you type "quit" or "q" as input, the program gracefully closes

