# Structs
Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold
 multiple related values. Like tuples, the pieces of a struct can be different types. 
Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the 
values mean. Adding these names means that structs are more flexible than tuples: you 
don’t have to rely on the order of the data to specify or access the values of an instance.

* To define a struct, we enter the keyword struct and name the entire struct. 
* A struct’s name should describe the significance of the pieces of data being grouped
 together. Then, inside curly brackets, we define the names and types of the pieces of
 data, which we call fields.
