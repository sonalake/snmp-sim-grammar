# snmp-sim-grammar

This is the home for the SNMP Simulator's SNMP Data File template grammar. The goal of this repository is to define [SNMP Data File Templates](https://docs.google.com/document/d/1hYdvWZjQyETBO7NTqr9L-Bv6WnQ5vSaO3Npf2StydMo/edit#heading=h.3p0q3uenu6fy) and produce a testable, canonical grammar for the SNMP Data File template language.

## Test Suite

This project includes a executable binary for exercising the grammar. Use
`cargo run` to build and run it. Using `--release` is encouraged as it is many
times faster. It supports several subcommands to point it at different files
or directories to examine. For example, if you check out the submodule
(explained below), you can run the tests with the following command:

    cargo run --bin coverage --release -- dir external/rust/src

This repository includes a submodule to the main rust repo to use as a
collection of Rust code to test against the grammar. The command `git
submodule update --init` may be used to fetch it. However, it is not necessary
and you may run the tool against any collection of Rust code at your disposal.

### Snapshot tests

Additionaly, a number of [snapshot tests](https://docs.rs/insta/*/insta/#how-it-operates)
are maintained in [testdata](testdata). These files are named according to the
convention of `<production>.<test-name>.input`, and the parse forest that they
produce is tracked in [src/bin/snapshots](src/bin/snapshots). The tests are run with:

    cargo run --bin snapshots --release

Once a snapshot is present for a given production, that production is,
to the best of the maintainers' ability, considered "complete"
in terms of its structure.

A test may also have the name `<production>.<variant>.<test-name>.input`,
in which case only that variant should be considered tested and marked
"complete" by that test case.

At all times we reserve the right to change parse forests that do not produce
unique parse trees, and to disambiguate tested parse forests to a subset.

## License

This SNMP Simulator Grammar is licensed under the [APACHE-2.0](https://www.apache.org/licenses/LICENSE-2.0) license.

## Contributing

Want to contribute? Great 🎉

There are many ways to give back to the project, whether it be writing new code, fixing bugs, or just reporting errors. All forms of contributions are encouraged!

For instructions on how to contribute, see our [Guide to contributing](https://github.com/sonalake/snmp-sim-rust/blob/main/CONTRIBUTING.md).