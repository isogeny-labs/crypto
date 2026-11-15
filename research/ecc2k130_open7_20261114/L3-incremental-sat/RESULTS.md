# L3 results: incremental SAT across decomposition targets (ECC2K-130 family, toy n)

All numbers here are `[pcc:unverified]`: no pcc claim id exists, and they are a single-host,
single-producer measurement.

**Verdict under the frozen rule (C3, B vs A0, conflicts): NO-CHANGE.** The persistent incremental
solver costs 1.02x the conflicts of today's cold solve, 95% CI [0.93, 1.12]. It costs 2.23x the
propagations [2.04, 2.46].

## Protocol hashes
- `PROTOCOL.md` as frozen before any data: sha256 `2d1b64cdcf431d875f97bc5f6052790d4e304b21709180f07e806c2c7f4c5d32`.
- After Amendment 1 (C3 m=29 changed to m=23, C4 dropped, two processes): `613dcaed75036113dc24d0514c2a444a57e3a0a11f250fc453eb4a196dc8e339`.
- After Amendment 2 (C2 cut to 4 arms and 20 targets): `7c7a8ce6959eee1a6d63b6a34000e01de5d6ee6ba633dfbc99e08994bc1eacec`.
- Rules and predictions were not changed by either amendment (see `PROTOCOL.md`, appended sections).

## Prior-art audit (step 1)
What earlier work already settles, and what it leaves open:

| Item in the owner's claim | Status before this lane | Citation |
|---|---|---|
| Compile the factor-base part once | **Done** for the S3 chain: a target-independent template, instantiated per target with XORs, used by the SAT and Gröbner frontends | `crypto:research/polynomial_reuse_20260914/README.md`; `cryptanalysis:suite/src/cryptanalysis/polynomial_reuse.rs` |
| Keep learnt clauses across targets | **Not done for decomposition.** The same README says "SAT answers and learned clauses are not persisted by this implementation" | same |
| Incremental SAT with target bits as assumptions | **Done for a different formulation**: the bit-level Montgomery ladder over prime fields (FRESH vs REUSE, CaDiCaL). There REUSE/FRESH was 0.003–0.005 at n=4..7. Its own README offers "table exhaustion of a tiny group" as one reading, and the prescribed replication is pending | `crypto:research/sat_ladder_scaling_20261110/README.md`, `PROTOCOL.md` |
| Which solver the lab uses | goal22 harness: pysat `CryptoMinisat`, a fresh instance per target, R folded in as constants. Native suite: own CDCL in `sat.rs` with **no assumption interface**. `reset_search` plus blocking clauses only enumerates models *within* one target | `cryptanalysis:ecc2k130/runner/research/candidates/goal22/codegen/indexcalc.py` (`solveCnf`), `testdecomp.py`; `cryptanalysis:suite/src/cryptanalysis/sat.rs`; `cryptanalysis:suite/src/cryptanalysis/koblitz_index_calculus.rs` (enumeration loop near "Enumerate models incrementally") |
| Did any prior run keep solver state across targets? | No decomposition run was found that did. Only the ladder study did | grep of `crypto:research/` and `cryptanalysis:` for incremental, assumption, learnt and learned-clause |
| Related caution | Resuming a native solve after a budget `Unknown` was unsound until it was fixed. Learnt clauses are now kept across `solve()` calls in `sat.rs` | `crypto:research/pdp_ml_sat_ceiling_20261027/README.md` ("Side result") |

Also read: the `n83_chain_sat_*` studies (their "warmup" is meet-in-the-middle table building,
not SAT state), `wdsat_rr_20260920` (per-target WDSat and CMS panel), `ecbench_sat_pdp_20261009`
and `sat_pdp_pricing_band_20261111` (cold per-target oracle, priced). Not available in this
container: research-kb and the lead atlas. **This is not a novelty claim.**

**Residual tested:** on the S3-chain decomposition system with a weight factor base, does a
persistent incremental solver (target bits as assumptions, learnt clauses kept) save conflicts per
target compared with a cold solve? What share of retained learnt clauses is target-independent?

