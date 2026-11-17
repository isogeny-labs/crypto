# Prospective metadata diagnostic

Root authorized one separate 60-second ordinary software diagnostic after the failed exporter attempt. No exporter retry or tests. Compile only `extern crate crypto_lib;` to metadata using the exact pinned rustc and cached rlib, with `RUSTC_LOG=rustc_metadata::locator=info`. This targets crate-location/metadata rejection, with no assumption that E0463 proves allocation failure.

Hard RLIMIT_AS 2147483648 bytes, core zero, file output limit 8388608 bytes per stream (16 MiB combined stdout/stderr), one rustc process, internal 50-second deadline, outer TERM55+KILL5 seconds. Signal raises into child process-group kill/reap. No dependency rebuild, Cargo, tests, payload, or scientific execution. Preserve logs and terminal receipt even on failure. No automatic retry or cap increase.

Official references: https://rustc-dev-guide.rust-lang.org/compiler-debugging.html ; https://rustc-dev-guide.rust-lang.org/tracing.html ; https://doc.rust-lang.org/rustc/command-line-arguments.html ; https://man7.org/linux/man-pages/man2/getrlimit.2.html .

Exact outer command: `timeout -k 5s 55s python3 metadata-probe.py` from this packet. Exact child argv/env are frozen in metadata-probe.py.
