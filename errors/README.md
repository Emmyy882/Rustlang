# ERROR HANDLING
Errors are a fact of life in software, so Rust has a number of features 
for handling situations in which something goes wrong. In many cases, Rust
requires you to acknowledge the possibility of an error and take some 
action before your code will compile.

**Rust groups errors into two major categories: recoverable and unrecoverable
errors. 
*For a recoverable error, such as a file not found error, we most likely
just want to report the problem to the user and retry the operation.

*Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.
