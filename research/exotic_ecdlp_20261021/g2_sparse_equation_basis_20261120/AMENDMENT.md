# Monitoring environment preflight

The first solver-wrapper invocation stopped before launching msolve because system Python 3.13 lacked `psutil`. The checked repository Sage launcher supplies the accepted Python runtime with `psutil` 7.2.2. Its runtime information was saved and hash-pinned before the solver run. The frozen inputs, msolve binary, run order, flags, and caps did not change.

The next execution launched both arms, but each exited at dyld startup: Homebrew's `opt/flint` path points to FLINT 3.5.0, while the installed msolve 0.11.0 binary requires `libflint.24.0.dylib` from FLINT 3.6.0. Both startup receipts and stderr files are retained under `startup_failure/`. A read-only preflight succeeded with `DYLD_LIBRARY_PATH` set to the already installed FLINT 3.6.0 library directory. The rerun pins that library's hash and sets the same environment for both arms; no global Homebrew link is changed.
