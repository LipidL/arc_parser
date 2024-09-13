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
+ Find substructure that is similar to a reference structure in a given .arc file




## Usage
From 0.2.0 version, usage have changed.
Users can now use subcommands to specify the type of the task.
For example, 
```
./arc_parser parse -f myfile.arc
```
or
```
./arc_parser check -p .
```
The following are subcommands supported
### parse

`parse` subcommand provides basic analystics to target arc file.

#### arguments

+ use `-f` or `--file` to specify the file you want to parse

+ use `-m` or `--minimum` to print the minimum energy structures in the .arc file

+ use `-c` or `--count` to count the structures in the .arc file

+ use `C` or `--consistency` to check if the structures in the .arc file have consistent atom compositionm, and view the atom composition

+ use `-l` or `--list` to list all energy present in the .arc file

    *note that energy difference less than 0.001eV will be seen as the same.*

+ use `--extract` to extract the structure with the minimum energy to minimum.arc

+ use `--coord` to specify the structure that you want to analyze coordination number. 

    *Note that the first structure in .arc file is number 0.*

If no number specified, the structure with the minimum energy will be automatically analyzed.

### check

`check` subcommand helps you check whether the result of [LASP](http://www.lasphub.com/) is valid.

#### arguments

+ use `-p` or `--path` to specify the path to the directory of the result.

### modify

`modify` subcommands do some modifications to the structure, and output the structure to another .arc file

#### arguments

+ use `-f` or `--file` to specify the file that you want to modify

+ use `-n` or `--number` to specify the structure that you want to modify

    *Note that the first structure in .arc file is number 0.*

If no number specified, the structure with the minimum energy is automatically selected.

+ use `-r` or `--rearrange` to rearrange atoms of the structure.
    
    *You should specify x, y or z after this argument*

+ use `-s` or `--scale` to scale the cell of the structure.

    *You should specify one or three number(s) after this argument*
    
if one number specified, the whole cell will be scaled by that proportion;

if three numbers specified, the a, b and c value will be scaled by the three proportions respectively

    *Note that if you shrink the cell by specifying a proportion less than 1, it is possible that the cell can't hold all the atoms.*

more subcommands arguments are still in progress