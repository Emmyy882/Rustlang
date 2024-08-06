# Capturing references
Closures can capture values from their environment in three ways, which 
directly map to the three ways a function can take a parameter: borrowing
immutably, borrowing mutably, and taking ownership.

** This program shows the defininition of a closure that captures an immutable 
reference to the vector named list because it only needs an immutable reference
to print the value:
