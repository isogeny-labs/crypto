# Idea 2: linear-leaf XCNF (pilot, 2026-11-14)

Pilot only: toy sizes, one shared container, nothing here is claim data. Every number below
is an unverified pilot assertion, tagged [pcc:unverified]. Protocol was written first:
`PROTOCOL.md`. Code: `pilot.py` (self-contained field, curve, encodings, arms),
`analyze.py`. Raw per-instance rows: `results.json` (merged from `raw/run_*.json`, logs in
`raw/*.log`). Full tables: `summary_tables.md`.

## What was run

- F_{2^23} = F2[t]/(t^23+t^5+1), curve y^2+xy = x^3+x^2+1, V = span(1..t^{l-1}), m = 3.
- Per l: 8 planted targets + 8 random targets, seeds 20261114*1000 + 100*l + idx.
  Random targets were all non-decomposable for l <= 7 (ground truth: native linear scan,
  cross-checked against an independent MITM table of all signed pairs for l <= 7).
- Arms (all on the same targets, interleaved per target):
  - L: native linear 2+1 scan, unit = leaf solves (2|F| - 1 here, about 2^l).
  - B0: untreated SAT baseline, chained S3(x1,xR,u), S3(x2,x3,u), CryptoMiniSat 5 with
    native XOR clauses (one Gauss matrix detected in every run), solver free.
  - T1: same, second S3 written in (s,p) (the affine form), solver free.
  - T2: T1's instance, x1 enumerated by assumptions in one incremental solver.
  - T3: x1 substituted by an outer loop, fresh solver per x1 (u left to the solver).
  - T4: x1 and u substituted by the outer loop; solver sees pure XORs in (s,p) + p = x2*x3.
  - G-B0/G-T1/G-T2: same encodings in Glucose 4, XORs cut to 4-literal CNF chunks.
- Work unit: CMS conflicts (from the solver's own `c [solve] ... confl:` line); for
  T2/T3/T4 conflicts + number of solve calls. Glucose rows also carry decisions and
  propagations in `results.json`.
- Validation: every SAT/UNSAT answer of every arm equals the native scan on every
  uncensored row (656 rows, assert-enforced); every SAT model lifted to curve points with a
  sign choice summing to the target; T4 produced 0 spurious leaf models.

## Deviations from PROTOCOL.md (all decided after the validation run, before the measured run)

1. l = 2 added, because validation showed B0 would be censored at l = 5 (cap 300 000
   conflicts) and only l = 3, 4 would be left for the baseline slope. l = 2 is a degenerate
   cell (|F| = 2).
2. Glucose arms only at l = 2, 3 (at l = 4 most Glucose solves hit the cap in validation).
3. B0 and T1 not run at l >= 6 (certain censoring). T2 not run at l = 8.
4. Cells were run as separate processes (up to 4 at once, one core each) instead of one
   sequential process. Conflict counts do not depend on this; wall times do.
5. l = 7 and l = 8 hit the run deadline: T2 at l = 7 has 2 non-decomposable targets, T3 at
   l = 7 and l = 8 has 1, T4 1 (34 rows NOT_RUN, listed in `results.json`).

## Results, non-decomposable targets (mean work per target) [pcc:unverified]

| l | |F| | L leaf solves | B0 | T1 | T2 (confl/call) | T3 (confl/call) | T4 (confl/call) | 2^l | 2^(2l) | 2^(3l)/6 |
|---|---|---|---|---|---|---|---|---|---|---|
| 2 | 2 | 3 | 1 996 | 177 | 193 (47) | 8.75 (1.2) | 3 (0) | 4 | 16 | 11 |
| 3 | 4 | 7 | 8 453 | 5 032 | 10 718 (1 339) | 71 (7.9) | 7 (0) | 8 | 64 | 85 |
| 4 | 6 | 11 | 121 420 | 109 136 | 35 990 (2 248) | 508 (31) | 11 (0) | 16 | 256 | 683 |
| 5 | 12 | 23 | all 8 censored (> 300 000) | 7 of 8 censored | 106 325 (3 322) | 5 278 (164) | 23 (0) | 32 | 1 024 | 5 461 |
| 6 | 27 | 53 | not run | not run | 354 490 (5 538) | 81 556 (1 273) | 53 (0.01) | 64 | 4 096 | 43 691 |
| 7 | 54 | 107 | not run | not run | 1 599 898 (12 498), n = 2 | 941 768 (7 357), n = 1 | 117 (0.09), n = 1 | 128 | 16 384 | 349 525 |
| 8 | 119 | 237 | not run | not run | not run | 6 209 450 (24 255), n = 1 | 760 (2.2), n = 1 | 256 | 65 536 | 2 796 203 |

n = 8 targets per cell unless stated. Glucose, l = 2 / 3, mean conflicts: G-B0 795 / 10 846,
G-T1 264 / 4 175, G-T2 483 / 28 914 [pcc:unverified].

Fitted bits per l (log2 of mean work, uncensored cells only) [pcc:unverified]:

| arm | sizes | slope |
|---|---|---|
| L native scan | 2..6 | 1.00 |
| B0 baseline SAT | 2, 3, 4 | 2.96 (3.84 on l = 3, 4 only) |
| T1 free, (s,p) encoding | 2, 3, 4 | 4.63 (4.44 on l = 3, 4 only) |
| T2 assumptions on x1 | 2..7 | 2.34 (3.77 on l = 2..4; 1.80 on l = 3..7) |
| T3 x1 substituted | 2..8 | 3.32 |
| T4 x1 and u substituted | 2..8 | 1.23 (1.00 on l = 2..6) |

Decomposable (planted) targets are in `summary_tables.md`; same ordering of arms, with the
enumeration arms stopping at the first hit.

## Reading against the pre-written kill criterion

- Prediction "conflicts fall to about 2^l" held only for T4, where the outer loop does the
  whole x1 and u enumeration and the solver is a leaf engine: 0 conflicts in nearly every
  leaf (Gauss decides at level 0), work = number of leaves = L's leaf count. That is the
  linear 2+1 scan by construction, not a SAT result, and it is never below the scan count
  [pcc:unverified].
- As soon as the solver holds any of the bilinear part, the per-x1 residual is not O(1):
  T2 spends 1 339 -> 12 498 conflicts per assumed x1 (l = 3 -> 7) and T3 7.9 -> 24 255
  conflicts per substituted x1 (l = 3 -> 8), i.e. T3's per-leaf cost alone grows about
  2.3 bits per l. Fixing x1 leaves an F2-affine system in u, but CMS's Gauss matrix does
  not see it through the AND gates u_j & s_i, u_j & p_k, and the solver searches (s,p,x2,x3)
  [pcc:unverified].
- Absolute level: at l = 6 the T2 work (354 490) is 5 500x the native scan (64 = 2^l), 87x
  2^(2l) and 8x 2^(3l)/6. The SAT baseline B0 on this chained encoding is itself far above
  one conflict per tuple (121 420 at l = 4 against 683 tuples), so B0 is a weak baseline
  relative to the repository's WDSat reference line; ratios to B0 say little
  [pcc:unverified].
- Slope test (T1 or T2 at least 0.7 bits per l below B0): the protocol's own rule makes
  this INCONCLUSIVE, because B0 completed uncensored only at l = 3, 4 of the pre-registered
  sizes. With the added l = 2 cell: T1 is 1.67 bits per l above B0 (kill direction); T2 is
  0.81 above B0 on the matched sizes 2..4 (kill direction) and 0.62 below B0 when T2's
  l = 2..7 fit is set against B0's l = 2..4 fit (short of the 0.7 bar). Excluding the
  degenerate l = 2 cell, T2 fits 1.80 on l = 3..7 against a two-point B0 figure of 3.84,
  which would read as a reduction; that comparison is not on matched sizes and B0's l = 5
  is only known to be above the cap [pcc:unverified].

Verdict: inconclusive on the pre-registered slope comparison (baseline censored), with the
measured levels pointing against the idea: no arm in which the solver does part of the
search comes near 2^l, and the only arm at 2^l is the native scan with a SAT leaf.

## What a full study would need

- A baseline SAT arm that reproduces the repository's one-conflict-per-tuple line on these
  instances (WDSat or the crate's symmetrised encoding), so that the slope comparison has
  a credible reference and at least 4 uncensored sizes.
- A treatment in which branching really is confined to x1 and one root bit of u, and the
  XOR engine re-derives the linear system after those are set: either a solver with
  user-controlled decision variables plus XOR reasoning over substituted ANDs
  (WDSat-style), or an encoding that makes the x1-fixed system syntactically linear for
  the Gauss matrix. CMS through pycryptosat exposes neither decision restriction nor
  per-call statistics beyond the conflict line.
- Varying n with l (n near 3l) instead of fixed n = 23, since the free 23-bit u dominates
  the small-l cells; more targets per cell; decisions and propagations for the CMS arms;
  m = 4.
- Not covered here: m = 4, any SAT arm at l >= 6 for B0/T1, Glucose beyond l = 3,
  decision-variable restriction inside the solver (only emulated by assumptions),
  CMS option sweeps (default options only), wall-time comparisons (processes shared cores).

## Rerun

Smallest cell (about 5 s):
`ARMS=B0,T1,T2,T3,T4 python3 pilot.py /tmp/idea2_l3.json 20261114 300 3`

Session-log note: the repository's `docs/session-logs/` entry was not written, because this
pilot was restricted to its own directory; this README is the handoff.
