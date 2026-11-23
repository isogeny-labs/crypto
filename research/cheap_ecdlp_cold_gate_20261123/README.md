# Cheap complete Prime31 ECDLP — cold-evidence and public-certificate gate

Date: **2026-11-23 UTC** (2026-11-22 Pacific). Owner: cheap ECDLP automation, [PR #4969](https://github.com/isogeny-labs/crypto/pull/4969). Status: **implementation committed but native Rust build/tests not run in this runtime**; mathematical/sample audit only, [pcc:unverified]. This is NOT a new relation-finding oracle, a completed blind unknown-index Prime31 solver, a measured acceleration, or independent signed validation.

## Fixed public problem

- Prime field: p = **2,147,483,647 = 2^31-1**.
- Ordinary prime-order curve: E/Fp: y^2 = x^3 + 847514846*x + 2087444724.
- Group order n = **2,147,446,831**, prime, cofactor 1.
- Generator G = (625037715, 2100627251).
- No smooth subgroup/secret key shortcut, weak-order replacement, or hidden target scalar is admitted into candidate relation finding. This is a 31-bit *algorithm-development* fixture, not real cryptographic security.

The branch is isolated to research/cheap_ecdlp_cold_gate_20261123/ plus the project tracking line and an append-only session log. **No edits** to owned S3, RR5, rootfinding, fiber-domain, graph/LA, transfer, F4 or maintained rho/IC production implementation. Shared-lane coordination: graph-IC #4960; RR5 public extractor #4967; mixed IC controls and isogeny fibers #4956; S3 shared elimination #4922; exact pair-spectrum #4951.

## Prior-art and current baseline audit (do not cross-divide cohorts)

| Source | Exactly completed | Charged comparison | Scientific qualification |
|---|---|---|---|
| [APS vs cold BSGS, 2026-11-16](../prime31_ic_20261115/native/APS_VS_BSGS_COLD_RESULT_20261116.md) | 24/24 unknown-index public Prime31 target logs in both Rust arms | 156.909 ms APS vs 122.565 ms **fresh-table** BSGS, aggregate 24 | APS **loses** to this independent BSGS control. This is one source/target panel, not the same as below |
| [APS vs 32/64-lane rho, 2026-11-16](../prime31_ic_20261115/native/APS_VS_NEG_RHO_WATCHDOG_RESULT_20261116.md) | 24/24 per arm in the *different* held-out Rust panel | 64.742 ms optimized APS vs 125.262 / 140.041 ms watchdog negation rho | APS wins versus **these two particular watchdog arms only**, not strongest WZ or non-generic scaling; original failed/capped rho pilot retained |
| [Mixed-base 1024-Q, 2026-11-17](../prime31_ic_20261115/mixed_base/MIXED_IC_VS_NATIVE_RHO_1024Q_RESULT_20261117.md) | 1024/1024 source-bound Rust logs from both IC and each rho control | **2684.798 ms complete shared IC** vs 3017.608 ms native *unfolded* rho32 / 5625.174 ms native folded rho32 | Restricted batch win IC/unfolded 0.88971, [0.85818,0.92276] target-bootstrap only; no isolated hardware/PCC, generic tables, **not cold one-target**; missing shared BSGS |
| [Run72 B677 K2–4, 2026-11-22](../prime31_f4_admission_20261121/RUN72_WIDTH_SWEEP_RESULT.md) | 9/9 Rust 32-target generic pair-table solver cells, all scalars checked | K4 1223.973–1299.790 ms vs same-runner 232.871–269.195 ms shared BSGS; **4.83–5.40x more costly** | Valid source-specific negative generic-control result; 916,469-key enumeration, no blind algebraic extraction |
| [Run75 preregistered fixed-stage floor, 2026-11-23](../prime31_f4_admission_20261121/RUN75_COLD_PREORACLE_FLOOR_20261123.md) | Calculation from the nine frozen Run72 cells | K4 setup + witness alone **2.316–2.752x** BSGS32, even if following relation-walk costs were zero | Architectural lower *cost floor* conditional on unchanged setup/witness, not a bound for new oracle |
| [Old 3-summand cold C++, 2026-11-15](../prime31_ic_20261115/README.md) | 9/9 full-rank/target solves | B1024 mean 4.043 s; original basic rho ~7–27 ms per Q | Historical non-Rust, non-strong-rho negative control, not eligible to set best frontier |

**Strongest applicable comparison remains open.** The maintained original Rust prime-fast and specialized batched IC baseline is separately being reconciled in [#4956](https://github.com/isogeny-labs/crypto/pull/4956); source-exact library WZ look-ahead / Brent negation reference is specified in [WZ frontier protocol](../prime31_ic_20261115/native/WZ_FRONTIER_PROTOCOL_20261116.md) but is not established as the fully matched external fastest reference here. No observed 1024-Q win or one-panel APS/rho win should be represented as the user's single-cold, stronger-than-best-rho/non-generic success.

## New original-Rust integration: executable certificate/target span gate

- Manifest: [Cargo.toml](Cargo.toml), separate unmerged package depending via path on **existing original native Rust** prime31-algebraic-ic crate and its original Arithmetic, Point, Curve, check_curve and Counts. Original library Git blob observed on this branch: **42e1e7c1912be43190959c5be8f84cf59c867786**. This avoids replacing EC arithmetic with a toy group.
- Source: [src/main.rs](src/main.rs); fixed JSON [public_sample.json](public_sample.json).
- Input: public factor-base points, signed true-RHS relations Σ c_i P_i = [r]G, and target certificates Q + [j]G = Σ c_i P_i. An optional caller-only known log is used **only for independent post-solve rejection**, never as a row or coefficient input.
- Exact checks: original group law replays each relation and target; prime-field F_n incremental echelon with rank/dependence counts; recover c^T L - j if and only if the supplied target coefficient is in the authenticated row span; confirm [d]G=Q. Preserve unresolved targets with recovered=null instead of guessing. Infinity, signs, coefficient bounds, duplicate base entries and poison/replay failures are guarded.
- Source has **7 Rust unit tests**: rank-deficient-but-recovered target, homogeneous/difference relation closure, absence of caller scalar, false RHS, false target witness, false claimed scalar, and negative signed duplicate relation. They are **written, not observed passing**. Math-only local public fixture check independently recomputed G on E, [n]G=O and P3-P2-P1=O; **not a native Rust run, timing result, or PCC attestation**.

Run when the build/compute reservation and actual root source gates permit (no science benchmark during a different lane's timed measurement):

    cargo test --release --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml
    cargo run --release --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml -- research/cheap_ecdlp_cold_gate_20261123/public_sample.json

The temporary stand-alone manifest has **no committed Cargo.lock**, so first resolution and exact dependency lock freeze remain open. Pin Rust 1.100.0 as in the original native source-qualified panels. Running these commands is *future validation*, not asserted in this checkpoint. The certificate engine does NOT discover its own relations; its progress is the final **public-only scalar recovery/correctness gate** for a candidate non-generic extractor.

## Mathematical recovery test and new falsifiable trial

Let A L = b be independently EC-checked linear equations modulo n for unknown factor-base logs L. A public target certificate gives log_G Q = c^T L - j. With a consistent system, this target is uniquely determined by the supplied equations **iff** c lies in the row space of A. If c = u^T A, then log_G Q = u^T b - j. If not, a nullspace direction changes the target while preserving every supplied linear equation. This theorem is not new: it was already used by [Run69](../prime31_f4_admission_20261121/RUN69_AUTHENTIC_TARGET_QUOTIENT_GATE.md); this branch implements a disjoint exact **source-independent certificate interface**, not a new asymptotic attack.

Specific next lead, distinct from the currently owned extractor implementations: preregister a **T=1, partial-rank certificate admission and full cold-cost comparison** on the same unchanged ordinary Prime31 curve after a nonenumerative public-only RR5/RR9 extractor supplies genuine rows. The test compares an early stop at target visibility to full base rank with identical public streams; all true-rank updates and query attempts are charged. **Falsifier:** no uniquely recoverable unknown-index target before full base rank, or any complete-cost regression after including the certificate/replay overhead. This is an incremental recovery/cost hypothesis, not mathematical novelty or a claimed shortcut to discovering relations. Run72 measured K=2/3/4 with T=32 under a generic oracle; it does not establish the T=1 non-generic outcome.

## Exact acceptance protocol and holds

1. **Freeze producer inputs** before a timed run: git SHA, original EC/arithmetic/S3/F4/LA/rho module blobs, Rust compiler/toolchain/lock, factor-base construction, natural target seeds, all public Q, target-unknown guarantee, valid positive/negative cases, timeout/RSS/wall caps, full attempt counter and known exceptional branches. Preserve all censored records and setup/rank/point checks.
2. **Require completed root extractor**: no planted factors, secret scalar in candidate code, pair/quadruple table disguised as polynomial precomputation, or censored or timed-out calls classified as proven empty sets. Replay every returned signed tuple by original group law and check authentic RHS. A target-conditioned result without root lifting is not a full relation.
3. **Use exactly matched references**: best maintained complete original-Rust IC pipeline (owned by #4956), cold native APS with same public Q, same-input strongest implementable original-library WZ-negation rho, and shared generic BSGS (both cold T=1 and amortized T=32/128/1024 reported separately). Same CPU isolation, source revision, shared threads/RAM, operation weights (0,0.1,1 GAE), and no competing builds. Compare strongest eligible reference rather than an arbitrary slow arm.
4. **Independent replay** after freezing candidate artifact: revalidate each EC relation, rank and target log with independent process/source; recompute figures from raw rows; run A/A and paired A/B rounds with corrected confidence intervals; signed PCC plus validation-gate schema before leaderboard admission.
5. **Claim threshold**: 100% completed certified public logs on the frozen eligible cohort, no hidden scalar leakage, precise full memory and complete cold one-target cost, statistically reproducible **strict improvement over both strongest IC and optimized rho**. A multi-target-only win is reported as such, not a cold one-target breakthrough.

No EKS/Spot/paid compute is authorized by this note; current environment exposes **no native subagent launcher, Conductor, research-KB MCP, local rustc/cargo or authenticated experiment admission route**. Do not start a timed experiment from this branch. KB fallback: targeted GitHub code/PR searches in isogeny-labs/crypto, crypto-autoresearcher, cryptanalysis, original logs/reports and publisher source metadata (Semaev, PKM, APS). No exhaustive novelty clearance.

## Primary prior art

- Igor Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves* (2004), https://eprint.iacr.org/2004/031 .
- Christophe Petit, Michiel Kosters, Ange Messeng, *Algebraic Approaches for the Elliptic Curve Discrete Logarithm Problem over Prime Fields* (PKC 2016), https://doi.org/10.1007/978-3-662-49387-8_1 .
- Alessandro Amadori, Federico Pintore, Massimiliano Sala, *On the discrete logarithm problem for prime-field elliptic curves* (2018), https://doi.org/10.1016/j.ffa.2018.01.009 .
- Standard negation-map fruitless-cycle caution: Bernstein, Lange, Schwabe, https://eprint.iacr.org/2011/003 ; source-specific Wiener–Zuccherato protocol linked above.

## Role/handoff and evidence ledger

- **Coordinator (single available execution context):** read current AGENTS.md, ACTIVE, latest Nov23 log, current PR and ownership, selected isolated certificate verifier; performed all shared GitHub writes and source review, declined competing scientific compute.
- **Lead/baseline package (independent read calls, NOT a spawned worker):** compared exact prior Rust 24/1024-Q and B677 evidence with strong-reference caveats; chose T=1 target-span admission as a disjoint follow-up, not a claimed new algorithm.
- **Rust integration package (same execution context, no native worker):** authored the new isolated Rust CLI plus seven exact replay/rank tests; reuses published original curve library.
- **Validation package (same execution context, no native worker):** checked public sample mathematical group equalities; specified mutation controls, exact-source build/replay requirements and complete cold-cost schema. The written tests do **not** establish compiler execution, independently signed verification, independent-worktree review or final solver benefit.
- **Remaining dependency:** get a real unknown-index nonenumerative extractor's public row/witness stream (owned by RR5/RR9), compile the frozen Rust checker and replay actual outputs, then collect properly reserved full-cold T=1/32+/strongest-baseline evidence. Concurrency here was **parallel independent repository read operations**, not three actual agents. No aggregate elapsed-time speedup is claimed.

Retain every negative source and failed/blocked status; do not promote a leaderboard row or merge this draft on a written protocol alone.
