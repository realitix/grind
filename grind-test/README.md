# Grind Test

This module allows to test and validate Grind behavior.
This module is in Python and uses PyTest. Why ?

Because we are launching tests from an external point of view, by loading the shared libraries.
If we would be using the rust integration tests, it would be from an internal point of view.

We could have write theses tests in rust but Python, as a scripting langage, can't be win on
its own field.

So we use:

- Python with pytest
- Binding to shared libraries with cffi

Theses tests can be run against the mesa OpenGl driver and against Grind, results have to
be the same.

## TDD

Theses tests will serve as a Specification Conformance Testing.
We write the test, then we write the code!
