* Improve error handling
    * Don't unwrap on parser error in lib?
    * Don't use unwrap everywhere in binary?
* Implement pre-processing with `cpp(1)`
* Support wildcards (`*`) in input format, e.g.:
    * Any date of a month
    * Any month of a year
    * Treat entries without a year as "any year", not "current year"
* Support relative "addressing", e.g.:
    * Last Saturday of the month
    * First Monday of the month
* Implement the OpenBSD's `-a` flag
    * Requires changing uid
    * Opportunity to experiment with low-level POSIX APIs in Rust
* Make `out_fmt` in `main.rs` configurable
* Consider tracking time and place in event description
    * For example: `23 Dec Meeting with Hannah - Coffee Place (13:00)`
    * Could mean: "Meet with Hannah on the 23th of December at the Coffee Place at 13:00 o'clock"
