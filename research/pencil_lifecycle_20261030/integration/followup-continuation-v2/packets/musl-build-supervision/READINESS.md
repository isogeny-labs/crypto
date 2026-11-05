# Prospective musl build supervision — not executed

Observed local commit da1464aea27534f6daf0bc3de0cdc687030b66d2. At the recorded check, cargo PID2079805 was active in the pencil worktree with no explicit CARGO_TARGET_DIR. No musl ecbench existed under /tmp/pencil-musl-target. A full tracked-source cleanliness check exceeded20seconds; cleanliness is UNKNOWN, not established. Exact timestamp and observations are in readiness-observation.json. Reconcile the current owner before a build; this snapshot is not a launch authorization.

Executable private wrapper: build-receipted.py. Python syntax was parsed with ast.parse; the wrapper itself was not executed. Root can run it only after its reviewed clean commit and source/cache ownership are established:

```
/var/tmp/pencil-musl-build-supervision-20261031/build-receipted.py \
  --commit da1464aea27534f6daf0bc3de0cdc687030b66d2 \
  --seconds ROOT_SELECTED_SOFTWARE_CAP \
  --receipt-dir /var/tmp/pencil-final-musl-build-UNIQUE_RECEIPT
```

If the final approved commit changes, supply that exact new commit. The required seconds argument is a root-selected software-build ceiling, accepted only from1 to3600; no campaign worker allowance is inferred or consumed by planning. The receipt directory must be new and private. Do not substitute a historical binary on failure.

Pre-build gates: nonblocking dedicated cache-owner lock; inspect active cargo/rustc owners by cwd or CARGO_TARGET_DIR; exact HEAD/tree; tracked clean and no untracked source inputs; Cargo.toml/Cargo.lock hashes; actual Rust1.99.1 identity; pinned private compiler wrapper/specs/libc and qualified manifest hashes. Git checks have120-second bounds and fail closed. The source lease must prevent concurrent edits for the entire build; pre/post checks alone cannot prove that a transient edit never occurred.

Execution: cargo+1.99.1 rustc locked/offline release/no-default-features musl, jobs2, target-cpu=x86-64, source commit/dirty0 metadata, native CC private musl-gcc, REALGCC=/usr/bin/gcc. Environment is restricted to PATH/HOME/CARGO_HOME/RUSTUP_HOME/TMPDIR/LC_ALL plus explicit build bindings. No ambient RUSTFLAGS/linker/CC/profile override or cloud credential is copied into the build environment. Uses /tmp/pencil-musl-target; no owner rebuild, ecbench execution, image or cloud command.

Supervision: GNU timeout sends termination at the supplied deadline and kills after5seconds; its process-group PID is saved before waiting. Python also supervises with an outer seconds+15 bound and terminates/reaps the process group on interruption/error. A hard kill of the Python wrapper can prevent a final receipt, so preserve/reconcile the saved process group and timeout owner rather than automatically rerunning. Partial logs and nonzero statuses remain. No automatic retry.

Required bindings retained by wrapper: requested/final commit and tree, Cargo input hashes, source clean checks before/after, toolchain identity, exact environment/command, compiler inputs, timestamps/exit, full verbose build stdout/stderr, printed final linker invocation, linker map hash, final copied ecbench SHA256, ELF machine/interpreter/NEEDED checks, and candidate CRT/libc object paths/hashes resolved from map/invocation.

The CRT/libc extraction is deliberately a candidate inventory: independent review must confirm which paths were selected and bind all actual startup objects and libc archives (including Rust self-contained paths). It is not proof from CC alone. Final image/base/runtime bindings remain unset. Root must produce the publication build-info.json using the verified final binary and source plus the independently retained owner receipt; the wrapper does not publish or assert runtime qualification.

A clean-source or owner gate failure exits before cargo. A build/link/ELF failure retains the receipt and logs. Even a successful static candidate is not a source-elliptic test, image qualification or scientific run.
