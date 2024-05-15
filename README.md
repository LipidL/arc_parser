# arc_parser
arc_parser is a small program written in Rust that parses .arc files produced by the [LASP](http://www.lasphub.com/) program.

## Completed Functions
+ Print the minimum energy structures in the .arc file
+ Count the structures in an .arc file
+ Identify duplicate minimum structures
+ Export the global minimum
+ List all energy of strucures
+ Rearrange atoms in a structure with X, Y or Z axis
+ Scale the crystal by X, Y or Z axis
+ Calculate interplanar spacing of given surface(specified by 3 atoms)
+ Check if the result of [LASP](http://www.lasphub.com/) program is valid
+ Extract unconverged [LASP](http://www.lasphub.com/) strucutres

## Functions in Progress
+ Automatically analyze exposure of crystal surface
+ Analyze symmetry of structure




## Usage
### Basic Command
To use arc_parser, enter the following command:
```
./arc_parser -f myfile.arc
```

### Arguments

+ use `--check` to check if the SSW result is valid

    having more than 3 strucutres in Badstr.arc or having more than 3 unconverged structures are seen as invalid.

    in case of invalid result, unconverged.arc containing all unconverged structures will be written.

+ use `-f` or `--file` to specify the file you want to parse

+ use `-m` or `--minimum` to print the minimum energy structures in the .arc file

+ use `-c` or `--count` to count the structures in the .arc file

+ use `C` or `--consistency` to check if the structures in the .arc file have consistent atom compositionm, and view the atom composition

+ use `-l` or `--list` to list all energy present in the .arc file

    *note that energy difference less than 0.001eV will be seen as the same.*

+ use `--extract` to extract the structure with the minimum energy to minimum.arc

+ use `-r` or `--rearrange` to rearrange the minimum block by atom's coordinate, x, y or z

    for example:
    ./arc_parser -f myfile.arc --rearrange x

+ use `--scale` to scale the crystal of the minimum block by given size

    for example:
    ./arc_parser -f myfile.arc --scale 2 #this command expands the crystal by 2
    ./arc_parser -f myfile.arc -s 1 -s 2 -s 2 #this command expands y and z of the crystal by 2 while keeping x unchanged

more arguments are still in progress