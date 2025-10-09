# faine

`faine` stands for _FAultpoint INjection, Exhaustible/Exploring_ and is an
implementation of testing technique known as
[_fail points_](https://man.freebsd.org/cgi/man.cgi?query=fail),
[_fault injection_](https://en.wikipedia.org/wiki/Fault_injection),
or [_chaos engineering_](https://en.wikipedia.org/wiki/Chaos_engineering),
which allows testing otherwise hard or impossible to reproduce conditions
such as I/O errors.

## How this works

- You instrument the source code, adding (fail)points where normal code flow
  can be overridden externally, to, for instance, return a specific error instead
  of calling an I/O function (note that for surrounding code, triggering such
  failpoint would be the same as named I/O function returning an error).
- You trigger these failpoints in the tests, effectively simulating otherwise
  hard to reproduce failures, and check that your code behaves correctly under
  these conditions.

On top of supporting that, `faine` implements automated execution path exploration,
running a tested code multiple times with different combinations of failpoints enabled
and disabled (NB: in much more effective way than trying all N² possible combinations).
This allows simpler tests (which do not know inner workings of the code, that is to
know which failpoints to trigger and which effects to expect), with much higher coverage
(as all possible code paths are tested).

## Example

Let's test a code which is supposed to atomically replace a file with given content.

Instrument the code by adding failpoint macros before (or around) each operation
you want to simulate failures of:

```rust
use faine::inject_return_io_error;

fn atomic_replace_file(path: &Path, content: &str) -> io::Result<()> {
    inject_return_io_error!("create file"); // <- added failpoint
    let mut file = File::create(path)?;
    inject_return_io_error!("write file");  // <- added failpoint
    file.write_all(content.as_bytes())?;
    Ok(())
}
```

Now write a test, utilizing [`faine::Runner`](crate::Runner):

```rust
use faine::Runner;

#[test]
fn test_replace_file_is_atomic() {
    Runner::default().run(|_| {
        // prepare filesystem state for testing
        let tempdir = tempfile::tempdir().unwrap();
        let path = tempdir.path().join("myfile");
        File::create(&path).unwrap().write_all(b"old").unwrap();
        // run the tested code
        let res = atomic_replace_file(&path, "new");
        // check resulting filesystem state
        let contents = read_to_string(path).unwrap();
        assert!(
           res.is_ok() && contents == "new" ||
           res.is_err() && contents == "old"
        ); // fires!
    }).unwrap();
}
```

See [examples/atomic_replace_file.rs](examples/atomic_replace_file.rs) for complete code for this example.

## Documentation

See https://docs.rs/faine/latest/faine/ for complete documentation.

## Other implementations of the same concept

Neither supports path exploration as far as I know.

- [chaos-rs](https://crates.io/crates/chaos-rs)
- [fail](https://crates.io/crates/fail)
- [fail-parallel](https://crates.io/crates/fail-parallel)
- [failpoints](https://crates.io/crates/failpoints)
- [fault-injection](https://crates.io/crates/fault-injection)

## License

- MIT OR Apache-2.0
