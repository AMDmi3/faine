// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(dead_code)]

use std::fs::{File, rename};
use std::io::{self, Write};
use std::path::Path;

use faine::inject_return_io_error;

/// Atomically replaces file contents (incorrect implementation)
fn atomic_replace_file_a(path: &Path, content: &str) -> io::Result<()> {
    inject_return_io_error!("create file");
    let mut file = File::create(path)?;
    inject_return_io_error!("write file");
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Atomically replaces file contents
fn atomic_replace_file_b(path: &Path, content: &str) -> io::Result<()> {
    // Note: should be with_added_extension in fact, but that's not
    // supported on older rust versions yet
    let temp_path = path.with_extension("tmp");
    {
        inject_return_io_error!("create temp file");
        let mut file = File::create(&temp_path)?;
        inject_return_io_error!("write temp file");
        file.write_all(content.as_bytes())?;
    }
    inject_return_io_error!("replace file");
    rename(&temp_path, path)?;
    Ok(())
}

fn main() {
    eprintln!("please run me as a test");
}

#[cfg(test)]
mod tests {
    use super::*;

    use faine::Runner;
    use std::fs::read_to_string;

    fn test_atomic_replace_file(replace_file: impl Fn(&Path, &str) -> Result<(), io::Error>) {
        let report = Runner::default()
            .run(|_| {
                // prepare old file contents for testing
                let tempdir = tempfile::tempdir().unwrap();
                let path = tempdir.path().join("myfile");
                File::create(&path).unwrap().write_all(b"old").unwrap();

                // run tested code
                let res = replace_file(&path, "new");

                // check consistency:
                // - either the function succeeds and file contains new content
                // - or the function fails, in which case old content must be preserved
                let contents = read_to_string(path).unwrap();
                assert!(res.is_ok() && contents == "new" || res.is_err() && contents == "old");
            })
            .unwrap();
        eprintln!("{:#?}", report.traces);
    }

    #[test]
    #[should_panic]
    fn test_atomic_replace_file_a() {
        test_atomic_replace_file(atomic_replace_file_a);
    }

    #[test]
    fn test_atomic_replace_file_b() {
        test_atomic_replace_file(atomic_replace_file_b);
    }
}
