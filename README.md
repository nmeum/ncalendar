## About

This is an implementation of the `calendar(1)` program as available on [many][openbsd calendar] [BSD][freebsd calendar] [variants][netbsd calendar].

## Status

This is mostly intended as a toy project for experimenting a bit with Rust.
The implementation is primarily inspired by the [OpenBSD `calendar(1)` implementation](https://man.openbsd.org/calendar) and implements most of its feature.
However, the input format is not intended to be fully compatible with OpenBSD.

## Features

* Saner and more strict input format compared to the BSD version
* Re-usable parser for the input format is provided as a Rust library
    * Allows building additional tools on top
    * For example, exporters for other formats
* Lots of great non-features:
    * No locale support
    * No support for Julian or Cyrillic calendars
    * No built-in support for [national holidays][openbsd ostern.c]

## Installation

This software can be installed using the following commands:

    $ git clone git://git.8pit.net/ncalendar.git
    $ cd ncalendar
    $ cargo install --path .

This will drop the `ncalendar` binary into `~/.cargo/bin`.
Make sure that directory is in your `$PATH`.

## Usage

The `ncalendar(1)` program reads calendar entries from the file `~/.ncalendar/calendar` by default.
The input format for this file is "documented" through [parser combinators][parser combinators wk] in `src/lib/format.rs`.
When invoked, all calendar entries which match a certain time span are written to standard output.
By default, entries for the current and the next day are printed.
The time span can be configured, via the `-B` (backward), `-A` (forward) and `-t` (set different current date) command-line options an.
For example:

    $ ncalendar -B 3 -A 11 -t 20122022

Will print all calendars in the inclusive range between the 17th December of 2022 and the 31th December.
The program is best invoked from a daily user-level cronjob.

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
[parser combinators wk]: https://en.wikipedia.org/wiki/Parser_combinator
[openbsd ostern.c]: https://github.com/openbsd/src/blob/47f32dc2b6cade03c63e7f98f4f715cb45238c6e/usr.bin/calendar/ostern.c
