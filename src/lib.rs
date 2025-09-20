// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! `faine` stands for _FAultpoint INjection, Exhaustible/Exploring_ and is an
//! implementation of testing technique known as
//! [_fail points_](https://man.freebsd.org/cgi/man.cgi?query=fail),
//! [_fault injection_](https://en.wikipedia.org/wiki/Fault_injection),
//! or [_chaos engineering_](https://en.wikipedia.org/wiki/Chaos_engineering),
//! which allows testing otherwise hard or impossible to reproduce conditions
//! such as I/O errors.
//!
//! # How this works
//!
//! - You instrument the source code, adding (fail)points where normal code flow
//!   can be overridden externally, to, for instance, return a specific error instead
//!   of calling an I/O function (note that for surrounding code, triggering such
//!   failpoint would be the same as named I/O function returning an error).
//! - You trigger these failpoints in the tests, effectively simulating otherwise
//!   hard to reproduce failures, and check that your code behaves correctly under
//!   these conditions.
//!
//! On top of supporting that, `faine` implements automated execution path exploration,
//! running a tested code multiple times with different combinations of failpoints enabled
//! and disabled (NB: in much more effective way than trying all N² possible combinations).
//! This allows simpler tests (which do not know inner workings of the code, that is to
//! know which failpoints to trigger and which effects to expect), with much greater coverage
//! (as all possible code paths are tested).
//!
//! # Illustrative example
//!
//! Let's imagine you want to test a code which atomically replaces a file, which is canonically
//! done by opening a temporary file, writing to it, and then renaming it over an old file -
//! that's at least 3 filesystem operations each of which may fail independently. With `faine`,
//! what you do is:
//!
//! - You add a failpoint before/around each I/O operation (e.g. `std::fs` call).
//! - In the test, you check whether resulting filesystem state after calling you code, which
//!   should be a file with either old or new contents, not anything else (like missing or empty
//!   file, which would be a result of incorrect implementation).
//!
//! Now imagine that instead of using highlevel `fs` primitives, you bring a whole filesystem
//! implementation into your code. You still add a failpoint before/around each low level I/O
//! operation (disk block reads and writes in this case), but the test code does not change at
//! all! And it checks your code bahavior agains a possible failur in _each_ of many operations
//! which consitute a filesystem transaction.
//!
//! # Code example
//!
//! As in example above, let's test a function which is supposed to atomically replace a file.
//! For illustrative purposes, let's take an invalid implementation.
//!
//! ```
//! # use std::path::Path;
//! # use std::fs::File;
//! # use std::io::{self, Write};
//! fn replace_file(path: &Path, content: &str) -> io::Result<()> {
//!     let mut file = File::create(path)?;
//!     file.write_all(content.as_bytes())?;
//!     Ok(())
//! }
//! ```
//!
//! Add failpoints to the code:
//!
//! ```
//! # use std::path::Path;
//! # use std::fs::File;
//! # use std::io::{self, Write};
//! use faine::inject_return;
//! fn replace_file(path: &Path, content: &str) -> io::Result<()> {
//!     inject_return!("create new file", Err(io::Error::other("injected error")));
//!     let mut file = File::create(path)?;
//!     inject_return!("write new file", Err(io::Error::other("injected error")));
//!     file.write_all(content.as_bytes())?;
//!     Ok(())
//! }
//! ```
//!
//! Implement setup code and check, and you can test it:
//!
//! ```should_panic
//! # use std::path::Path;
//! # use std::fs::{File,read_to_string};
//! # use std::io::{self, Write};
//! # use faine::inject_return;
//! use faine::Runner;
//! # fn replace_file(path: &Path, content: &str) -> io::Result<()> {
//! #     inject_return!("create new file", Err(io::Error::other("injected error")));
//! #     let mut file = File::create(path)?;
//! #     inject_return!("write new file", Err(io::Error::other("injected error")));
//! #     file.write_all(content.as_bytes())?;
//! #     Ok(())
//! # }
//! #[test]
//! # fn dummy() {}
//! fn test_replace_file_is_atomic() {
//!     faine::Runner::default().run(|| {
//!         // prepare filesystem state for testing
//!         let tempdir = tempfile::tempdir().unwrap();
//!         let path = tempdir.path().join("myfile");
//!         File::create(&path).unwrap().write_all(b"old").unwrap();
//!         // run the tested code
//!         let res = replace_file(&path, "new");
//!         // check resulting filesystem state
//!         let contents = read_to_string(path).unwrap();
//!         assert!(
//!            res.is_ok() && contents == "new" ||
//!            res.is_err() && contents == "old"
//!         ); // fires!
//!     }).unwrap();
//! }
//! # test_replace_file_is_atomic();
//! ```
//!
//! <details>
//! <summary>For completeness, let's test the correct implementation</summary>
//!
//! ```
//! # use std::path::{Path, PathBuf};
//! # use std::fs::{File,read_to_string, rename};
//! # use std::io::{self, Write};
//! use faine::{Runner, inject_return};
//! # // XXX: hack to provide clean and correct test code, until
//! # // `path_add_extension` gets into all supported rust versions
//! # struct MyPath<'a> {
//! #     path: &'a Path,
//! # }
//! # impl MyPath<'_> {
//! #     fn with_added_extension(&self, extension: &str) -> PathBuf {
//! #         // incorrect, but that's irrelevant for the example
//! #         self.path.with_extension(extension)
//! #     }
//! # }
//! fn replace_file(path: &Path, content: &str) -> io::Result<()> {
//! #    let path = MyPath{path};
//!      let temp_path = path.with_added_extension("tmp");
//! #    let path = path.path;
//!      {
//!          inject_return!("create temp file", Err(io::Error::other("injected error")));
//!          let mut file = File::create(&temp_path)?;
//!          inject_return!("write temp file", Err(io::Error::other("injected error")));
//!          file.write_all(content.as_bytes())?;
//!      }
//!      inject_return!("replace file", Err(io::Error::other("injected error")));
//!      rename(&temp_path, path)?;
//!      Ok(())
//! }
//!
//! #[test]
//! # fn dummy() {}
//! fn test_replace_file_is_atomic() {
//!     Runner::default().run(|| {
//!         let tempdir = tempfile::tempdir().unwrap();
//!         let path = tempdir.path().join("myfile");
//!         File::create(&path).unwrap().write_all(b"old").unwrap();
//!         let res = replace_file(&path, "new");
//!         let contents = read_to_string(path).unwrap();
//!         assert!(
//!            res.is_ok() && contents == "new" ||
//!            res.is_err() && contents == "old"
//!         ); // now OK!
//!     }).unwrap();
//! }
//! # test_replace_file_is_atomic();
//! ```
//!
//! </details>
//!
//! # Specifying failpoints
//!
//! The examples above shows the most verbose way to specify failpoints,
//! however there are shorter forms:
//! - You may omit failpoint names if you don't care about them (in which
//!   case they are generated from the source path, line and column).
//! - There are shortcuts which just return `std::io::Error::other`, as
//!   testing I/O operations is probably the most common use case.
//!
//! ```
//! # use std::io;
//! # use faine::{inject_return, inject_return_io_error};
//! inject_return!("failpoint name", Err(io::Error::other("injected error")));
//! inject_return!(Err(io::Error::other("injected error")));  // name autogenerated
//! inject_return_io_error!("failpoint name");                // return io::Error
//! inject_return_io_error!();
//! # Ok::<(), io::Error>(())
//! ```
//!
//! There is a set of macros with the same variations which, instead of returning
//! early, wrap an expression and replace it with something else when failpoint
//! is triggered:
//!
//! ```no_run
//! # use std::io;
//! # use std::fs::File;
//! # use faine::{inject_override, inject_override_io_error};
//! let f = inject_override!(File::open("foo"), "failpoint name", Err(io::Error::other("injected error")));
//! let f = inject_override!(File::open("foo"), Err(io::Error::other("injected error")));
//! let f = inject_override_io_error!(File::open("foo"), "failpoint name");
//! let f = inject_override_io_error!(File::open("foo"));
//! # Ok::<(), io::Error>(())
//! ```
//!
//! These are also useful if the tested code has its own branching based on the
//! result of an operation:
//!
//! ```
//! # use std::path::Path;
//! # use std::fs::File;
//! # use std::io;
//! # use faine::{inject_override, inject_override_io_error};
//! fn open_with_fallback() -> io::Result<File> {
//!     if let Ok(file) = inject_override_io_error!(File::open("main.dat")) {
//!         Ok(file)
//!     } else {
//!         inject_override_io_error!(File::open("backup.dat"))
//!     }
//! }
//! ```
//!
//! # Executing the instrumented code
//!
//! In the test, just construct a default `Runner` and call its `run()` method
//! with the tested code:
//!
//! ```
//! # use faine::Runner;
//! # fn tested_code() {}
//! #[test]
//! # fn dummy() {}
//! fn test_foobar() {
//!     Runner::default().run(|| {
//!         tested_code();
//!     }).unwrap();
//! }
//! ```
//!
//! # Controlling behavior
//!
//! - You can disable/enable failpoints processing:
//!
//!   ```
//!   use faine::enable_failpoints;
//!
//!   enable_failpoints(false);
//!   // failpoints will be ignored here
//!   enable_failpoints(true);
//!   ```
//!
//! - `Runner` has some knobs to tune its behavior.
//!
//! # Other implementaions of the same concept
//!
//! Neither supports path exploration as far as I know.
//!
//! - [chaos-rs](https://crates.io/crates/chaos-rs)
//! - [fail](https://crates.io/crates/fail)
//! - [fail-parallel](https://crates.io/crates/fail-parallel)
//! - [failpoints](https://crates.io/crates/failpoints)
//! - [fault-injection](https://crates.io/crates/fault-injection)

mod collections;
mod common;
mod error;
mod functions;
mod macros;
mod options;
mod runner;
mod tree;

#[doc(hidden)]
pub mod __private;

pub use common::{Branch, Label};
pub use error::Error;
pub use functions::enable_failpoints;
pub use runner::Runner;
