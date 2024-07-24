# Hash Map
The HashMap<k, v> stores a mapping of keys of type k to vallues of type v using a 
hashing function, which determines how it places these keys and values into memory.

* Hash maps are useful when you want to look up data not by using an index, as you can
with vectors, but by using a key that can be of any type. For example, in a game, you 
could keep track of each team's score in a hash map in which each key is a team's name
and the values are each team's score. Given a team name, you can retrieve its score.
