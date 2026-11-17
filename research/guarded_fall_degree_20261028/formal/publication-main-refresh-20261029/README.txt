PR 2859: current-main refresh validation, 2026-10-29

This separate receipt covers Rust source 6796d63d91c3dff4e5a5e7c3bc55079e51a93db5 at f46343729454468b1979d6d686ff008b60b2d14a.
It does not replace the older 5251-pass receipt or any mathematical proof evidence.
The entire prior guarded-study Git tree is unchanged from the published predecessor.
All four proof-source checksum manifests passed recheck.

Full command: RAYON_NUM_THREADS=8 cargo test --release --lib --locked -- --test-threads=1
test result: ok. 5261 passed; 0 failed; 123 ignored; 0 measured; 0 filtered out; finished in 1896.34s
Actual Cargo exit: 0. This is development validation, not a performance claim.
The serialized test configuration matches main's documented ML-export acceptance;
all test entries were retained, with no filter or newly ignored test.

The earlier four-thread attempt ended by SIGKILL without an outer suite summary or
supervisor exit receipt. Its exact external cause is not established. The full
partial output is preserved in a lossless archive under the 64KiB data threshold.
verification.json binds source objects, actual binary, raw/archive hashes and exits.
The complete successful development log is retained locally as named in the receipt.

All original theorem hypotheses, dated partial receipts and numerical evidence remain
unchanged. No scientific experiment, scope expansion, recovery or security claim.
