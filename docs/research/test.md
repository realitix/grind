# Functional tests

There are two ways to do functional tests:

- Rust tests
- Python tests

Let's compare pro and con for each of them.

## Rust tests

- Same language as the code = this is the **best point**
- Managed directly by Cargo and integrated in the Rust ecosystem
- If we use `extern crate`, we don't need to rebin to EGL/OpenGL library -> Faster
- **BUT** in the same time, we don't test the library from an external point of view

## Python tests

- Another language (not really a problem)
- Another ecosystem to manage tests (that is **the problem**)
- Mandatory to test from an external point of view (**good** point)
- Faster tests with **Python**


## Conclusion

So here the dilemma.
The real question behind this is:

**Do we need to run tests by linking in the same manner as others libraries ?**

- Yes => Go for Python (faster, better testing environnment)
- No  => Go for Rust (integrated environment)

