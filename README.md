## About

This is an implementation of the `calendar(1)` program as available on [many][openbsd calendar] [BSD][freebsd calendar] [variants][netbsd calendar].

## Status

This is mostly intended as a toy project for experimenting a bit with Rust.
The implementation is primarily inspired by the [OpenBSD `calendar(1)` implementation](https://man.openbsd.org/calendar) and implements most of its feature.
However, the input format is not intended to be fully compatible with OpenBSD.

## Installation

TODO.

## Usage

TODO.

## Test

Due to [limitations of the time crate][time crate threads] this test suite can only be executed in a single threaded environment:

    $ RUST_TEST_THREADS=1 cargo test

Unfortunately, this option [can't be enabled via the Cargo.toml][cargo defopts] configuration file.

## License

This program is free software: you can redistribute it and/or modify it
under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

[openbsd calendar]: https://man.openbsd.org/calendar
[freebsd calendar]: https://www.freebsd.org/cgi/man.cgi?query=calendar
[netbsd calendar]: https://man.netbsd.org/calendar.1
[time crate threads]: https://github.com/time-rs/time/issues/538
[cargo defopts]: https://github.com/rust-lang/cargo/issues/8430
