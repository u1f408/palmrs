# palm.rs

A collection of libraries and command-line utilities for interacting with Palm
OS devices, and the data stored on them.

## Command-line tools

To install the command-line tools:

```shell
% cargo install --git https://github.com/u1f408/palmrs.git palmrs
```

### Debugging / development tools

To use these tools, enable the `cli-debug` feature when installing palm.rs
(`cargo install --features cli-debug palmrs`, or similar).

* Dump a Palm OS database file: `palmrs-db-dump`

## Library crates

The library crates that make up palm.rs include:

* Palm OS database support (PRC/PDB files): [palmrs-database][]


[palmrs-database]: ./palmrs-database/README.md


<br>

#### License

<sup>
Licensed under either of
<a href="http://www.apache.org/licenses/LICENSE-2.0">Apache License, Version
2.0</a> or <a href="http://opensource.org/licenses/MIT">MIT license</a>, at
your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
