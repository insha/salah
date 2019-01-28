How to contribute to Salah
==========================

Thank you for considering contributing to Salah!

Reporting issues
----------------

- Describe what you expected to happen.
- If possible, include a [minimal, complete, and verifiable example](https://stackoverflow.com/help/mcve) to help
  us identify the issue. This also helps check that the issue is not with your
  own code.
- Describe what actually happened. Include the full traceback if there was an
  exception.

Submitting patches
------------------

- Include tests if your patch is supposed to solve a bug, and explain
  clearly under which circumstances the bug happens. Make sure the test fails
  without your patch.
- Make sure all commits are verified.
- Make sure there are no trailing spaces in the any of the modified files.

First Time Setup
----------------

Best way to have a local Rust development environment set up is by using [rustup](https://www.rust-lang.org/tools/install).

There are two direct dependencies for this crate:
- [chrono](https://docs.rs/crate/chrono/0.4.6)
- [Spectral](https://docs.rs/spectral/0.6.0/spectral/) (needed only for running tests.)

These would be installed when you run `cargo build` (or `cargo test`).

Start coding
------------

- Create a branch to identify the issue you would like to work on (e.g.
  `2287-dry-test-suite`)
- Using your favorite editor, make your changes, [committing as you go](https://dont-be-afraid-to-commit.readthedocs.io/en/latest/git/commandlinegit.html#commit-your-changes).
- Make sure there are no trailing spaces in the any of the modified files.
- Include tests that cover any code changes you make. Make sure the test fails
  without your patch.
- Push your commits to GitHub and [create a pull request](https://help.github.com/articles/creating-a-pull-request/).
- Celebrate 🎉

Running the tests
-----------------

Run the basic test suite with:

    cargo test

If you would only like to run a single test you can do so with:

    cargo test <name of test>

Building the docs
-----------------

Docs for the crate can be locally built using:

    cargo doc --no-deps