## Instrument
- Rust harness `tool/src/main.rs` (CaDiCaL 3.0.1 via `cadical-sys 0.8.0`, `factor=0`, otherwise defaults).
  Self-test passes: ONB arithmetic (two independent multipliers agree), curve order equal to the
  Lucas formula at m=5 and m=11, the CNF S3 circuit equal to field S3 at m=5..29, and S3 vanishing on (P, Q, P+Q).
- Every target is fully enumerated (all decompositions, then UNSAT). **In every arm, the model set
  for every target equals a brute-force ground truth: 0 mismatches over 812 scored (arm, target)
  rows**, and there were no censored solves. At m=11 the ground truth is itself cross-checked by
  two enumerations.
- Binary sha256: `l3_incsat` `98ba3b8f…0b54`, `analyze` `90c5c583…5b3c` (`raw/binary.sha256`).
- Wall time (a note only): 8,643 s of solver time in total, at load average 3–9 on 4 vCPU shared by seven lanes.

## Main table (paired geometric-mean ratio of conflicts per target, positions 1–39 or 1–19, 95% bootstrap CI)

| config | template vars | SAT targets | **B/A0** | AA/A0 (A/A) | A1/A0 (encoding) | B/A1 (pure reuse) | BS/B (A/A) | rule reading |
|---|---|---|---|---|---|---|---|---|
| C1 m=11 k=2 w=2 | 1,492 | 6/39 | 0.883 [0.838, 0.931] | 1.020 [0.990, 1.050] | 2.070 [1.945, 2.210] | 0.426 [0.394, 0.465] | 0.990 [0.944, 1.041] | NO-CHANGE |
| C2 m=11 k=3 w=2 (4 arms) | 2,952 | 10/19 | 0.904 [0.660, 1.235] | 0.977 [0.915, 1.046] | 4.085 [3.347, 5.034] | 0.221 [0.168, 0.295] | n/a | INCONCLUSIVE |
| **C3 m=23 k=2 w=2 (decides)** | 6,436 | 0/39 | **1.020 [0.934, 1.117]** | 0.998 [0.973, 1.024] | 2.212 [2.114, 2.313] | 0.461 [0.423, 0.505] | 1.023 [0.956, 1.093] | **NO-CHANGE** |

The same ratio for propagations (one unit priced as an unpriced native op in §7):

| config | B/A0 propagations | B/A0 decisions | A1/A0 propagations | B/A1 propagations |
|---|---|---|---|---|
| C1 | 1.150 [1.085, 1.224] | 0.709 | 3.657 | 0.314 |
| C2 | 0.682 [0.502, 0.927] | 0.692 | 4.548 | 0.150 |
| C3 | **2.233 [2.036, 2.457]** | 0.789 | 5.879 | 0.380 |

SAT and UNSAT split (conflicts, B/A0): C1 SAT 0.942, UNSAT 0.872. C2 SAT 1.429, UNSAT 0.543. In C3
every target is UNSAT (0 of 39 decomposable), which is the regime of a real IC relation search.

## Where the B/A1 saving comes from (C3, conflicts, vs A1)

| arm | what survives between targets | ratio vs A1 |
|---|---|---|
| CK | everything, but flushed every 4 targets | 0.858 [0.807, 0.915] |
| CI | only live derived clauses with **no R literal** (median 20,093 carried) | 0.759 [0.662, 0.870] |
| CA | all live derived clauses, no heuristic state (median 30,135 carried) | 0.687 [0.607, 0.775] |
| B | all clauses + phases, scores and restart state | 0.461 [0.423, 0.505] |

C1 shows the same ordering: CK 0.675, CI 0.869, CA 0.794, B 0.426. Target-independent clauses
alone buy about 24% at m=23. All the R-mentioning clauses together buy about 31%. The larger part
of B's advantage is heuristic state (B vs CA), which a cold solver could in principle also get
(saved phases, warm activity) with no clause reuse.

## Contrarian measurement: are retained learnt clauses target-independent?

