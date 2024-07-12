# Method Syntax
Methods are similar to functions: we declare them with the fn keyword and a name, they
can have parameters and a return value, and they contain some code that’s run when the
method is called from somewhere else. Unlike functions, methods are defined within the 
context of a struct (or an enum or a trait object.

* To define the function within the context of Rectangle, we start an impl (implementation) 
block for Rectangle. Everything within this impl block will be associated with the 
Rectangle type. Then we move the area function within the impl curly brackets and 
change the first (and in this case, only) parameter to be self in the signature and 
everywhere within the body. In main, where we called the area function and passed 
rect1 as an argument, we can instead use method syntax to call the area method on our 
Rectangle instance. The method syntax goes after an instance: we add a dot followed by
the method name, parentheses, and any arguments.

![gitmethod1](https://github.com/Emmyy882/Rustlang/assets/110739304/d0162baf-244c-4bda-91fb-7b861314c34e)
