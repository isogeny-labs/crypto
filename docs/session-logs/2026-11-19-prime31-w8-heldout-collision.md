# Prime31 held-out signed-weight and EC collision audit — 2026-11-19 (PR #4807)

## 2026-11-19 12:18 UTC — What was done (commands, files, features)
- Inspected current `AGENTS.md`, `docs/ACTIVE.md`, #4692, #4728, #4805, #4803 and #4793 before selecting distinct task. Read existing #4805 Rust source, verified report, frozen JSON, session log, and original-source CI workflow. Existing W1 power-of-two halves each reported 65,496 vs formal maximum 65,536. Prior 256-target pilot 205 old vs 226 new is not held-out.
- Reviewed PKM 2016, APS 2018 and Kudo et al. prime-field index-calculus limitations; reviewed original GitHub previous directions listed in protocol. KB MCP/wrapper unavailable in connector runtime; bounded audit.
- Created feature branch, committed active-task line, opened draft PR #4807, linked PR in ACTIVE. Preregistered `W8_HELDOUT_COLLISION_PROTOCOL_20261119.md` with held-out seed `48454c444f555431`, 1024 public points, exact collision certificates, original pinned source and no generic/non-generic confusion.

## Decisions made (and why)
- Investigate **non-formal finite-group** collisions separately from pure-integer signature collisions and held-out hit frequency. Leave the actual RR9/F4 public-only extractor to #4692, avoiding duplicate candidate work.
- Reuse original Rust arithmetic; use generic 4+4 MITM **only as a grading oracle**. Do not promote whole-solver or ECDLP advantage from graded hits.

## Open threads (unfinished, pick up next time)
- Implement and run exact W1 collision witnesses + 1024 held-out natural public points in the original Rust crate on source-specific CI; independently replay an accident witness.
- Publish all actual counts, raw evidence, and next nonenumerative extractor gates.

## Errors (what went wrong, how it was handled)
- No local Rust toolchain or GitHub DNS in container. Source-specific GitHub Actions must run the new Rust example; distinguish lack of local test from CI success.

## 2026-11-19 12:27 UTC — What was done (commands, files, features)
- Source-specific [GitHub Actions #44523406912](https://github.com/isogeny-labs/crypto/actions/runs/44523406912) PASSED on pinned original Prime31 Rust 1.100.0; 40/40 original lib tests, 3/3 focused tests, 1024 held-out targets, jq outcome gates and independently authored affine/BigInt verification of 24 collision certificates. Raw [artifact #14528827855](https://github.com/isogeny-labs/crypto/actions/runs/44523406912/artifacts/14528827855) JSON SHA256 `b3b7dc977584825082ea2b4f48c7f138dbc47244627ad8a9a4541e6d806d05d1`. Source/workflow commit `f9dbd02cd0352bc3c8a7540ddc43dc573cf180af`.
- Frozen held-out paired 1024 public queries: old 772/1024 vs powers-of-two 902/1024 (676 both, 96 old-only, 226 candidate-only, 26 neither). Per-window old [195,194,189,194], new [228,223,229,222]. +130 matched hits / +12.6953 pp, descriptive paired exact p ~3.12e-13; **generic grading only**; no candidate algebraic extractor or complete solve.
- Independent locally authored Python affine census confirmed W1 each half 65,496 unique sums and exactly 40 non-formal colliding words. All 40 left-half collision pairs share integer coefficient difference `[0,8,1,8,6,-2,0,-1]`; independent EC arithmetic verifies its six-point group relation vanishes on base x [12,14,16,18,19,26] in W1. Right-half weights are exactly 16× left, so the identical 40 fibers are mathematically forced by invertibility of [16] on the odd-order group.

## Decisions made (and why)
- Record held-out **grade-yield** improvement, not complete-solver advantage. Precise structural result suggests a reusable single four-slot sumset S, with eight-slot representation R=U+[16]V for U,V in S. This may suggest shared polynomials for #4692, but a degree-256 multiplication map may offset any gain.
- Extend witness capture from first 12 to **all 40 per half** with independent replay, as a post-outcome validation amendment, preserving the first preregistered run as primary. No seed/outcome tuning.

## Open threads (unfinished, pick up next time)
- Commit exact-scale theorem, report and full certificate validation. Coordinate upstream #4692. Investigate collision-aware right weights [16,32,64,127] only on a separate prospectively frozen corpus, not the observed 1024 targets.

## Errors (what went wrong, how it was handled)
- No source-bound implementation failure; no local Rust. Tool-specific plain-text GitHub API source and CI log verification substituted for local build. [pcc:unverified].

## 2026-11-19 12:34 UTC — What was done (commands, files, features)
- Post-outcome strengthened exact-certificate source and independently authored Python script (all 40 in each W1 half, no seed change). Source-specific [CI #44524323141](https://github.com/isogeny-labs/crypto/actions/runs/44524323141) PASS at source `cc044eda1d854672891094ec0a691c92ca17605b`: 40/40 original Rust lib, 3/3 focused, 1,024 target grading and **80/80 independently recomputed short-relation collision certificates**. [Raw artifact #14529033204](https://github.com/isogeny-labs/crypto/actions/runs/44524323141/artifacts/14529033204), raw JSON SHA256 `3a297b765e3ada35961aa5683b1ac004e606d273ef057297149304996ab12794`.
- Checked in scientific result `W8_HELDOUT_COLLISION_RESULT_20261119.md` and frozen `W8_HELDOUT_COLLISION_SUMMARY_20261119.json`. The report covers exact scale-16 theorem, one six-point group relation explaining all 40 pair collisions, held-out 772/1024 vs 902/1024 generic grading, same-source test evidence, literature/previous leads, and public-only F4/complete-solver next gates.
- Full study original Rust code `native/examples/prime31_w8_heldout_collision.rs` and independent verifier `native/verify_prime31_w8_heldout_collision.py` committed. This remains an out-of-band MITM grader; no genuine public-only unknown-index extractor was implemented, no new rank31 whole-solve benchmark, no signed PCC.

## Decisions made (and why)
- Promote only the validated mathematical collision certificate and sampled **generic relation availability**; do not promote solver-speed or asymptotic claims. The new `S+[16]S` representation is a hypothesis to test in the existing #4692 algebraic extractor, with duplication-chain degree/memory charged.
- A separate candidate with last weight 127 removes the observed W1 right-half collision in independent affine exploratory enumeration, but needs fresh prospective registration before yield or F4 cost comparisons.

## Open threads (unfinished, pick up next time)
- Owner #4692: implement source-bound non-generic RR9/F4 or remainder-tower B8 extraction on public residuals, compare both weight arms, frozen 46+/18- corpus, then rank31/1,024 full cost vs same-runner fastest shared BSGS/IC/rho.
- Independent signed PCC and held-out cross-curve generalization; full eight-slot additive-energy/multiplicity study; optional near-powers right-weight candidate.

## Errors (what went wrong, how it was handled)
- GitHub runner jobs passed; no local Rust installation. Repository-global inherited artifact checks are unrelated and not claimed as passing. [pcc:unverified].
