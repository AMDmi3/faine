# Changelog

## 0.2.0

- **breaking**: `enable_failpoints` is now a macro for usage and behavior
  consistency with `inject_*` macros.
- **breaking**: Function passed to `Runner::run()` should now accept a single
  argument (see the next item). You may use `_` if you don't need it.
- **added**: Implement trace introspection: you can now inspect which failpoints
  an execution has passed through and which failpoints were activated.
  This is possible both both from within the execution to examine current
  trace, and after `Runner::run` completion to examine all traces. Traces
  can be pretty printed in a fancy way too.
- **changed**: Improve documentation: less repeated example code, more complete
  reference.
- **changed**: Atomic file replace demonstration code was cleaned up and extracted
  into standalone example.

## 0.1.1

- **fixed**: Fix `rust-version` to 1.88, as the code requires if let chains feature.

## 0.1.0

- Initial release.
