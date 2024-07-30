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

* Using the **assert!** macro helps us check that our code is functioning in
the way we intend.
