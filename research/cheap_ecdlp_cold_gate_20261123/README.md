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
- Source has **15 authored Rust unit tests**: the prior eleven plus four opt-in prefix-stop controls (normal/full consistency, unprocessed poison, prefix-zero preflight, and empty-target refusal). The earlier rank-deficient fixture additionally checks that a valid group witness is reported as replayed even while its logarithm remains unresolved. **None has been observed compiling or passing natively in this runtime.** Math-only local public fixture checks independently recomputed G on E, [n]G=O and P3-P2-P1=O; these are not native Rust results or PCC attestation.
- New result fields: `first_determined_relation_prefix` (null if underdetermined), `group_replay_passed` (true for every admitted target, regardless of scalar rank) and `scalar_replay_passed` (true only after successful scalar recovery and `[d]G=Q`). The previous version misleadingly set `group_replay_passed=false` for valid but underdetermined public witnesses; that reporting bug is corrected.

Run when the build/compute reservation and actual root source gates permit (no science benchmark during a different lane's timed measurement):

    cargo test --release --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml
    cargo run --release --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml -- research/cheap_ecdlp_cold_gate_20261123/public_sample.json

The temporary stand-alone manifest has **no committed Cargo.lock**, so first resolution and exact dependency lock freeze remain open. Pin Rust 1.100.0 as in the original native source-qualified panels. Running these commands is *future validation*, not asserted in this checkpoint. The certificate engine does NOT discover its own relations; its progress is the final **public-only scalar recovery/correctness gate** for a candidate non-generic extractor.

## Mathematical recovery test and new falsifiable trial

Let A L = b be independently EC-checked linear equations modulo n for unknown factor-base logs L. A public target certificate gives log_G Q = c^T L - j. With a consistent system, this target is uniquely determined by the supplied equations **iff** c lies in the row space of A. If c = u^T A, then log_G Q = u^T b - j. If not, a nullspace direction changes the target while preserving every supplied linear equation. This theorem is not new: it was already used by [Run69](../prime31_f4_admission_20261121/RUN69_AUTHENTIC_TARGET_QUOTIENT_GATE.md); this branch implements a disjoint exact **source-independent certificate interface**, not a new asymptotic attack.

Specific next lead, distinct from the currently owned extractor implementations: preregister a **T=1, partial-rank certificate admission and full cold-cost comparison** on the same unchanged ordinary Prime31 curve after a nonenumerative public-only RR5/RR9 extractor supplies genuine rows. The test compares an early stop at target visibility to full base rank with identical public streams; all true-rank updates and query attempts are charged. **Falsifier:** no uniquely recoverable unknown-index target before full base rank, or any complete-cost regression after including the certificate/replay overhead. This is an incremental recovery/cost hypothesis, not mathematical novelty or a claimed shortcut to discovering relations. Run72 measured K=2/3/4 with T=32 under a generic oracle; it does not establish the T=1 non-generic outcome.

## Chronological target-span admission without rescanning prefixes (2026-11-23)

The basis now stores the **1-based arrival index** of each independent relation row, with the original row order untouched. For a fixed public target coefficient vector `c`, reduce `(c,0)` through the chronological normalized pivots. If all coefficient columns reduce to zero, `c` has one unique expansion in this independent basis. The earliest relation prefix that determines `c^T L` is **the maximum birth index among the pivots used with nonzero coefficients**; the empty expansion has prefix 0. This follows because a linear combination involving any basis vector born after prefix `t` cannot equal a combination of the earlier independent vectors. The input rows may be dependent, so neither total relation count nor rank alone can be substituted for this birth-index maximum.

This adds one chronological diagnostic per target without rerunning Gaussian elimination once per incoming row. It does **not** stop extraction or avoid reading/verifying later rows in the current CLI; no cost benefit or elapsed-time advantage is asserted. The future streaming integration can use this value to compare a physically charged early-stop candidate with the same frozen full stream, after including the cost of authentic relation discovery, witness formation, replay and matrix work. Every reported target's public group equality is already checked before row-space admission. Determined scalar replay is a separate boolean.

**Local non-Rust mathematical cross-check:** an independently coded Python elimination oracle, seeded with 15125, checked 12,000 synthetic consistent row-stream instances (dimensions 1–8, relation counts 0–19) against brute-force earliest-prefix recomputation and verified recovered functionals against planted values. This only supports the mathematical chronology rule, **not the compiled Rust implementation, source independence for PCC, an actual ECDLP solve or runtime cost**. [pcc:unverified]

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

## Opt-in physically checked relation-prefix gate (2026-11-23 UTC)

Original-Rust source on this PR: commit [f398217e](https://github.com/isogeny-labs/crypto/commit/f398217e0bd9b5a09af07542f4ee578d3888c6a8), file `src/main.rs` blob `178c4712f1c80f64924cd2d67da2830ea10af7ec`. A new optional top-level input key,

```json
"stop_when_all_targets_determined": true
```

activates an **early-stop replay experiment**, not an extractor or timed online algorithm. Default (key absent/false) remains full validation of every submitted relation. The opt-in mode validates **every** public target point, signed decomposition, offset and any harness claimed-log *before* consuming relation 1, then authenticates rows in input order using original elliptic-curve arithmetic. It tests visibility at prefix 0 and after each **independent** relation; dependent rows cannot enlarge a row space. It halts when **all** target coefficient vectors are in the span, recovers each scalar from the authenticated rows and verifies `[d]G=Q`. The empty-target case is refused rather than counted as a successful zero-work stop.

The JSON report distinguishes `relation_count` (submitted), `checked_relation_count`, `unchecked_relation_count`, `dependent_relations` (among checked rows), `prefix_span_queries`, and the selected mode. **Unchecked tail rows are not authenticated** and must never be counted as successful or failed relations. The whole JSON is still read and parsed before replay, and all target decomposition witnesses are supplied by the caller; the program has **no relation discovery, root solving, online file IO, timing or memory accounting**. Each prefix-span query is counted, but its modular arithmetic is not yet fully priced, so it cannot support any candidate-cost saving. The report's EC operation counters include all preflight target work and all consumed relation work, not a cold extraction charge.

Fifteen Rust `#[test]` functions are now authored, **not compiled/executed in this runtime**. The four new cases check exact consumed/unconsumed counts and same recovered scalar as full replay; a malformed tail is accepted only as *unprocessed* in the opt-in mode and rejected in full mode; a prefix-zero identity target is resolved without consuming relations and a false public witness is rejected at preflight; and a target-free early-stop request is rejected. This makes the partial-validation boundary explicit instead of accidentally claiming to have verified omitted inputs. [pcc:unverified]

A separate **mathematical-only Python reference**, random seed `0xCE31C0DE`, compared 4,000 synthetic consistent linear streams over `F_101` (dimensions 1–9; up to 26 rows; one to four independent random target functionals, with duplicate and zero rows) against full per-prefix rank recomputation. The modeled stopping prefix agreed in all 4,000; 3,137 synthetic streams stopped before the submitted end. This did **not** execute the Rust source, elliptic-curve arithmetic, actual Prime31 extraction or a timed job, and is neither external independent validation nor a statistical performance result. [pcc:unverified]

**Integration dependency:** ingest frozen, public-only naturally extracted RR5/RR9 tuples and target decompositions from the owning lane, run the original-Rust scoped tests and a signed independent checker, then compare matched cold T=1 costs with all prefix queries and preflight/extraction costs charged. This scope remains disjoint from graph/LA #4972, RR5 #4970/#4974, S3 and transfer lanes. Do not substitute planted scalar witnesses, reinterpret an unprocessed tail as proven empty, or mix reference cohorts.

## Exact interface reconciliation with the live RR5 lane (2026-11-23)

Review of draft [RR5 public extraction #4970](https://github.com/isogeny-labs/crypto/pull/4970) at head `95e5062573e24d124235756c062e2ab8178aac4c`, [`native/relation_adapter.rs`](../prime31_rr5_public_relations_20261123/native/relation_adapter.rs) blob `ae4fca2ca6e95304b8473a821bd75419c7d1d267`, and [`native/solver.rs`](../prime31_rr5_public_relations_20261123/native/solver.rs) blob `d87fc00c110bd203aedd9beb1b2e7c9a63263ff8` exposed a **specific remaining integration mismatch**: this PR's certificate format authenticates only rows `sum c_i P_i = [r]G` plus a separately supplied public target decomposition. RR5's actual public-query form is `sum c_i P_i = [a]G + [b]Q`, producing the augmented linear row `[c_0, ..., c_(B-1), -b | a]` and recovering `log_G Q` as the final-coordinate functional. A genuine `b != 0` row **cannot** be converted to the existing `rhs=r` input without already knowing the target logarithm. Do **not** fabricate this conversion, feed hidden sample logs into the extractor, or treat RR5's toy F11 run as a Prime31 solve.

The owned RR5 lane already implements signed relation extraction and augmented target recovery, while [graph #4972](https://github.com/isogeny-labs/crypto/pull/4972) owns the sparse linear-algebra target-uniqueness correction. **Do not duplicate either implementation here.** At a frozen compatible revision, the disjoint next integration is a read-only independent public-`Q` certificate checker or adapter that consumes RR5 receipts and authenticates `aG+bQ` using this repository's original EC code, independently recomputes augmented row span and `[d]G=Q`, and records exact consumed/checkpointed row counts. The present `stop_when_all_targets_determined` mode applies **only to the existing G-only/decomposition certificate interface**, not RR5's augmented rows. The strongest cold IC reference remains unqualified for this new mixed stream; the merged [#4956](https://github.com/isogeny-labs/crypto/pull/4956) tracked closure attempts but does not itself supply a matched winning measurement. [pcc:unverified]


## 2026-11-23 08:20 UTC: independent mixed-RHS public receipt gate (continuation)

The source-frozen RR5 adapter on draft [#4970](https://github.com/isogeny-labs/crypto/pull/4970) at head `95e5062573e24d124235756c062e2ab8178aac4c`, `native/relation_adapter.rs` blob `ae4fca2ca6e95304b8473a821bd75419c7d1d267`, accepts authentic public equations `sum c_i P_i = [a]G+[b]Q`. That is *not* the G-only certificate shape in this branch's original CLI. A nonzero `b` must not be replaced by an invented `r` depending on `log_G Q`.

A **new, disjoint, source-only** checker lives at [`src/bin/rr5_receipt_check.rs`](src/bin/rr5_receipt_check.rs), blob `44f4cd3807358651951b7493743237a4813e1784` as fetched after commit `f094aa75f81205a010f5c817256a12d46400d4dd`. It uses the maintained original Prime31 elliptic-curve arithmetic and independently implements a bounded Gauss-Jordan receipt verification, **without importing the RR5 producer/extractor or its echelon solver**. It accepts only public `base`, `target_q`, `relations:[{terms,a,b}]` and an optional prefix-only mode; no scalar claim or factor-base logarithm is an input. Each processed row is authenticated under the original group law before insertion as `[c_0,...,c_(B-1),-b | a]`. A pivot in the final, last-indexed target column uniquely determines `log_G Q`, even if factor-base columns remain free; the recovered value is replayed by `[d]G=Q`. This is a source-independent *algorithmic implementation*, not separately signed scientific independence.

The input [`public_rr5_sample.json`](public_rr5_sample.json) is **synthetic** Prime31 arithmetic: base `[G,2G,3G,4G]`, public `Q=5G=(281707092,1865416923)`, and two rows `P_0=G` and `P_0=[n-4]G+Q`. The checker is intended to recover `5` from the augmented rank-two system while the four factor-base logs are not all determined. It is **not** a root-extractor output and **not** a blind Prime31 ECDLP trial. The caller's known fixture construction is not accessible as a checker input. The original G-only binary's **15 authored Rust tests** do not validate this new binary; its separate Rust source currently contains **11 authored `#[test]` cases, none compiled or run in this environment**. [pcc:unverified]

**Math-only development cross-checks, not native Rust execution:** a separate Python implementation over the original Prime31 curve verified that the published base points are `G,2G,3G,4G`, that `Q=5G`, that `[n]G=O` and `[n-4]G+Q=P_0`. Independent exact-rank recomputation over F101 for **2,500 synthetic consistent augmented streams / 28,343 prefixes** agreed with a separate implementation of the new Gauss-Jordan pivot rule, including target uniqueness and recovered scalar. In 2,153 simulated streams the target became uniquely determined, in 2,050 before the full stream; these synthetic proportions say **nothing about natural relation yield, runtime or algorithmic speedup**. Hostile input controls now reject noncanonical curve coordinates and oversized signed coefficients before summing, avoiding i64 overflow on malformed receipts. A signed independent validator has not run. [pcc:unverified]

**Read-only replay CLI (future admissible runner):**
```sh
cargo test --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml --bin rr5_receipt_check --release
cargo run --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml --bin rr5_receipt_check --release -- research/cheap_ecdlp_cold_gate_20261123/public_rr5_sample.json
```
These commands are *not* a claim they executed. This environment exposes no `cargo`/`rustc`; we deliberately did not launch competing cloud/CI builds, timed jobs, extra agents or paid compute. The opt-in `stop_when_target_determined` mode counts checked rows and labels the tail *unchecked*, never authenticated or proven empty. Both modes read/parse the whole JSON, so stopping is not physical extraction termination. Modular elimination/inversion cost, input parsing, memory, extraction/root solving and failed queries are **not fully priced**. No cost ratio or accepted solve exists.

**Source/capacity coordination:** #4970 owns the signed RR5 extractor/first solver; #4972 owns graph sparse-LA target uniqueness; #4974/#4976 own RR5/FASS root/selector work; maintained IC and optimized negation-rho reference runs have their own reservations. This branch edits none of those sources. The central scientific blocker is **a natural, nonenumerative blind Prime31 mixed-RHS receipt with complete failure/censoring records**. At that frozen artifact, compile/test the exact Rust binary and independently authenticate both this checker and the producer's output, then measure fully charged cold T=1 against the strongest same-cohort IC and optimized negation-rho (plus BSGS); separately evaluate T=32/128/1024. Preserve the prior APS/BSGS loss and Run72/Run75 negative evidence. [pcc:unverified]

**Coordinator / package ledger:** one coordinator; **zero native independent worker slots available**. Lead/baseline package reviewed #4956/#4970/#4972 and the prior strongest controls, Rust package wrote only this independent consumer and synthetic fixture, and validation package used an independent Python group/rank reference. These were coordinator-executed logical work packages with parallel independent GitHub reads, **not concurrent spawned agents or a separate signed validator**, and no elapsed-time gain is claimed. KB MCP and Conductor unavailable; fallback reviewed committed sources and Semaev (2004), Petit–Kosters–Messeng (PKC 2016), without asserting novelty.

**Latest source-only refinement:** commit [`c3d7fea2`](https://github.com/isogeny-labs/crypto/commit/c3d7fea2fc487246b2684382af5a3d1e9fbc9f71), checker blob `4d985c2d0c922230ebcbb58c0a8349909397a2e1`. The independent mixed-RHS report now exposes two **partial, non-priced** Gauss-Jordan counters (row multiply/subtract pairs and Fermat inversions); a Rust test explicitly distinguishes an unrelated early row that delays the target's first determined prefix from a change to its value. This is an authored source update, not a run. [pcc:unverified]


## 2026-11-23 09:30 UTC: independently implemented Python replay bridge for native RR5 records

Added two **isolated read-only development files** to this draft lane:

- [independent_rr5_replay.py](independent_rr5_replay.py) — standard-library Python verifier on the *same ordinary* Prime31 curve. Independently calculates each authenticated group equation with affine EC addition; checks canonical coordinates, signed coefficients, complete four-leaf bounds, duplicate projective row keys, producer disposition/rank history, and public \`[d]G=Q\`. It maintains **two separately reduced coefficient systems** (with and without the Q column) to enforce the exact target-visibility criterion \`rank([C|-b]) - rank(C) == 1\`, and separately eliminates base columns with RHS to compute \`d\`. The two invariants must agree. This is not an implementation of the natural RR5 extractor, optimized linear algebra or a paid benchmark.
- [test_independent_rr5_replay.py](test_independent_rr5_replay.py) — direct and hostile input cases, including mutated \`b\`, malformed points/leaves, duplicate and identity rows, ordering changes, incorrect producer rank/answer, and 120 reproducible synthetic signed-relation streams. Test fixture construction uses known small multiples **outside** the verifier; its CLI never accepts point logs or a target secret.

This bridge accepts either the existing explicit [public_rr5_sample.json](public_rr5_sample.json) format (\`--receipt\`) or the **native #4970 \`SolveRecord\` output** (\`--record\`) with an independently supplied public factor-base manifest (\`--base\`). In the latter mode, the manifest must identify exactly \`{"curve":"ordinary_prime31","p":2147483647,"n":2147446831,"a":847514846,"b":2087444724,"g":[625037715,2100627251],"base":[[x,y],...]}\`. It replays every recorded \`attempts[].point = [a]G+[b]Q\`, then each accepted \`attempts[].receipts[].canonical_terms, a, b\` in chronological order. It refuses producer rank, disposition, per-receipt recovered scalar, terminal rank/scalar or final-replay mismatch. It reports SHA-256 of original input bytes, attempts, extracted candidate-row count, accepted authenticated rows, rank, target visibility and terminal status, while retaining the distinction between attempted/extracted and accepted rows. **It cannot reconstruct extractor failures not actually recorded in the upstream JSON**; these must remain in frozen producer evidence. The manifest supplies point coordinates, never logarithms.

Commands for the read-only development bridge (tested locally using Python 3.13.5):

\`\`\`sh
python3 -m unittest discover -s research/cheap_ecdlp_cold_gate_20261123 -p 'test_independent_rr5_replay.py' -v
python3 research/cheap_ecdlp_cold_gate_20261123/independent_rr5_replay.py --receipt research/cheap_ecdlp_cold_gate_20261123/public_rr5_sample.json
# Once a real source-frozen native SolveRecord and matching public base manifest exist:
python3 research/cheap_ecdlp_cold_gate_20261123/independent_rr5_replay.py --record SOLVE_RECORD.json --base PUBLIC_BASE_MANIFEST.json
\`\`\`

**Development evidence (not PCC):** 8/8 Python unit tests passed locally (including 120 seeded synthetic streams); direct synthetic Prime31 fixture replay returned \`authenticated_rows=2\`, \`independent_rank=2\`, \`rank_without_target_column=1\`, \`first_target_prefix=2\`, \`recovered=5\`. A separate, deliberately **synthetic** native-\`SolveRecord\`-shaped JSON and public base manifest also replayed via the second CLI mode to that same result. After a local cleanup, the **exact Git blob hashes** of both executed Python sources match GitHub readback: checker `346036d8e1df814f572e8dce1c079178dc77d9b1` and tests `a3593ab19e790be02c36044d156012376988b8a4`. Re-running the tests and both synthetic CLI modes at these exact source bytes succeeded under Python 3.13.5. The Rust compiler/tests and actual RR5 natural producer have **not** run in this session. No genuine unknown-index extracted Prime31 receipt, independent signed validator, new complete-cost result, elapsed-time speedup, or advantage claim exists. [pcc:unverified]

This change is strictly disjoint from #4970/#4974/#4976's extraction and #4972's sparse LA, and does not take over the maintained IC/rho baseline lanes. Primary research background remained Semaev (2004) and Petit–Kosters–Messeng (PKC 2016), already linked above; inspected the actual #4970 public \`RelationReceipt\` and \`SolveRecord\` Rust schemas. KB MCP and Conductor were not exposed in this runtime; no novelty claim is made. One coordinator executed all changes and the three logical lead / Rust compatibility / validation packages, with zero native independent workers. No reserved compute or CI job was dispatched.

**Next:** obtain an actual natural RR5 producer record with a source-identity-matched public base manifest; run the two read-only checkers against that *same frozen receipt* on an admitted runner and reconcile every equation/rank/recovery discrepancy, then source-exact native Rust tests and matched fully charged T=1 cold baseline runs. Keep the PR in draft until these gates are satisfied.


**Frozen synthetic development replay artifact (09:35 UTC):** [development_fixture_replay_20261123.json](development_fixture_replay_20261123.json) is exact stdout from the committed Python checker operating on the **byte-identical** committed public fixture. Verified by \`git hash-object\`: fixture \`44a7437ea5cc74cd66638a0a7bdb47525833b593\`, checker \`346036d8e1df814f572e8dce1c079178dc77d9b1\`, test suite \`a3593ab19e790be02c36044d156012376988b8a4\`, stdout \`ac2cba738cd85d7b48459fd5fb137c876ce7d18a\`. The direct fixture's SHA-256 is \`d954db5502909259edbe075d7a5cb73ea5b811d2cafeb86b2d55ae857ba05862\`. The source-exact local command \`python3 independent_rr5_replay.py --receipt public_rr5_sample.json\` returned 2 authenticated rows, first target prefix 2 and target 5; \`python3 -m unittest -q test_independent_rr5_replay.py\` still passed 8/8. This **development record** is neither a natural blind RR5 extraction receipt nor signed PCC evidence. No current-performance, extraction-yield or security claim follows. [pcc:unverified]

## Genuine Grevlex B8 extracted-row boundary — 2026-11-23 UTC

A **real RR5 algebraically extracted signed row**, rather than another planted
rank-system example, is now stored as
[`rr5_grevlex_b8_producer_row.json`](rr5_grevlex_b8_producer_row.json).
It is a read-only conversion of the frozen
[RR5 development09 Grevlex B8 positive stdout](https://github.com/isogeny-labs/crypto/blob/95e5062573e24d124235756c062e2ab8178aac4c/research/prime31_rr5_public_relations_20261123/evidence/development-09/b8-positive.stdout)
(Git blob `c16fac3b6bd28a2c125f53e466eea2c97b84f95f`), published on
[RR5 PR #4970](https://github.com/isogeny-labs/crypto/pull/4970).
The fixed-positive target was **constructed from the factor base by the
producer's correctness test**, not drawn as a blind unknown-index ECDLP
challenge. This is not a native `SolveRecord`; the `a=0, b=1` labels are
the exact, explicit `R=Q` group identity of that correctness fixture,
not additional evidence that the producer generated an `aG+bQ` query.

Source's eight natural-abscissa points have indices 0–7. The one
nonenumeratively extracted four-leaf row is
`P[0] - P[1] + P[2] + P[3] = Q`, where
`Q=(92816593,1965281801)`. Two nonshared arithmetic checks of the
public identity (Python group law and JavaScript BigInt group law)
reconstructed that exact Q; a pinned-source transcript comparison also
matched the eight points, target and signed row. This is **mathematical
development replay**, not a compiled Rust run, signed independent PCC, or
blind scalar recovery. [pcc:unverified]

The row has augmented rank **1** and factor-base-only coefficient rank
**1** over n=2147446831; hence target-coordinate rank gap **0**,
no determined scalar and no earliest target prefix. There is nothing
legitimate for the gate to report as a solved ECDLP. The producer's frozen
B8 traversal reports seven F4 runs and 54,099,486 F4 field operations to
obtain this one row; these are *partial stage counters*, not an
end-to-end timing, a representative per-relation rate or an IC/rho ratio.
The producer reports `Complete` for its **fixed B8 extraction only**;
B677 natural extraction remains open.

Both standalone checkers now include source-coupled regression tests:
`test_independent_rr5_replay.py::test_extracted_grevlex_b8_row_stays_underdetermined`
(also poisons the second signed leaf), and
`rr5_receipt_check.rs::tests::real_extracted_grevlex_b8_public_row_has_rank_but_no_target_scalar`
(also checks the poisoned row fails). The Python suite has **9 authored
tests** and the mixed-RHS Rust binary **12 authored tests**, plus
15 from the original separate Rust gate. These **new tests have not been
executed in this session**, and the Rust compilation gap is unchanged.

For future source-frozen replay, the intended commands are:

```sh
python3 -m unittest discover -s research/cheap_ecdlp_cold_gate_20261123 -p 'test_independent_rr5_replay.py' -v
python3 research/cheap_ecdlp_cold_gate_20261123/independent_rr5_replay.py --receipt research/cheap_ecdlp_cold_gate_20261123/rr5_grevlex_b8_producer_row.json
cargo test --release --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml --bin rr5_receipt_check
cargo run --release --manifest-path research/cheap_ecdlp_cold_gate_20261123/Cargo.toml --bin rr5_receipt_check -- research/cheap_ecdlp_cold_gate_20261123/rr5_grevlex_b8_producer_row.json
```

**Admission next:** source-freeze and independently replay a natural
unknown-index `SolveRecord` with an exact point-only B677 manifest;
for the candidate to recover d, its independently authenticated rows
must first give `rank([C|-b])-rank(C)=1`. No shortcut from one
fixed-positive B8 row can satisfy this. Preserve all refused chart,
failed-root and bounded-inconclusive cases; charge all extraction, setup,
modular algebra, verification and timeout costs before any matched
cold-one-target benchmark.

**Roles and constraints:** no native agent launcher was exposed. The
coordinator performed the GitHub source audit, read-only producer-row
conversion and all writes. The baseline/relation-source and test-design
reads were independent parallel tool calls but **not separately spawned
workers**. Python and JS group calculations are separate mathematical
replays, **not independently signed validation**. No competing build,
paid compute, reserved runner, merge, or speed claim was made.

## Live baseline-head reconciliation (2026-11-23)

A fresh [#4956](https://github.com/isogeny-labs/crypto/pull/4956)
read finds this PR **merged**, but its single changed file was
`docs/ACTIVE.md`. That tracking-only merge did **not** bring a new
`prime_fast` or specialized batched complete-cold reference, an accepted
Wiener–Zuccherato negation-rho implementation, or a new matched-cost
receipt into main. The reference-selection dependency remains open and
must be satisfied against actual executable source rather than a PR's
merged label. Separately [#4901](https://github.com/isogeny-labs/crypto/pull/4901)
is merged and has 109 changed files, including the retained Run72/75
**generic** negative controls. Those do not become nonenumerative
measurements merely by landing in main. The distinct genuine RR5 B8
fixed-positive witness above likewise does not establish a cold-cost
comparison. These are repository-status observations, not new timings or
performance conclusions. [pcc:unverified]

## 2026-11-23 11:23 UTC — fail-closed natural-record denominator admission

The [independent Python RR5 replay](independent_rr5_replay.py) at new commits
[`3aead7b`](https://github.com/isogeny-labs/crypto/commit/3aead7b91df7895e58c8f5b36ee00e6f772c27ae) and
[`01f3bc2`](https://github.com/isogeny-labs/crypto/commit/01f3bc2f84eb47cfc7dcf53d66072ef5c0f3fbbf)
has two deliberately narrow hardening changes, without changing RR5 production or the Rust relation engine:

1. **Factor-base contract parity:** the callable replay function now rejects an
   identity/infinity factor-base leaf and noncanonical/off-curve points even
   when invoked outside its CLI's strict point parser; a legitimate identity
   **target** remains allowed. This matches the original Rust adapter's
   nonidentity base contract and prevents an invalid zero point from being
   mistaken for a useful linear-algebra variable.
2. **Preserve discarded extraction work:** a producer `SolveRecord` report now
   explicitly distinguishes `extracted_candidate_rows`, `accepted_rows`,
   `extracted_but_not_accepted_rows` and
   `attempts_without_accepted_rows`. Receipt count exceeding recorded
   extraction count is rejected. Producer `solved`/`solved_after_deadline`
   status must agree with independently recovered, point-replayed scalar.
   These fields are **reported producer counts**, not verified F4 timings
   or proof that extraction was algebraic. Every row used for rank still
   passes the existing independent public EC equality and canonical
   coefficient checks.

[The test suite](test_independent_rr5_replay.py) has **11 authored tests**,
including a forged zero-admission, three-extraction-row `SolveRecord` that
must report rank zero and all three extracted rows unaccepted, a spoofed
`solved` status that must fail, and identity/off-curve base poison tests.
The two new tests were **not executed in this run**. They must be executed
against the exact new Git blobs in Python, followed by native Rust replay on
the same frozen records; none of this qualifies as PCC validation.
`[pcc:unverified]`

The useful mathematical target-recovery certificate remains a left-nullspace
witness: for authenticated rows `C L - b d = a` modulo prime `n`,
a vector `u` with `u^T C=0` and `u^T b != 0` gives
`d = -(u^T a)/(u^T b) mod n`. Such a witness is a proposed compact
cross-check for future genuine B677 record replay, **not** an additional
implemented extraction method or measured cost saving. If no accepted rows
yield such a witness, the target is not recovered regardless of the number
of F4 candidates reported. This scope is disjoint from the RR5 producer,
graph/sparse-LA, S3, and transfer lanes.

**CI triage at prior frozen PR head `4d4d796`:** the
[pcc-gate run 45155651851](https://github.com/isogeny-labs/crypto/actions/runs/45155651851)
failed three `NameError` tests (`reasons` twice, `q` once), also observed
in RR5 #4974. The exact `tools/pcc/test_pcc.py` Git blob
`47be6b2e761d0ca77423be7351e94ac8a6aebbbe` is identical on main and
this branch; no shared PCC-file repair was attempted. A separate
[validation-gate run 45155651946](https://github.com/isogeny-labs/crypto/actions/runs/45155651946)
failed while pulling approved artifact-store inputs absent from its local
Git-history cache; the log explicitly says S3 was not contacted. Neither
failure certifies or disproves this isolated Python change, nor is either a
substitute for exact-source Rust compilation. No new runner/job was launched.

**Next admitted steps:** (i) run the 11 source-frozen Python tests; (ii) run
the original-arithmetic Rust tests and independent CLI fixture comparison
on an authorized runner after its build reservation clears; (iii) ingest
a complete unknown-index B677 `SolveRecord` and separately attest every
natural extraction, accepted row, rejected row, rank increment and target
recovery; (iv) only then collect matched complete cold T=1 comparisons.
No signed PCC standing, native Rust execution, blind Prime31 target solve
or measured advantage is established by this change. `[pcc:unverified]`

## 2026-11-23 — extraction-to-acceptance *provenance* gate

A further adversarial gap was found in the previous Python `SolveRecord`
reader: verifying a receipt's **true group equation** and checking that the
*total* number of accepted rows does not exceed extracted candidates does
**not** establish that any accepted row actually came from the corresponding
attempt's extractor output. A true signed row from a different run could
otherwise be spliced into a reported attempt, improperly advancing useful
rank while falsely charging the reported F4 work. This is an input-integrity
issue, not a faster ECDLP solver. `[pcc:unverified]`

At the frozen producer source
[solver.rs at 95e5062](https://github.com/isogeny-labs/crypto/blob/95e5062573e24d124235756c062e2ab8178aac4c/research/prime31_rr5_public_relations_20261123/native/solver.rs),
the producer iterates `extraction.relations` in order and sends each
candidate to the
[relation adapter](https://github.com/isogeny-labs/crypto/blob/95e5062573e24d124235756c062e2ab8178aac4c/research/prime31_four_summand_20261122/native/relation_adapter.rs).
The latter combines repeated signed indices, removes zero sums, sorts indices,
and—if the first surviving term is negative—negates *all* coefficients
and *both* public RHS scalars modulo `n`. It stops acceptance on deadline,
an error, or a recovered answer. Consequently, for each reported attempt,
its accepted receipts must be the **exact canonicalized prefix** of its
reported extracted candidates, with the attempt's unchanged public
`a,b` (modulo that permitted global sign flip). Checking only row truth or
aggregate extraction counts is insufficient.

The read-only [Python source](independent_rr5_replay.py) now reconstructs
that canonicalization and checks the ordered candidate/receipt pairing
before authenticating any row or admitting rank. Extracted-but-unaccepted
candidates still count toward the recorded denominator, but not rank.
The implementation is isolated to this PR's existing validation directory,
not the Rust extractor, graph/S3 solvers, or frozen RR5 output.

New source commits:
[admission logic `8323a15`](https://github.com/isogeny-labs/crypto/commit/8323a15ba68bd0284e2824ed2784020edb571c3b)
and [two authored regression cases `0339450`](https://github.com/isogeny-labs/crypto/commit/0339450f8bd74459b9021de3984d103d5b8c937c).
The authored Python test-suite count is **13**, with a true two-attempt
public solve control, accepted-row substitution, skipped-candidate and
changed-public-query poison cases, plus exact signed-leaf normalisation.
The **new repository tests have not been executed**; no native Rust test
or natural B677 replay was run. The coordinator separately ran seven
small *local prototype-only* canonical-prefix checks with Python 3.13.5
(`python /mnt/data/prime31_admission/check_provenance.py`, 7/7 pass,
SHA-256 `74911178daeddb57b2ec7b1159434987951760a883b75bce23ecb8a1d632d351`).
That prototype is not the repository implementation, a source-exact
test receipt, a signed independent validator, or a new claim of execution
standing. `[pcc:unverified]`

**Remaining evidence boundary:** a matching candidate/receipt prefix
proves *record-internal consistency only*. It does not prove that the reported
F4 algorithm ran, that its transcript was not forged, or that all wall time,
failures and base construction were charged. Require source-frozen extractor
execution custody, an independently authenticated candidate stream and
both original-Rust/Python replay, then complete-cold matched baselines.

**Worker/ownership record:** no native worker agent launcher was exposed.
The coordinator performed three disjoint logical work packages: (1) checked
prior APS/BSGS/Run72/75 negative controls and live #4970/#4974/#4972 ownership
without choosing cross-cohort baselines; (2) inspected the original Rust
`SolveRecord` and `RelationAdapter` source contracts and changed only this
Python consumer; (3) designed the candidate-link poison controls and the
separate, non-source-exact seven-case prototype. There were no spawned
parallel workers, root builds, timed experiments, external spend, PR merges,
or elapsed-time speed claims. `[pcc:unverified]`

**Next:** execute the 13 exact-source Python cases and original Rust 15+12
tests on an unreserved authorized runner; preserve the first genuine
unknown-index Prime31 RR5 record with its full extraction/attempt stream;
then check actual rank, independent target replay and all-cost comparisons.
The [B677 owner #4974](https://github.com/isogeny-labs/crypto/pull/4974)
still has not supplied admitted accepted natural B677 relations.


## Source-exact public-query schedule reproducibility gate (2026-11-23, follow-up)

The native producer at [RR5 #4970 source `95e5062`](https://github.com/isogeny-labs/crypto/blob/95e5062573e24d124235756c062e2ab8178aac4c/research/prime31_rr5_public_relations_20261123/native/solver.rs) uses unsigned 64-bit SplitMix64, advancing **twice per public query** to produce `(a,b)` modulo the ordinary curve's prime subgroup order. This branch now adds an **opt-in** read-only sequence audit to [`independent_rr5_replay.py`](independent_rr5_replay.py), requiring a separately supplied seed and maximum number of queries:

```sh
python3 research/cheap_ecdlp_cold_gate_20261123/independent_rr5_replay.py \
  --record frozen-solve-record.json --base frozen-public-base.json \
  --seed 42 --max-queries 2
```

Only use the example seed/cap for a record actually produced with that seed and cap. The checker now reproduces every public RHS label in order, rejects skipped or reordered public queries or changed `a,b`, refuses attempts beyond the declared cap, and requires exactly the declared number of attempts for `query_limit`. A `deadline` status may legitimately retain a shorter **prefix**, not an arbitrary subsequence. Its output distinguishes `query_schedule_checked`, `source_exact_scheduled_queries`, and `declared_max_queries` from genuine relation/rank admission. The new audit never sees a private target scalar; it reuses the point, group, signed-extractor-prefix, authenticated-row and rank checks already present.

An independently written BigInt calculation of the native arithmetic constants gave seed 42's first four reduced words `[49979948, 1033465976, 1897459893, 556465603]`. These are a **development arithmetic cross-check**, not a Rust/Python source-exact execution or PCC verdict. The Python source has one newly authored test method with seeded normal, mutated label, reversed attempts, omitted query, deadline prefix and missing paired flag controls; the suite is now **14 authored tests, not executed in this environment**. No new Rust tests, builds, timed runs or extra F4 queries were made. [pcc:unverified]

This closes one *record-level schedule consistency* gap only if the seed and cap have independently frozen provenance. The producer does **not** serialize its seed, the audit does not independently attest that the natural F4 run happened, and the producer's `complete_cost_accounting=false` remains unchanged. It does not verify wall time, resource usage, hidden skipped compute before seeding, seed selection after observing results, factor-base generation or unreported external attempts. The native B677 unknown-index extractor/target-span/cold-cost experiment is still owned by #4974/#4970 and remains unfinished; no gain or asymptotic change is claimed. Preserve the fixed B8 true row's **rank-gap zero** outcome.

To qualify a future blind B677 run, precommit the public target derivation, exact source/toolchain hashes, seed, max-query cap, factor-base manifest, all limits and complete attempt transcript **before** the measurement; replay both original Rust and this consumer after freezing the record, then match cold T=1 fully charged timings against best maintained IC, source-exact WZ negation rho and fresh BSGS with reservations respected. Do not use this optional seed audit to relabel prior records whose seed was not frozen.


## 2026-11-23 14:30 UTC — auditable dual target-separation certificate (opt-in)

The isolated independent public-RR5 replay now offers `--emit-dual-certificate` in either direct-receipt or producer-record mode, with default replay unchanged. For each **already authenticated** accepted row `r_i=[C_i|-b_i|a_i]` modulo the ordinary Prime31 prime order `n=2147446831`, the target is uniquely determined when sparse weights `w_i` produce

```
sum_i w_i r_i = [0, ..., 0 | 1 | d]  (mod n),
[d]G = Q.
```

This produces an independently checkable separating *dual* certificate: a verifier needs only the public accepted rows, weights, field arithmetic and original EC scalar replay; it does **not** need the factor-base logarithms or to trust the producer's rank label. The new helper `verify_dual_certificate(originals, weights, scalar, base_size)` separately checks canonical row references, nonzero weights, zero base-column residual, unit target-column residual and exact scalar RHS. Provenance is carried through the existing base-column elimination on demand, storing source rows only when admitted as a base pivot or used in the first target certificate. Memory is bounded by an explicit opt-in `B<=1024` gate; this is **not** a root extractor or a cost/speed optimization and adds unmeasured work.

- Source: [independent_rr5_replay.py](independent_rr5_replay.py) commit `b48caa0d67283201cf1a3b2e8f4f9b9878ea10d6`, Git blob `1b2de4e4c247d666f5b5f2869226465f9307b3fd`; [authored test suite](test_independent_rr5_replay.py) commit `eaf82a39ed7014cfbabff1bf14b79ad39df3e3b4`, Git blob `3c17d7b01bcef0099e1f475252cb0dcb7a7e3b5c`. **15 authored Python test methods, not executed source-exact** in this runtime. The new case checks honest fixture weights, poisoned weights/indices/scalar, delayed first-target prefix, and identity preflight; the authentic fixed-positive RR5 B8 relation remains underdetermined and emits null rather than inventing a dual certificate. No original-Rust code was modified.
- The synthetic public fixture's two equations yield the explicit signed combination `w=[(row 1,1),(row 2,n-1)]`, base residual zero, target coefficient 1 and `d=5`; this is **a constructed correctness fixture, not an unknown-index ECDLP solve**. For the genuinely extracted B8 row, the unchanged rank gap is zero, so no separating witness exists from that single row.
- A fresh **separate local mathematical reference**, not the GitHub Python module, used Python 3.13.5 and seed `0xCE31D2`: **2,400** consistent random linear streams over `F_101`, variable widths 1–8 and up to 16 submitted rows, checked earliest target visibility against independently recomputed prefix rank; **1,948** had a target dual witness, all produced `[0,...,0,1,d]`, and max witness support was 9. This is proof-of-concept arithmetic consistency only, **not** source-exact Python execution, Rust testing, signed PCC, Prime31 blind extraction or runtime validation. [pcc:unverified]
- Prior art/reuse: [HEC target-only dual certificate](../hec_prefix_operator_20261118/TARGET-CERTIFICATE.md) already derives the exact left-nullspace criterion in a separate genus-4 lane. This change implements an independent **Prime31 RR5 receipt-checking interface** and claims neither a new linear algebra theorem nor a new asymptotic ECDLP algorithm. The checked repo/related-repo code searches found the preceding certificate, HEC and graph/quotient work; research-KB MCP/CLI was unavailable here.
- Source/ownership: preserved #4969 draft; RR5 public extraction remains #4970 at `95e5062573e24d124235756c062e2ab8178aac4c`, B677 lane #4974 at `6aad5ef2c9c5ce5192ed847b30bb8fa1df4a63a2`, graph #4972 at `7bd2e679680c45d16375da2d2b482c3ef5772ae3`. No neighbor source, frozen evidence, baseline, paid resource, workflow launch, timed measurement or merge was touched.

**Next acceptance gate:** on admitted noncompeting compute, execute the exact 15 Python tests and the original Rust 15+12 tests at frozen source/toolchain/lock; use `--emit-dual-certificate` on a genuinely extracted, unknown-index B677 SolveRecord, recheck the weights in an independent original-Rust process, and measure completely priced cold T=1 against strongest maintained index calculus, WZ/optimized negation rho and BSGS with all failures/attempts/setup charged. Keep the unverified/nonwinning baseline rows unchanged.
