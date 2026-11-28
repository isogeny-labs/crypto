# CT207 — bounded native toy DLP compilation gate: missing compatibility source

2026-11-18. Continuation on existing draft [crypto #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on #4697. **No solver-cost advantage asserted.** [pcc:unverified]

## Evidence read before change

- Exact prior PR-head `c66278071097522817a517a74af27eda7ef118cb` executed scoped [GitHub Actions run 44300070562](https://github.com/isogeny-labs/crypto/actions/runs/44300070562), job `134787182962`. The independent finite-field Python oracle passed; the native Rust job failed.
- The native log reached `Compiling crypto v0.1.0` after successfully compiling numerous Rust dependencies and internal crates, then stopped before *any* `prime_cover_toy_complete_dlp` tests ran:

```text
error: couldn't read `src/bin/../../compat/ct_dynamic_audit.rs`: No such file or directory (os error 2)
 --> src/bin/ct_dynamic_audit.rs:4:1
4 | include!("../../compat/ct_dynamic_audit.rs");
error: could not compile `crypto` (bin "ct_dynamic_audit") due to 1 previous error
```

- `Cargo.toml` registers `ct_dynamic_audit` as an explicit binary and `src/bin/ct_dynamic_audit.rs` includes the missing file. The file **exists** at `compat/ct_dynamic_audit.rs` on the PR branch; the scoped workflow's sparse checkout omitted it.
- Reviewed `AGENTS.md`, today's shared session log, CT204–206 reports, producer #3877/#4215, cryptanalysis #1523/#1766 and parent infrastructure #4697. Research-KB tooling unavailable in this GitHub connector runtime, so no novelty search or discovery claim is made.

## Change and acceptance

On the same existing #4715 branch, add only `/compat/ct_dynamic_audit.rs` to the `native-toy-dlp` job's sparse checkout. Preserve frozen public curve, tests, Rust 1.98 toolchain, `cargo test --locked --release --test prime_cover_toy_complete_dlp -- --nocapture`, the 40-minute limit, and the absence of S3/EKS/Spot spending.

Change commit: [53eaf817](https://github.com/isogeny-labs/crypto/commit/53eaf817191188aadda6ebf9b723ac5244366a5a).

**This is a checkout repair, not a test pass.** Acceptance requires the exact-new-head native test's successful compilation and execution, including the 43 source and 43 Jacobian BSGS scalar recoveries and explicit negative control. Inspect log if more missing paths or Rust/arithmetic errors appear; do not infer correctness from the independent Python oracle.

## Next decisive experiments and open gates

The F31 toy BSGS uses generic BSGS on both groups. Equal abstract addition counts (expected 670/670) do not make the larger Jacobian implementation faster. After a passing native correctness receipt, account for actual Jacobian/E arithmetic, two pullbacks per target, the norm pushforward, verification and cold-process setup. For the main target, price the original authenticated 695/701 corpus with `C169` versus `C13a + C13b`, `N2_actual` and `D` on matched host, then compare fully cold transferred IC versus the strongest rho, BSGS and IC with blinded targets, memory and uncertainty. CT198 shows the norm-point reuse term cancels between direct and retained routes. The original S3 `HeadObject` 403 currently prevents original-model custody and native pricing; this separate public-toy CI cannot replace it.

No native output, independent PCC signature, protected merge or paid compute is claimed by this report. [pcc:unverified]
