# Optimized-policy integrity correction

The driver now returns an error if its options file changes during the run, after preserving rows and an analysis marked `invalid_mutated_arm_or_policy`. The Unix-only regression runs a fake solver that changes the shared options file and checks both the error and retained evidence. Exact Rust 1.78 locked offline release panel tests pass 13/13, with changed-file rustfmt passing. [pcc:unverified]

`development.tar.xz` losslessly retains the final-source test/fmt/Clippy bodies and `native-source.sha256`. Strict Clippy still stops only at the unchanged `format_collect` warning in `panel_driver.rs:238`; its raw failure and exit 101 are retained. A repeat scoped to that inherited lint class passes. These are local correctness controls, not native L2 measurement, calibrated operations or signed review. [pcc:unverified]
