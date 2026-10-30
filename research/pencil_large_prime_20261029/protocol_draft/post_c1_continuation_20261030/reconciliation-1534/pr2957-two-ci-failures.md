# PR 2957: two CI failures, read-only inspection

Observed 2026-10-30. No edits, reruns, builds or cloud mutations.

Both requested jobs fail at compilation with the same baseline syntax error:

- Rust all-targets run41079004238/job124145788890: `RUSTFLAGS=-Awarnings cargo check --workspace --all-targets`; error `this file contains an unclosed delimiter`, `src/cryptanalysis/ecbench/methods.rs:8821:3`; exits101.
- Factor-base sweep CLI run41079004365/job124145550593: step `Native shape, polynomial and progress comparison unit tests`; same unclosed delimiter at methods.rs:8821:3; exits101. Subsequent CLI witnesses/caps/interruption/resume/corruption and independent audit steps were skipped. This is not evidence of a factor-base test assertion failure.

Both checkout logs identify the actual CI merge commit `cea0c6d738b01b82e3c595bcad0937f75989ecad`, merging PR head `9e4f191136040f5df56ace3590907a6b5056c4ce` into then-base `496084e3b4f2d5e8b123fcdb8e1e429c75c9187a`. The PR API subsequently reported a different current base, so comparison must use the checked-out baseline above.

Read-only GitHub contents retrieval shows methods.rs at that CI baseline and merge are byte-identical, SHA256 `c6900af99bc480e733b3d32de1d144d94b7c1a43a7e80d4a85713f60d8c539b8`.

At lines8675–8683, `neg_strong_tests::method` opens `resolve(&MethodSpec {` and its params chain ends `.collect(),` immediately before a new `#[cfg(test)] mod tensor_contraction_tests {`, without closing the struct/call/function/module. The compiler identifies additional unclosed delimiters later in the same combined tail. This malformed file is already present in the exact baseline. The reviewed PR diff contains research evidence/docs, not this methods.rs modification.

Conclusion: both failures are caused by the same inherited baseline syntax corruption, not by the PR's evidence/publication changes. Applicable checks are still failed; this attribution does not make CI passing. Root should coordinate repair of the baseline methods.rs tail, then rerun applicable checks on the resulting exact head. A brace-only patch should not be guessed without comparing the contributing original test modules.

Private evidence:
- /tmp/pr2957-rust-alltargets-failed.log
- /tmp/pr2957-rust-job.log
- /tmp/pr2957-factorbase-job.log
- /tmp/pr2957-base-methods.rs
- /tmp/pr2957-merge-methods.rs

Initial gh run log-failed call for the factor-base workflow was refused because the overall workflow remained in progress; direct completed-job log retrieval succeeded. No job was restarted.
