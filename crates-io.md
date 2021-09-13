# palm.rs

A collection of libraries and command-line utilities for interacting with Palm
OS devices, and the data stored on them.

The `palmrs` crate itself contains the palm.rs command-line utilities, as well
as re-exporting the palm.rs subcrates. It's extremely likely that you don't
want to add a dependency on the `palmrs` crate directly - instead, your crate
should add dependencies on the individual palm.rs crates that it actually
requires to function.

For more information, including the list of available palm.rs subcrates, and
the list of command-line utilities included in this crate, have a look at
[the palm.rs repository on GitHub][repo].

[repo]: https://github.com/u1f408/palmrs

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