| config | learnt clauses in B | with no R and no activation literal | mean applicability to a random target, Σ2^-#R / N (non-activation) | clauses with ≥7 R literals |
|---|---|---|---|---|
| C1 | 111,039 | 11.1% | 0.161 | 56% |
| C2 (k=3) | 6,015,241 | 51.8% | 0.561 | 37% |
| C3 | 3,054,809 | **6.1%** | **0.093** | 73% |

At k=2 the last S3 link involves R directly, so 89–94% of learnt clauses mention target bits. Most
of them carry seven or more R literals, so they are inert for almost every other target. At k=3,
the first link (S3(p0, p1, t)) is target-independent and half the learnt clauses are R-free. That
is the only setting where B's UNSAT-target saving reaches B/A0 = 0.543. In C2, SAT targets get
worse (1.43), and the overall CI is wide.

## Controls
- **A/A:** AA/A0 and BS/B both have CIs that include 1 at every configuration (see the table). The noise floor is about ±5–10%.
- **Positive control (repeat block), passed.** B/A1 on repeated targets: C1 0.438 [0.367, 0.526],
  C2 0.128 [0.086, 0.188], C3 0.318 [0.264, 0.373]. As noted before the C3 data (Amendment 1),
  in C1 the repeat ratio equals the stream ratio, so the repeat shows general warm-up rather than
  target-specific reuse. In C3 and C2 repeats are somewhat cheaper than new targets (0.32 against
  0.46 in C3).
- **Determinism:** B equals CA at position 0 (conflicts, decisions and propagations identical) in C1 and C3.
  Proof tracing does not change the search.

## Predictions vs outcome
- P1 (A1/A0 > 1): **held.** 2.07, 4.09, 2.21. Keeping R as assumptions instead of folding it into
  the circuit costs 2–4x the conflicts and 3.7–5.9x the propagations.
- P2 (B/A1 < 1 at m=11, nearer to 1 at C3): the first half held (0.43, 0.22). The second did not:
  C3 0.46 is about the same as C1.
- P3 (R-free share < 25% and applicability < 0.1 everywhere): held at C3 (6.1%, 0.093). It failed
  at C1 on applicability (0.161) and at C2 on both (51.8%, 0.561).
- P4 (C3 NO-CHANGE or NEGATIVE): **held**, NO-CHANGE.
- Falsifier (B/A0 upper CI < 0.8 at C3): not triggered.

## Counted-cost mapping (AGENTS.md §5a band)
Per-target medians of propagations (C3): A0 1.84e7, B 3.80e7, A1 1.08e8. Priced at 0, 0.1 and
1 GAE per propagation, that is A0 {0, 1.84e6, 1.84e7}, B {0, 3.80e6, 3.80e7}, A1 {0, 1.08e7, 1.08e8}
GAE per target. At 0.1 and 1, B ranks behind A0. At 0, both are zero. So no ordering in B's favour
holds across the band. Conflicts do not convert to GAE without a calibrated cost per conflict, and
this lane has none. No end-to-end IC number exists here (no relation yield, no linear algebra), so
**no gap to matched rho is reported.** Any saving shown here is a stage ratio of the decomposition
oracle, not an end-to-end speed-up.

## Limitations
- Toy fields only: m = 11 and 23. m = 29 at w = 3 censored the cold solve at 2e6 conflicts (an
  unscored smoke run). 2 is not primitive mod 23.
- One solver: CaDiCaL with `factor` off. The lab's goal22 harness uses CryptoMiniSat with native
  XOR, which this lane did not reproduce (cadical-sys has no XOR; pycryptosat was ruled out by
  Amendment 1 of COMMON.md).
- The circuit is the direct m²-AND ONB multiplier, not the codegen Karatsuba IR.
- The full-enumeration metric charges the final UNSAT call. A first-model-only relation search
  would weight SAT calls differently (first-call stats are in the rows).
- C2 ran 4 arms over 20 targets, so its CI is wide.
- One target stream per configuration. A second seed would show whether the stream matters.
