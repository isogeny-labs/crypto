# Prime31 RR9/B8 exact randomized-residual feasibility gate

**17 November 2026.** Artifact type: exact mathematical accounting plus native Rust regression. **Not** a new solver-speed observation, relation extractor, or complete ECDLP algorithm. Original prime31 curve `p=2147483647`, `E: y^2=x^3+847514846*x+2087444724`, prime cofactor-one `n=2147446831`, `G=(625037715,2100627251)`. Source-native Rust pinned in [protocol](RR9_B8_COST_GATE_PROTOCOL_20261117.md) and [example](../native/examples/prime31_rr9_b8_cost_gate.rs). All nonmathematical evidence producer-only `[pcc:unverified]`.

## 1. Exact theorem: fixed B8 supports and randomized target shifts

For the first eight rational x-coordinates [0,1,2,3,5,6,8,9], the previously [verified original-curve Rust 4+4 enumeration, #4575](https://github.com/isogeny-labs/crypto/pull/4575) found **256 distinct** group sums of the eight individually signed base points. This is the complete support set `S` of the *fixed eight-distinct-coordinate branch*, not all eight-point B31 decompositions. The new native test directly regenerates these points with unchanged `Arithmetic::add/neg` and fails on any collision. Original curve parameters and cardinality are frozen; CI replay status belongs to the source-specific job.

For any **fixed** ECDLP public target `Q`, independent uniform `b∈Z/nZ` gives `R=Q+[b]G` uniform over `E(F_p)` exactly because `G` generates the full prime-order group. Therefore

```
Pr[R in S] = |S| / n = 256 / 2,147,446,831
          ≈ 1.1921133333987537 × 10^(-7).

E[number of independent draws to first S-hit] = n / 256
          = 8,388,464.18359375.

E[draws to eight S-hits] = 8 n / 256
          = 67,107,713.46875.
```

The eight successes are **not asserted to have independent relation rows**. Any rank failure increases required trials. Adaptive, nonuniform shifts, a changing factor base, and target-informed algebraic choices are out of scope; this theorem is not a generic-group lower bound.

In the hypothetical case where **all** of the historical 1,024-Q shared-BSGS runtime `170.113 ms` (#4532) were devoted just to finding these hits, no setup/LA/recovery costs:

| Membership successes required | Expected B8 draws | Maximum *amortized* ns/draw within 170.113ms |
| --- | ---: | ---: |
| 1 | 8,388,464.1836 | **20.2794 ns** |
| 8 | 67,107,713.4688 | **2.5349 ns** |

These are feasibility ceilings under a *model*, not lower bounds for unrestricted ECDLP algorithms, and not new benchmark times. They make direct use of a costly fixed-B8 D9 membership test as the sole relation-finding oracle a particularly low-priority route for a complete p31 solver. #4604 remains separately owned for its narrower mathematical negative-certificate goal.

The earlier source-specific B8 4+4 generic control queried 1,024 independent blinded `Q` and returned **zero** supports; its setup/query timings were diagnostic, and its zero hits are fully consistent with the exact rarity, not an index-calculus failure. A uniform fixed-target test has expected hits of only `1024*256/n ≈ 0.00012207`.

## 2. Why the B31 experimental branch must not inherit this bound

Changing to **all squarefree eight-coordinate supports drawn from 31** gives a *formal* signed support count of `2^8*C(31,8)=2,019,513,600`; collisions may make the group image strictly smaller. Thus even its maximum possible coverage is `2,019,513,600/2,147,446,831≈0.9404254`, so at least `127,933,231` group elements are not reachable by that **squarefree** predicate.

The complete **repeated/cancellation-allowed eight-word** signature set is instead `D8` with `5,654,193,417` integer net signatures; its count exceeds `n`, so the same simple pigeonhole noncoverage claim does **not** apply. Existing native Rust [S9 B31 half-sum oracle](../native/examples/prime31_s9_b31_half_sum_oracle.rs) exact-controls the actual Dickson depth-6 base, not a hypothetical random base. It recorded **22/24** natural public Q positive and **2/24** complete exhaustive misses, each under 618,885 distinct four-sum complements (producer observation). [Cross-base/multiplicity audit](../native/S9_B31_CROSS_BASE_REPLAY_AND_MULTIPLICITY_HANDOFF_20261116.md). Neither these 24 targets nor Poisson occupancy are a population-coverage measurement.

This is the crucial **lead split**: the exact B8 rarity gate can retire B8-only end-to-end speculation, but it cannot retire real B31 multiplicity-aware target-conditioned algebraic extraction.

## 3. Nonduplication review and current full-solver frontier

| Existing lane | Evidence and disposition |
| --- | --- |
| [#4532](https://github.com/isogeny-labs/crypto/pull/4532) | Historical producer complete Rust **1,024-Q**: mixed IC **2,221.474ms**, shared BSGS **170.113ms**, 13.06× slower mixed IC. Not remeasured here |
| [#4556/#4558](https://github.com/isogeny-labs/crypto/pull/4558) and [#4575](https://github.com/isogeny-labs/crypto/pull/4575) | Exact B8 barycentric signed enumeration, generic-equivalent and slower than generic B8 4+4. Do not describe as nonenumerative IC |
| [#4548](https://github.com/isogeny-labs/crypto/pull/4548), [#4549](https://github.com/isogeny-labs/crypto/pull/4549), [#4550](https://github.com/isogeny-labs/crypto/pull/4550), [#4581](https://github.com/isogeny-labs/crypto/pull/4581) | RR9 D5/D6 generic Koszul ranks; projective D9 exact separator; split seven-equation quotient `F_p^128`. Exact statements but not a fast full solver |
| [#4604](https://github.com/isogeny-labs/crypto/pull/4604) | **Active separately owned** matrix-free D9 rank/negative-certificate gate. Avoid overlapping its branch, algorithm or CI |
| [#4596](https://github.com/isogeny-labs/crypto/pull/4596) | **Active separately owned** same-1,024-Q batched pair-table IC vs shared BSGS comparison on the actual curve. **CI #44148657581 failed in compilation** (missing sparse `include_str!` inputs and duplicate F4 fall-degree definitions); no performance JSON from that run. Precise blocker was commented in #4596 |
| [autoresearcher #3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301) and [#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) | 39-lead heatmap fold, generic-pair-table classification, Semaev alternatives and errata. Retain proof / scale / evidence-status distinctions |
| [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) | Matched same/random/auxiliary factor-base controls; no transferable complete-p31 non-generic speed claim |

Original prime-field prior art: Petit–Kosters–Messeng, PKC 2016 [DOI 10.1007/978-3-662-49387-8_1](https://doi.org/10.1007/978-3-662-49387-8_1); Amadori–Pintore–Sala, *Finite Fields and Their Applications* 2018 [DOI 10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009). Sparse Wiedemann and classical generic meet-in-the-middle are established algorithms, not novel claims. Shared research KB unavailable in this tool runtime; bounded GitHub and primary-publication review cannot establish absence of prior art.

## 4. Prioritized falsifiable next experiments (do not duplicate owned sessions)

**P0 — finish the frozen complete-cost comparator:** On owned #4596, repair only checkout/compilation (not the frozen seed, cap, base or targets), record complete unknown-log success and exact wall/ops/memory; compare identical 1,024 public Q against all three cached BSGS arms and a tuned native negation rho. Preserve failed runs. No substitute synthetic or different p allowed.

**P1 — actual non-generic candidate, Dickson31:** Use existing Rust `M8_RATIONAL_QUOTIENT_BRIDGE_20261116.md`, #4228/#4349 root/quotient code and certified F4 on the **same special B31 base**. Candidate receives `Q` and public base only; no `4A` group table, no 618,885 generic-complement enumeration, no caller scalar, no planted factorization. Independently replay exact rational `f`, root multiplicities, y/sign/orientation and group relation against fixed 22/24 positive, two exact negative, and fresh seeded 1,024 unknown-target controls; charge every failed attempt.

**P2 — relation rank and individual logarithm recovery:** Once P1 produces reproducible verified relations, use native Rust modular LA to solve base logs and each public unknown `k`, handle adaptive `R=[a]G+[b]Q` for misses without using hidden k, and verify every `[k]G=Q`. Compare complete costs against #4596 and fastest available external prime-field index calculus, not only rho. Require signed PCC before leaderboard promotion.

**P3 — structural lead veto:** For any proposed x/y hybrid, only advance if it reduces the *complete* algebraic relation-extraction cost and avoids an implicit signed-word group enumeration; reject experiments whose only improvement is fewer chart coefficients while retaining `2^B` sign choices. See [#4547](https://github.com/isogeny-labs/crypto/pull/4547) and [#4581](https://github.com/isogeny-labs/crypto/pull/4581) for explicit generic-equivalence witnesses.

## 5. Verification and limits

Code and prospective protocol are committed in PR #4612. The exact-source native Rust CI passed 40/40 library and 1/1 new example tests, and the bounded report assertions passed; details below. Python integer arithmetic independently reproduced the stated counts and rational expectations; Rust source executes native arithmetic if source-specific Actions passes. **No new complete ECDLP timing, no new relation-extraction speed measurement, and no novel non-generic solver have been demonstrated here.** All prior producer timings lack independent signed PCC in this record. No paid compute, merge, or leaderboard changes.

### Exact-source Rust verification (now complete)

[GitHub Actions run #44161786208](https://github.com/isogeny-labs/crypto/actions/runs/44161786208) on commit `b4bc74e684663110a31bdf2047789bf647cc3e40` **PASS**, pinned native Rust 1.100.0. Original native library tests **40/40**, proposed example tests **1/1**, release build completed, executable emitted JSON and all `jq` arithmetic/schema assertions passed. Source-specific [raw JSON ZIP artifact #14363726583](https://github.com/isogeny-labs/crypto/actions/runs/44161786208/artifacts/14363726583) uploaded (723-byte archive), GitHub-reported digest `sha256:e7a9adcaf068f12fc17948ea4fb44b95a98366f614ccc8bf3803d35430d67276`. **The archive digest was read from GitHub metadata, not independently rehashed.** No signed PCC or new full ECDLP timing is claimed.
