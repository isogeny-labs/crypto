# Offline dependency-lock gate: source review

Verified the final 49-file closure against its exact source hashes and byte sizes. All explicit Cargo path dependencies and declared target paths resolve within that preserved layout. No Cargo resolution, compiler payload, test or native unit was run.

The explicit amendment changes two Cargo manifest entries: the archived native package is named kummer-five-leads-study and its consumer retains the five-leads-study dependency key through a package alias. Rust files remain unchanged. The inspected closure has no CARGO_PKG or CARGO_CRATE references. Its environment macros use CARGO_MANIFEST_DIR, whose layout is preserved. This is source-level correspondence; binary identity or runtime equivalence has not been measured.

The initial broad writable path, missing file limit and incomplete caller custody were corrected. Only the fresh owned workspace is writable; donor inputs remain read-only. The unit uses 1 CPU, 2 GiB, no swap and 64 tasks, with a 16 MiB per-file bound. Worker work stops at 45 seconds; native lifetime is 50 seconds plus five for stopping; the caller has a 59-second timeout plus one-second kill bound. The entire 60 seconds needs its own campaign reservation.

The rustc proxy accepts only the exact -vV version query against the hash-verified compiler binary; every compilation argument is rejected. RUSTDOC is false. Cargo uses an empty private home, pinned vendor and offline mode under native network isolation.

Type=exec and RemainAfterExit retain initial invocation identity. Caller checks that identity before stopping, records native status before stop, and checks the recorded cgroup. Post-stop unit disappearance is cleanup evidence only. Success requires the worker result, captured native success/exit0/MainPID0 and empty owned cgroup. Missing receipt or abnormal exit cannot admit a lock.

No remaining blocking source finding was identified in manifest 729ecb2b. Exact release publication, serial predecessor termination and a separate reservation remain parent-owned launch checks. Generated lock bytes, if any, must be published before the separately reviewed combined build. No runtime-fit or dependency-resolution result is asserted.

Sources checked: actual gate scripts and manifests, prior signed producer and Kummer driver packets, and retained donor manifest. KB unavailable; exact repository primary sources were used. Initial review findings remain in pending-findings.json as historical observations superseded by this final review.
