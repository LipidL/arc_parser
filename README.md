# arc_parser
arc_parser is a small program written in Rust that parses .arc files produced by the [LASP](http://www.lasphub.com/) program.

## Completed Functions
+ Prints the minimum energy structures in the .arc file

## Functions in Progress
+ Counts the structures in an .arc file
+ Identifies duplicate minimum structures
+ Exports the global minimum
+ Displays the energy difference of various structures

## Usage
### Basic Command
To use arc_parser, enter the following command:
```
./arc_parser -f myfile.arc
```

### Arguments
+ Use `-m` or `--minimum` to print the minimum energy structures in the .arc file.

    for example:
    ```
    ./arc_parser -f myfile.arc -m
    ```
+ use `-c` or `--count` to count the structures in the .arc file.

+ use `--consistency` to check if the structures in the .arc file have consistent atom composition

+ use `-e` or `--energy-list` to list all energy present in the .arc file

    *note that energy difference less than 0.001eV will be seen as the same.*

more arguments are still in progress