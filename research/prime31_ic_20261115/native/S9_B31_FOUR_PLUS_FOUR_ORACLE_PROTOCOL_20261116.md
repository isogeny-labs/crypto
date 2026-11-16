# Prime31 S9 B31 oracle protocol: exact 4+4 signed meet-in-the-middle

Registered 2026-11-16, before the B31 oracle source and its CI results. Development evidence only [pcc:unverified].

## Scope and problem

On the existing ordinary prime-order curve `PRIME31` (`p=2^31-1, n=2147446831`), determine whether a public target Q is a signed sum of **exactly eight** factor-base leaves among the first B rational x-coordinates, allowing repetitions, mixed signs, cancellations and O. At B31 the exact **integer** net-signature ceiling is 5,654,193,417, exceeding n: it is *not* a coverage guarantee. The B8 exact-image census and natural-positive impossibility calculation are in `S9_SIGNED_SUPPORT_CEILING_AND_B8_GATE_20261116.md`.

This trial constructs an **independent generic group-law oracle** against which any later target-conditioned S9/K–S5 algebraic root extractor can be checked. It is **not** itself the non-generic extractor, does not compute unknown factor-base logarithms, and cannot establish a cheaper complete ECDLP attack.

## Prior sources and classification

1. `research/prime31_ic_20261115/native/S9_SIGNED_SUPPORT_CEILING_AND_B8_GATE_20261116.md`: exact signed integer signature counts, B4/B8 image census, golden representatives.
2. `research/prime31_ic_20261115/native/APS_RHO_FOLD_SENSITIVITY_48Q_RESULT_20261116.md`: the 48-Q APS 109.734280 ms and unfolded rho64 99.473873 ms producer-level fully cold controls; neither establishes a strongest-rho win.
3. `research/prime31_ic_20261115/native/RHO_JUMP_WIDTH_PROTOCOL_20261116.md`: independent 16/32/64-jump generic comparator, with source already committed; no result adopted without exact-run receipt.
4. `isogeny-labs/crypto` PR #4228 (selected-root quotient) and #4349 (actual-root APIs, native certified F4), which do **not** yet supply a completed B31 nonenumerative S9 factor polynomial extractor.
5. `isogeny-labs/crypto-autoresearcher/experiments/semaev-alternatives-20261105/SUMMARY.md` and `ERRATA.md`: avoid unchanged Vieta-conic, F4 tower and genus-2 negative/inconclusive leads.
6. Published APS single-relation index calculus (Amadori–Pintore–Sala, Finite Fields Appl. 51 (2018), DOI 10.1016/j.ffa.2018.01.009); meet-in-the-middle decomposition is a generic baseline, not the algebraic advantage sought.

Research-KB MCP and wrapper, conductor, and a local rustc/cargo are not available in this GitHub connector execution. This bounded GitHub fallback is **not** a novelty-by-absence search.

## Exact algorithm and prespecified controls

Let `A={P_1,-P_1,...,P_B,-P_B}`, choosing smaller-y representatives at the first B rational abscissae. Build the **complete four-leaf sumset** `4A` by nondecreasing four-token words. There are `C(2B+3,4)` candidate words, **677,040 at B31**; the **integer** net-signature count at B31 is **618,885**, before group collisions. Retain one literal four-word witness per distinct group point in a hash index, plus a deterministic insertion-ordered search vector. For each public Q, scan `U in 4A`, query `Q-U` in the index, and independently re-add all eight returned leaves. If the exhaustive scan fails, report a genuine absence from `8A`; no probability-based replacement for the decision.

- Use only the existing Rust native `Arithmetic`, `PRIME31`, `check_curve`, and scalar point arithmetic. Do not substitute Python or a new EC library.
- Assert `4A` integer-signature cardinalities B4=225, B8=2945, B31=618885; enumerate B4/B8 and test actual image sizes against their known injective eight-sum images.
- Fixed fresh caller-only SplitMix64 target seeds: `0x53394233314e4154` for **24 distinct natural** nonzero `Q=[k]G`, and `0x5339423331504c54` for **16 constructed-positive** eight-token sums; the witnesses are not passed into the lookup routine. Include an identity/cancellation control separately.
- Every positive output must provide eight concrete signed leaves and pass original-curve sum replay; every negative is certified by exhausting the exact four-sum list. Keep the per-target probe count and elapsed time, all source construction costs, hash size and collision counts. Record exact target/scalar seeds for independent reproduction.
- No guarantee of 24/24 natural positives: `S(31,8)>n` does not prove full reachability.
- Charge **all** base construction, pair generation, 677,040 four-word enumerations, hash work, probing and verification. Report cold time and separate amortized stage times, but do not mix them with existing separate-host APS/rho timings; there is no matched ECDLP solver arm here.
- Keep the oracle **out of the non-generic or IC leaderboard**. A 4+4 hash is generic MITM. A later algebraic S9 candidate must agree with this reference while discovering f from Q *without* scanning the 4A table or enumerating the 48,903,492 degree-eight f candidates.

## Decision gates

1. Pinned Rust 1.100.0 unit tests and B31 oracle full run finish in exact-source CI, all positive eight-leaf relations independently replay.
2. Measure natural B31 eight-sum hit rate and true source/setup versus query costs, rather than extrapolating B8 or the Poisson random-map heuristic.
3. Given-f verifier and target-conditioned algebraic extractor must agree on blinded B4/B8 plus natural B31 Q; only then attempt rank-complete factor-base logarithm recovery and full cold single-target ECDLP comparison against tuned WZ rho and mature external IC methods.

All outcomes will remain producer-only [pcc:unverified] until independent signed validation. No paid compute, history rewriting, merge or leaderboard promotion is part of this experiment.


## Pre-execution amendment: match the actual algebraic S9 factor base

**2026-11-16, before obtaining any B31 oracle result.** Cross-reading `research/prime31_ic_20261115/native/M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md` revealed that the actual balanced S9/K–S5 algebraic path uses the *depth-six Dickson fibre's 31 liftable abscissae*, not the first 31 rational abscissae selected above. Running only the first-B oracle would create **incompatible ground truth** for the S9 worker.

Retain all original first-B31 cases, and **add a parallel Dickson31 panel** using `Base::build(PRIME31,6,...).points`, 31 canonical actual rational points, with precisely the same 24 natural scalar seeds, 16 planted-token seed, exhaustive 4+4 algorithm, identity test, original-point replay, work counters and correctness gates. The two bases have separate table-build costs, `4A` cardinalities, collision reports and per-Q results; do not merge the hit rates or select the better base. The native Dickson31 oracle is the relevant reference for the algebraic K961/S5 elimination. Neither branch has been measured at the time of the amendment and neither is a non-generic solver.

This amendment also rules out treating coverage of an arbitrary first-31 base as evidence that the special Dickson f exists or vice versa.
