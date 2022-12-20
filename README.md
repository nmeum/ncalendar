## About

This is an implementation of the `calendar(1)` program as available on many BSD variants.

## Features

* Saner and more strict input format
* Re-usable parser for the input format
    * Allows building additional tools on top
    * For example, exports to other formats

## Test

Due to [limitations of the time crate](https://github.com/time-rs/time/issues/538) this test suite can only be executed in a single threaded environment:

    $ cargo test --test-threads 1

Unfortunately, this option [can't be enabled via the Cargo.toml](https://github.com/rust-lang/cargo/issues/8430) file.

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


