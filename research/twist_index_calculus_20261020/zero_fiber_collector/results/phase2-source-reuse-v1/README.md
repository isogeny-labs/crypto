# Exact source reuse and protected files

Committed and working-tree `Cargo.toml`, `Cargo.lock`, `src/` and `examples/`
both match integration source `1055dcbcdb4998e78e22c900ab737ddc9913de70`
(exit 0; empty diffs). Its retained 4,492-pass library gate and 1+3 example
tests are reused as source-identical evidence, not claimed as new executions.
The changed standalone native study has its own fresh 89-test gate.

The first protected-file check uses this child worktree for the parent's
relative manifest and cannot resolve its two files (exit 1). The corrected
v2 check runs from `/home/ec2-user/crypto-twist-ic` and both hashes match
(exit 0). Neither protected file is edited, restored or staged. The original
wrong-directory receipt remains unchanged.
