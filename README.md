# palm.rs

A collection of libraries and command-line utilities for interacting with Palm
OS devices, and the data stored on them.


## Command-line tools

To install the basic set of palm.rs command-line tools:

```shell
% cargo install --git https://github.com/u1f408/palmrs.git palmrs
```

To install _everything_, including all sync conduits and debugging tools:

```shell
% cargo install --git https://github.com/u1f408/palmrs.git --features all palmrs
```


### HotSync

The main HotSync tool is `palmrs-sync` (**NOT YET IMPLEMENTED**).

The adaptation of the Palm OS database formats to and from formats that other
tools can understand is performed by _sync conduits_, which are external
command-line applications.

The default installation via Cargo only installs the sync conduits that are
deemed "essential" - to install all provided sync conduits (even if you don't
enable them in your sync configuration), enable the `sync-all` feature when
installing palm.rs (`cargo install --features sync-all palmrs`, or similar).

**Default sync conduits:**

* Tasks (To Do) app <-> [todo.txt](http://todotxt.org): `palmrs-conduit-todotxt`

**Other sync conduits:** (enabled with the `sync-all` feature)

* TODO


### Debugging / development tools

To use these tools, enable the `cli-debug` feature when installing palm.rs
(`cargo install --features cli-debug palmrs`, or similar).

You can run any of the below tools with the `--help` argument for a summary of
their usage, and available command-line options.

**Palm OS database tools:**

* Dump a Palm OS database file: `palmrs-db-dump`

**HotSync / sync conduit tools:**

* Call a sync conduit in debugging mode: `palmrs-sync-dbgconduit`


## Library crates

The library crates that make up palm.rs include:

* Palm OS database support (PRC/PDB files): [palmrs-database][]
* HotSync (and sync conduit) support: [palmrs-sync][]


[palmrs-database]: ./palmrs-database/README.md
[palmrs-sync]: ./palmrs-sync/README.md


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
