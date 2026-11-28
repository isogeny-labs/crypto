# Combined qualification v8 concurrent-traversal successor

V8 preserves the v7 mathematics, all eight overlays, all 50 driver-source
bindings, exact cache bytes and modes, generated locks, panels, resource caps,
and single-attempt policy. It corrects only filesystem accounting during a
live compiler: Rust/Cargo may remove temporary files and directories between
directory enumeration and `lstat`.

The shared scanner tolerates only an `ENOENT` that is confirmed by a second
`lstat` and lies inside an explicitly mutable compiler tree (`cache/target` or
`cache/tmp`), or has one of the exact atomic receipt-temp names. Permission
errors, symlinks, FIFOs and other special files, non-`ENOENT` errors,
disappearance under immutable source/driver trees, and unrecognized durable
output disappearance remain fatal. Final source/driver verification and
post-cleanup output sealing remain strict.

The next charged invocation runs a solver-free concurrent traversal control
inside the admitted worker before materialization and compilation. Its
coordinated deletion checks
both a compiler-temp directory and file, and its negative controls require
fail-closed handling of immutable disappearance, symlink, FIFO, permission
error, and an unrecognized durable-output disappearance. The temporary fixture
is capped at 1 MiB, all helper threads are joined, and its directory is removed
before the materializer's fresh-workspace check. Source preparation
does not run that control, Cargo, tests, a resolver, or a scientific payload.
The worker requires the returned control elapsed time to be at most 30 seconds;
the enclosing systemd/build deadlines remain the hard runtime stops rather than
a separate 30-second watchdog.

Combined limits remain 1800 build + 600 execution seconds, 2 CPU / 16 GiB,
one attempt, no automatic retry. The v7 failure is preserved: materialization
and cache-mode verification passed; after 287.088 seconds of Cargo activity the
old monitor raised `FileNotFoundError` for transient
`target/release/deps/rmetaOgIf8S`, killed the compiler, sealed receipts, and
executed no tests.

Pending: independent v8 source review, publication, current terminal ledger,
live owner, serial reconciliation, and a separately reviewed release. The
admission draft remains false with null current ledger and lease fields.

Sources checked: the pinned v7 worker/terminal receipts, Python 3.9 `os.walk`
behavior in the installed standard library, and the existing durable atomic
write implementation. The research-KB wrapper is unavailable in this checkout;
no novelty claim is made.
