# EXP-R31-MIXED4+4-IC — prospective complete Rust cross-base relation collector

**Frozen before implementation or measurement, 2026-11-17 UTC. [pcc:unverified]**

## Motivation and assumptions

Use the existing `prime31-algebraic-ic` native Rust package, `PRIME31`, `Arithmetic`, and `Base::build(PRIME31,6)`, not a Python or replacement curve implementation. Curve: `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831` prime, `h=1`, `G=(625037715,2100627251)`. Compare the actual **first 31 rational-x** and **31-point Dickson-depth-six** factor bases. Prior original-curve half-sum studies #4391/#4451 reported equal four-sum image cardinalities (618,885) but different 8-leaf coverage; two public Q failed on Dickson and passed on first-x. Both are producer evidence only.

**New question:** does a shared *mixed* first-x four-sum and Dickson four-sum allow reliable **fully complete** modular relation collection/rank recovery, with one table per base, without requiring that either homogeneous eight-leaf base represents a particular residual? This is an **index-calculus-derived**, generic 4+4 meet-in-the-middle relation collector; it is NOT a new non-generic algebraic extractor, and cannot substitute for the target-conditioned RR9/S9 objective.

For each public residual `R=aG+bQ` and found `R=sum_i u_i A_i + sum_j v_j D_j` (each half a signed four-leaf word, with cancellation allowed), create the verified equation

`sum_i u_i log_G(A_i) + sum_j v_j log_G(D_j) - b log_G(Q) = a (mod n)`.

The two bases may overlap! **Deduplicate identical affine points across bases** before assigning unknown columns; do not assume rank 63 from 31+31 basis positions. Cross-base exact overlap is checked at run time. Collect rows into incremental modular RREF and solve when rank = number of distinct basis points + one target. Reject any inconsistent dependent row. Recheck each individual recovered factor-base log and the final target log against original `[k]G=Q`. The actual `k` used by the caller to make Q is never passed to the solver; it is solely a post-solve auditor input.

## Pre-registered experiment and stop gates

* Exactly two fresh caller-only target scalars from SplitMix64 seed `0x4d495834_50333131`; first target is the primary. Use same algorithm and frozen constants for both, one full **cold** table build per target (no free sharing).
* For each target, enumerate all 677,040 four-token multisets on each base, deduplicate signed *integer net* signatures (expected 618,885 per base), construct point->one verified signature and ordered point key list. Preserve collisions and do not infer algebraic uniqueness if collisions appear. Signed tokens are in original curve E(Fp).
* For each relation attempt draw `a in [1,n-1]`, `b in [1,n-1]` from a fixed separate SplitMix64 stream, form the public residual with the existing Rust group arithmetic, and cycle over first-base half-list from a randomized offset until the complementary Dickson half point is found. Each probe, failed attempt, duplicate/dependent row, and equality check is charged. At most **160** attempts per target; stop when rank is full or cap is reached. A miss is reported as incomplete, never discarded.
* Each collected signed relation is replayed in original curve and its integer vector recorded as a compact SHA-independent textual digest (full reconstructed row and signed signatures are retained for the first and final relations). For each row, verify `sum leaves = aG+bQ`. If the table contains any half-sum point collision, preserve its first witness and record collision count.
* Fully cold `Instant` timings and group-operation counters include curve checks, base build, both four-sum tables, all probes, relation acquisition, rank/linear algebra and final scalar recovery. Freeze method/source SHA in CI and preserve raw JSON. Report any timeout or cap, including full spent work. Do not compare unrelated GitHub-hosted totals as if matched same-host measurements.
* Development correctness gate: both Q must be recovered, all basis logs replay, full row rank and every relation validated. Speed observation is **descriptive only**; the prior APS fully cold 24Q result was ~81ms total (~3.4ms/Q) but not a same-run, same-host arm. A credible gain later needs ≥96 fresh unknown Q, matching strongest tuned Wiener–Zuccherato rho, APS and published external IC on isolated hardware with paired intervals and independently signed PCC. This panel has no official speedup acceptance gate because no matched comparator is scheduled.

## Reuse, prior leads, and intentional scope

Read `AGENTS.md`, `docs/ACTIVE.md`, dated shared session logs, `APS_LINEAR_CHORD_24Q_RESULT_20261117.md`, `S9_B31_MULTIPLICITY_PROTOCOL_20261116.md`, `S9_B31_CROSS_BASE_REPLAY_AND_MULTIPLICITY_HANDOFF_20261116.md`, `RR9_NORM_FACTOR_PROTOCOL_20261117.md`. Relevant merged work: crypto #4391/#4451/#4454/#4466/#4486; rootfinder #4228/#4349; censored F4 #4373; generic/frontier #4108/#4363/#4468; crypto-autoresearcher #3301/#3305; cryptanalysis #1536. Literature: Amadori–Pintore–Sala, FFA 2018 DOI 10.1016/j.ffa.2018.01.009; Petit–Kosters–Messeng, PKC 2016 DOI 10.1007/978-3-662-49387-8_1; and classical Shanks baby-step/giant-step. These establish that the point-table method is generic. Research-KB MCP / repository wrapper and Conductor are unavailable in the current connected runtime; fallback is pinned GitHub sources and published references, not a novelty clearance.

**Highest-priority separate scientific lead remains:** target-conditioned RR9/S9 signed-norm coefficient extraction from public R with original-ideal equality, *without* half-table or support enumeration; blinded B4/B8, Dickson negative controls, then B31 full unknown-Q rank with total cost. Unchanged scratch-array and linear-chord variants failed registered improvements and must not be rerun to cherry-pick results.

## Verification state

No new Rust compilation or ECDLP timing exists at protocol freeze. All prospective claims are [pcc:unverified]; producer tests are not a signed PCC verdict. Do not edit IC leaderboard or mark a new fastest algorithm from this experiment.
