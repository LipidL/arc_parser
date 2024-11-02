# arc_parser

**arc_parser** is a small program written in Rust that parses `.arc` files produced by the [LASP](http://www.lasphub.com/) program.

## Completed Functions

- Print the minimum energy structures in the `.arc` file
- Count the structures in an `.arc` file
- Identify duplicate minimum structures
- Export the global minimum structure or the structure at the speficied position
- List all energies of structures
- Rearrange atoms in a structure along the X, Y, or Z axis
- Scale the crystal along the X, Y, or Z axis
- Calculate interplanar spacing of a given surface (specified by 3 atoms)
- Check if the result of the [LASP](http://www.lasphub.com/) program is valid
- Extract unconverged [LASP](http://www.lasphub.com/) structures
- compare the structure of two files and find the substructure in file1 that is similar to the structure in file2. This function uses multithreading to accelerate the process.

## Functions in Progress

- Automatically analyze exposure of crystal surface
- Analyze symmetry of structure
- Find substructure that is similar to a reference structure in a given `.arc` file

## Usage

From version 0.2.0, the usage has changed. Users can now use subcommands to specify the type of task. For example:

```sh
./arc_parser parse -f myfile.arc
```

or

```sh
./arc_parser check -p .
```

The following subcommands are supported:

### parse

The `parse` subcommand provides basic analytics for the target `.arc` file.

#### Arguments

- Use `-f` or `--file` to specify the file you want to parse.
- Use `-m` or `--minimum` to print the minimum energy structures in the `.arc` file.
- Use `-c` or `--count` to count the structures in the `.arc` file.
- Use `-C` or `--consistency` to check if the structures in the `.arc` file have consistent atom composition and view the atom composition.
- Use `-l` or `--list` to list all energies present in the `.arc` file.
  - *Note that energy differences less than 0.001 eV will be considered the same.*
- Use `--extract` to extract the structure to extracted.arc. An argument is needed to specify the structure you want to extract. The first structure is numbered 1 and so on. If you use 0 as an argument, the program will automatically extract the minumum structure.
- Use `--coord` to specify the structure that you want to analyze for coordination number.
  - *Note that the first structure in the `.arc` file is number 0.*
  - If no number is specified, the structure with the minimum energy will be automatically analyzed.

### check

The `check` subcommand helps you verify whether the result of the [LASP](http://www.lasphub.com/) program is valid.

#### Arguments

- Use `-p` or `--path` to specify the path to the directory of the result.

### modify

The `modify` subcommand performs modifications to the structure and outputs the structure to another `.arc` file.

#### Arguments

- Use `-f` or `--file` to specify the file that you want to modify.
- Use `-n` or `--number` to specify the structure that you want to modify.
  - *Note that the first structure in the `.arc` file is number 0.*
  - If no number is specified, the structure with the minimum energy is automatically selected.
- Use `-r` or `--rearrange` to rearrange atoms of the structure.
  - *You should specify x, y, or z after this argument.*
- Use `-s` or `--scale` to scale the cell of the structure.
  - *You should specify one or three number(s) after this argument.*
  - If one number is specified, the whole cell will be scaled by that proportion.
  - If three numbers are specified, the a, b, and c values will be scaled by the three proportions respectively.
  - *Note that if you shrink the cell by specifying a proportion less than 1, it is possible that the cell can't hold all the atoms.*

### compare

#### Arguments

- Use `-f` or `--file1` to specify the first input file. 
  - this input file can have multiple structures. 
- Use `-F` or `--file2` to speficy the second input file.
  - this input file is regarded as the reference file. Only the first structure is used as the reference
- Use `-t` or `--threads` to specify the number of threads you want to create.
  - this program uses multithreading to accelerate the compare process
  - by default, the progeram parallel over different structures, i.e. each thread handles a structure. 

More subcommand arguments are still in progress.