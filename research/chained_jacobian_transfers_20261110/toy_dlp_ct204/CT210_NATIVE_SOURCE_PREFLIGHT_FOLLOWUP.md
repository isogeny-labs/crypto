# CT210 — native source preflight catches missing embedded research/audit inputs

Date: 2026-11-18. Continuation of [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on #4697. **[pcc:unverified]**

## Observed exact-HEAD result

[CT209 workflow run #44330063088](https://github.com/isogeny-labs/crypto/actions/runs/44330063088), source revision `52d07b4b13e47ba8eeef4c9d75823ed8a4e0ba31`:
- Independent F31/F31² quotient/oracle and sextic-obstruction job: **passed**.
- Native job `134891739951`: **failed before cargo compilation**, in the intended source-existence preflight. The five Python scanner unit tests passed.
- Preflight inspected **36 Cargo binary roots and 47 literal include occurrences**. It found three missing paths, rather than spending 20–40 minutes rebuilding crates to discover them:
  1. `src/bin/ct_dynamic_audit.rs` includes `../../research/ct_dynamic_audit_20261102/src/main.rs`.
  2. `src/bin/ecdlp_audit.rs` includes `../../docs/ecdlp-audit/ecdlp-audit-plan.schema.json`.
  3. `src/bin/ecdlp_audit.rs` includes `../../docs/ecdlp-audit/ECDLP_AUDIT_PLAN.mock.json` twice.

The inputs exist as versioned files on the exact branch. This is a sparse-checkout omission, **not** an ECDLP or group-law test result. The path count refers to three distinct missing files and four diagnostic occurrences.

## Scoped remedy and controls

Add only `/research/ct_dynamic_audit_20261102/` and `/docs/ecdlp-audit/` to the already bounded noncone sparse checkout. Preserve the 5-minute independent oracle, Rust 1.98.1 toolchain, 40-minute native job timeout, `CARGO_BUILD_JOBS=2`, and `cargo test --locked --release --test prime_cover_toy_complete_dlp -- --nocapture`. No S3 hydration, EKS or unapproved paid compute.

The source preflight must now complete on the runner before the compiler is invoked. A preflight pass does **not** imply Rust compiles: dynamic includes, transitive modules, and generated files are outside the Python scanner's scope. The decisive acceptance test remains actual native recovery of all 43 targets and deliberate wrong-scalar rejection, followed by same-host full cold costs.

## Sources and future action

Sources: exact-job log from #44330063088; CT204–209 versioned reports; merged crypto #3877; current crypto #4697 and cryptanalysis #1766; [Cargo integration-test behavior](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [Git sparse-checkout semantics](https://git-scm.com/docs/git-sparse-checkout.html). Shared research-KB endpoint and wrapper were unavailable in this runtime; no literature novelty claim.

After the new exact-head CI verdict, if a compiler error is genuinely within the Jacobian code, correct it and independently replay the all-43 recovery. Otherwise document the new source/toolchain error without calling it an arithmetic failure. Keep the independent 695/701 original-corpus S3 403 and physical `C169` versus `C13a + C13b` pricing as separate blockers; no complete single-target speedup is established. **[pcc:unverified]**
