# CT212 — first exact-head native end-to-end toy Jacobian ECDLP success

Date: 2026-11-18. Existing [draft crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on [#4697](https://github.com/isogeny-labs/crypto/pull/4697). Scientific standing: **[pcc:unverified]** (the GitHub CI run is not an independent signed PCC execution attestation).

## Exact observed native result

The already-published CT211 source head `ea467fd505ca4e6d8cf939b4a21bad622a9519ae` finished [GitHub Actions run 44351488002](https://github.com/isogeny-labs/crypto/actions/runs/44351488002), [native job 134968026108](https://github.com/isogeny-labs/crypto/actions/runs/44351488002/job/134968026108), **success**, on 2026-11-18 at 17:11:42 UTC. The previously untested Rust harness actually compiled in release mode and ran:

```sh
cargo test --locked --release --test prime_cover_toy_complete_dlp -- --nocapture
```

Exact relevant log excerpt:

```text
running 2 tests
test wrong_scalar_is_rejected_on_both_groups ... ok
CT204: 43/43 actual toy logarithms; direct E additions=670; Jacobian additions=670; 43 pullbacks and norms plus 43 generator pullbacks; no complete-cost speedup inferred [pcc:unverified]
test all_43_public_logs_recovered_through_real_balanced_jacobian ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

The preflight executed nine Python regression tests successfully and scanned **135 explicit/auto-discovered Cargo binaries, 48 literal includes** before Rust compilation. Native release compilation itself took 17m52s; that is **build time, not DLP solve time**. The separate `independent-field-oracle` job also passed, but its Python arithmetic is not a full independent implementation of genus-two BSGS and cannot confer PCC standing.

## What the experiment establishes

- On `E/F31: y²=x³+3`, `G=(1,2)` with order 43, and `C:v²=u⁶+3`, compiled Rust recovered **all 43 known-answer targets** both directly in E and through `Jac(C)` by the same generic BSGS code. The test reconstructs the Jacobian class, norms the pulled-back target, and verifies the recovered scalar on E.
- A separate test rejected a deliberately wrong scalar 18 for a target with log 17 on both groups.
- The sum of BSGS group additions was **670 elliptic and 670 Jacobian** across the 43 targets. Transfer additionally used **86 pullbacks and 43 pushforwards** in that test, with reconstruction, verification, and ordinary fixture-generation work outside those counted BSGS additions.
- The 0.02-second number is the reported **entire Rust test execution elapsed time** on this tiny fixture, not per-target cold time, a paired speed comparison, a primitive timing, a memory measurement or a benchmark of a scalable solver. Equal addition counts do not imply equal CPU expense.

## Negative scientific scope and follow-through

This is now **executed** end-to-end toy DLP recovery, not merely a compiled proposal. It is **not** an index-calculus attack, independently blinded target solving, a large-subgroup demonstration, or complete cold-performance advantage. Do not use the 43 known-answer fixture to claim an ECDLP speedup. No original 695/701-record custody, C169/C13a/C13b/N2_actual/D physical measurements, optimized rho/BSGS/IC comparison, uncertainty, memory, or independent PCC receipt was obtained here.

Cross-checked current lanes before writing: merged root [#3877](https://github.com/isogeny-labs/crypto/pull/3877), original norm-point frontier [#4215](https://github.com/isogeny-labs/crypto/pull/4215), implementation [#4697](https://github.com/isogeny-labs/crypto/pull/4697), cryptanalysis [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), [#1766](https://github.com/isogeny-labs/cryptanalysis/pull/1766) and mirror [#1770](https://github.com/isogeny-labs/cryptanalysis/pull/1770), plus GPU IC [#4178](https://github.com/isogeny-labs/crypto/pull/4178). Neither the independent end-to-end cost-gate nor the original-model S3 authorization problem is addressed by the toy pass. No research-KB connector was available in this runtime; no novelty assertion is made.

**Next experiment with highest relevance:** Obtain authorized original-model hydration and native replay on the frozen 695/701 corpus, then pair the **actual** `C169`, `C13a`, `C13b`, `N2_actual`, `D` primitive costs on one host, counting norm-point reuse once. Only then conduct complete cold source-target solves with strongest rho/BSGS/IC and full setup, transfer, linear algebra, recovery, verification, memory, uncertainty and independent PCC. In parallel, a genuine even-degree genus-two destination IC remains an unimplemented solver requirement.

This CT212 report is a documentation/evidence update on the existing feature branch; no arithmetic source or original frozen fixture was changed. No paid EKS/S3/Spot compute or protected merge.
