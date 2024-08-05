# Capturing the Environment with Closures

we can use closures to capture values from the environment they’re defined 
in for later use. Here’s the scenario: Every so often, our t-shirt company
gives away an exclusive, limited-edition shirt to someone on our mailing
list as a promotion. People on the mailing list can optionally add their
favorite color to their profile. If the person chosen for a free shirt 
has their favorite color set, they get that color shirt. If the person 
hasn’t specified a favorite color, they get whatever color the company
currently has the most of.

There are many ways to implement this. For this example, we’re going to
use an enum called **ShirtColor** that has the variants **Red** and **Blue** 
(limiting the number of colors available for simplicity). We represent the
company’s inventory with an **Inventory** struct that has a field named **shirts**
that contains a **Vec<ShirtColor>** representing the shirt colors currently 
in stock. The method **giveaway** defined on **Inventory** gets the optional 
shirt color preference of the free shirt winner, and returns the shirt 
color the person will get
