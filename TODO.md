* Improve error handling
    * Don't use unwrap everywhere in binary?
* Implement the OpenBSD's `-a` flag
    * Requires changing uid
    * Opportunity to experiment with low-level POSIX APIs in Rust
* Make `out_fmt` in `main.rs` configurable
* Consider tracking time and place in event description
    * For example: `23 Dec Meeting with Hannah - Coffee Place (13:00)`
    * Could mean: "Meet with Hannah on the 23th of December at the Coffee Place at 13:00 o'clock"
* Write a simple GUI using the library for visualizing events
    * Or an export to PDF / HTML
    * See also: https://dianne.skoll.ca/projects/remind/
