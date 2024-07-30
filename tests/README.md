# WRITING TESTS
Tests are Rust functions that verify that the non-test code is functioning
in the expected manner. The bodies of test functions typically perform
these three actions:

* Set up any needed data or state.
* Run the code you want to test.
* Assert the results are what you expect.

To change a function to a test function, add **#[test]** on the line before fn.
When you run your code with **Cargo test** command, Rust builds a test
runner binary that runs the annotated functions and reports whether each
test function passes or fails.

### assert! macro
The **assert!** macro, provided by the standard library, is useful when you
want to ensure that some condition in a test evaluates to **true**. We give the 
**assert!** macro an argument that evaluates to a Boolean. If the value is true,
nothing happens and the test passes. If the value is **false**, the **assert!**
macro calls **panic!** to cause the test to fail.

**NB:** Using the **assert!** macro helps us check that our code is
functioning in the way we intend.
