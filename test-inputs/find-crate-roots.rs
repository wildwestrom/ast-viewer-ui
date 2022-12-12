#!/usr/bin/env run-cargo-script
//Credit: https://gist.github.com/qryxip/f62fd9aa5c0eed3b5ff115456d469cf0
//! ```cargo
//! [dependencies]
//! term = "0.4.6"
//! ```

extern crate term;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use term::{Attr, color};

fn main() {
    use std::io::Write;

    macro_rules! write_error_colored {
        ($attr:expr, $color: expr, $format: tt) => {
            {
                let mut term = term::stderr().unwrap();
                term.fg($color).unwrap();
                term.attr($attr).unwrap();
                write!(term, $format).unwrap();
                term.reset().unwrap();
            }
        }
    }

    match crate_roots() {
        Ok(ref roots) if !roots.is_empty() => {
            for (i, root) in roots.iter().enumerate() {
                println!("{}: {:?}", i, root);
            }
        }
        Ok(_) => {
            write_error_colored!(Attr::Bold, color::RED, "error: ");
            writeln!(io::stderr(),
                     "could not find `Cargo.toml` in `{}` or any parent directory",
                     env::current_dir()
                         .unwrap_or_default()
                         .to_str()
                         .unwrap_or_default())
                .unwrap();
            process::exit(101);
        }
        Err(e) => {
            write_error_colored!(Attr::Bold, color::RED, "IO error: ");
            writeln!(io::stderr(), "{}", e).unwrap();
            process::exit(e.raw_os_error().unwrap_or(101));
        }
    }
}


/// If a directory named "Cargo.toml" is found, returns `Err`.
fn crate_roots() -> io::Result<Vec<PathBuf>> {
    fn find_cargo_toml<P: AsRef<Path>>(dir: P) -> io::Result<bool> {
        for entry in fs::read_dir(dir)?.filter_map(|entry| entry.ok()) {
            let ref path = entry.path();
            let ref name = path.file_name();
            if name.is_some() && name.unwrap() == &OsString::from("Cargo.toml") {
                return if path.is_dir() {
                    let message = "aborted because a directory named `Cargo.toml` was found";
                    Err(io::Error::new(io::ErrorKind::Other, message))
                } else {
                    Ok(true)
                };
            }
        }
        Ok(false)
    }

    let mut roots = Vec::new();
    let mut dir = env::current_dir()?;
    loop {
        if find_cargo_toml(&dir)? {
            roots.push(dir.clone());
        }
        if !dir.pop() {
            return Ok(roots);
        }
    }
}
