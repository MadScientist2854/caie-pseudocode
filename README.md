# CAIE Pseudocode interpreter
an interpreter for the Cambridge pseudocode specification.

### This is a work in progress. Currently the whole of the IGCSE/O-Level specification is implemented and, as far as i can see, bug-free, but the more complicated concepts from the A-Level specification are currently either buggy or not done yet.

### Usage
Download one of these files depending on your operating system:
 - Windows 32-bit: 
 - Windows 64-bit: 
 - Linux 32-bit: 
 - Linux 64-bit: 
 - Mac: if you have a Mac and are willing to compile an executable for me, please contact me on Discord at MadScientist#6032

create a file in the same folder as the downloaded file called `source.txt`. write your code in `source.txt`, then run the downloaded file (as long as you didn't rename it, it should be called `PseudocodeInterpreter`). a window should pop up that either shows the output or the errors in your program.

## Manually Compiling
install cargo and the rust compiler, and run `cargo run <source code file>`

Note: the normal order of operations (PEMDAS/BODMAS) is used for arithmetic operators, though the specification doesn't actually say what order should be used. comparison operators (`>`, `<`, etc.) come next after arithmetic operators. the boolean operations `AND` and `OR` come after those. `=` and `<>` come last in the precedence. the boolean operation `NOT` comes before the arithmetic operators