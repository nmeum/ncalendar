use std::convert;
use std::fs::File;
use std::io::Read;
use std::path;
use std::process::{Command, Stdio};
use std::str;

use crate::Error;

pub fn has_cpp() -> bool {
    // XXX: This is a bit hacky as we don't know if the spawn failed
    // because cpp(1) doesn't exist or because of some other reason.
    //
    // Unfournuately, std::process doesn't have a function to iterate
    // over binaries in $PATH (e.g. analog to Go's LookPath) and we
    // can't check the spawn error for ENOENT either it seems.
    return Command::new("cpp")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .is_ok();
}

pub fn preprocess<P: convert::AsRef<path::Path>>(fp: P) -> Result<String, Error> {
    if has_cpp() {
        let f = File::open(fp)?;
        let child = Command::new("cpp")
            .arg("-traditional")
            .arg("-undef")
            .arg("-U__GNUC__")
            .arg("-w")
            .arg("-P")
            .stdin(f)
            .stdout(Stdio::piped())
            .spawn()?;

        let out = child.wait_with_output()?;
        Ok(str::from_utf8(&out.stdout).map(|s| s.to_string())?)
    } else {
        let mut f = File::open(fp)?;
        let mut buf = String::new();

        f.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
