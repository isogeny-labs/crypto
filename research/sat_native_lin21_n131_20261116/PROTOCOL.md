# PROTOCOL: native `lin21` CDCL with clause deletion, a learning-free leaf and n = 131 (pilot)

Written 2026-11-16 (about 05:40Z) before any measured run; committed alone before the first
measured row. Local pilot on a shared 4-core container with another CPU-heavy agent. No
benchmark record, leaderboard row or pcc claim; every result line will be tagged
`[pcc:unverified]`. Wall times are indicative; the A/A band is reported with them. All
computation, checks, tables and figures are Rust (`rustc -O`, standard library only); `run.sh`
only builds and starts processes.

## Questions

1. Small fields (the lin21 pilots' series, n = 3l + 2): does learnt-clause deletion and/or a
   learning-free leaf stop the growth of (solver wall) / (fast scan-leaf wall) with l?
2. Does the solver work at n = 131 on the ECC2K-130 curve with multi-word field blocks, and
   what does it cost there against the fast scan leaf and against plain pair enumeration?

## Frozen code (sha256 at the time of writing)

- `fastcdcl_mw.rs` d921c948e3ef804d20bfd1418b5ff14b474ebec42f0a12d8e11ecb7f23b0f211: copy of
  `research/sat_native_lin21_fast_20261116/fastcdcl.rs`
  (ef88dca6020ceaefe1b14341a864037cd4e82787062330676ebd140aefcb3c05); its header lists every
  change (M1 multi-word blocks `[u64; W]`, M2 `delete_keep`, M3 `leaf_nolearn`, M4 counters)
- `mw.rs` a11a23e7d75f9a5e1e9a064e2a708add2b4a1db08ceb4e8f92f882afb8f90b79: generic-width field
  (pclmulqdq intrinsic when the build enables it, portable fallback otherwise), curve, the
  fast-leaf port, point-pair enumeration
- `n131.rs` af75c93936e034966b064573f5cfb37db416fd36d4cb176c6e74110abcb66bf6: harness
- `summary.rs` 7fbfbdd3c1b387bb38a9dba26ae436f7a7f4686ee9d787c6899f1aff52edb611: tables,
  decision, SVG figures
- `run.sh` 16cfbf2ddbd16be8fb844d8cb819ee956a597d491551fba36c1be2aafe0cacbc
- Included unchanged through `#[path]` / `include!`:
  `research/sat_native_lin21_fast_20261116/fastcdcl.rs` (ef88dca6..., arm `merged`),
  `research/sat_native_lin21_fast_20261116/base_lin21_verbatim.rs`
  (c9282e389106fa7ecb8e2246c20bcfc0f94c8b43b975b71f662bb951b9998891: field, curve, pair oracle,
  plain scan, model checks for n <= 63), `examples/pdp_lin_cdcl.rs` (996770f4..., needed by the
  included file, not a measured arm)
- Source of the fast-leaf port: `research/scan_floor_leaf_20261116/scan_leaf.rs`
  (816d0831ff42f9056e85569b387800f9828f314786c26eeced94e858f7c9196e), baseline arm (`Arm::Base`,
  level 4, no pretest). That file did not compile here with rustc 1.97.0 (type error at its line
  1314), so its own binary was not rerun; its README's 273 to 428 ns per leaf is the reference
  for the port's speed.
- Target files reused unchanged (seed 20261116), sha256 first 8 hex: l5 466cdaaf, l6 e66b6ec6,
  l7 9d0a7a84, l8 2fe17d90, l9 898f54f7, l10 298ef9c7, l11 b8ec6015, l12 b6bdd9d1 (lin21
  pilot), l13 2862dc4c (lin21-fast pilot).

Build: `rustc -O -C target-cpu=native` for `n131.rs` (all arms in one binary, the unmodified
`merged` arm included), rustc 1.97.0.

## Development before this protocol (seeds not used for measured rows)

- Seeds 555, 777, 991, 992 and three hand-picked small-field abscissae (300000078 at l = 9,
  20000000779 at l = 11, 1500000000778 at l = 13). No measured target file was run through any
  arm of this harness. Measured rows: the reused target files above and
  `k130targets --seed 20261116`.
- One change after a development measurement: the first generic reduction made the fast-leaf
  port cost 699 to 1,068 ns per leaf at n = 29..41; a one-word two-fold reduction path (the one
  of scan_leaf.rs) brought it to 241 to 469 ns per leaf. The denominator is thereby at the speed
  of the file it ports.
- Differential test, n <= 63 (`n131 dt --seed 991 --per 400`), run before this protocol: 2,800
  fresh instances at (n, l) = (10,3), (12,3), (13,3), (14,4), (15,4), (17,5), (20,6) (1,400
  planted, 1,447 decomposable); arms `mw`, `del` (keep 64), `del1` (keep 1), `leaf`, `delleaf`
  against the unmodified fastcdcl and the pair oracle: 0 verdict mismatches, 7,235 models
  checked, 0 bad, 0 non-liftable on admissible targets; control `mw` and the two-word control:
  conflict count equal to fastcdcl's on 2,800 of 2,800; `leaf` and `delleaf` differ in conflict
  count on 1 instance each, `del` and `del1` on none.
- Multi-word differential test (`n131 dtmw --seed 992 --per 10`): 11 cells from
  (W, n, d) = (1, 17, 5) to (3, 131, 6), 210 targets (105 decomposable); solver arms against
  point-pair enumeration and the fast-leaf port: 0 verdict mismatches, 420 models, 0 bad.
- `selfcheck`: pclmulqdq equals the portable multiply; z^131 + z^8 + z^3 + z^2 + 1 passes the
  prime-degree irreducibility test; the number A of nonzero abscissae of degree < d that lift is
  10, 17, 34, 74, 131, 252, 519 for d = 4..10, equal to the values behind the pair counts of
  `RESEARCH_ECC2K130_RR_SOLVER_PANEL.md` section 11 (45, 136, 561, 2,701, 8,515, 31,626, 134,421).
- Development timings (seed 777 and the three abscissae, one target per small cell, 8 per
  ECC2K-130 cell at d = 6, 7, 12) are the basis of the predictions below. Both run.sh dry runs
  used these only.

Both differential tests and the selfcheck are rerun by `run.sh` at every build.

## Small-field series

Setting as in the lin21 pilots: F_2^n, n = 3l + 2, l = 5..13, field polynomial `find_irr(n)`,
curve y^2 + xy = x^3 + x^2 + 1, V = elements < 2^l, problem ALG0. Targets: every line of the
reused files (16 non-decomposable uniform targets per cell at l <= 12 plus the decomposable
draws and 4 planted; 6 + 2 at l = 13).

One process per target, one at a time, 600 s cap. In each process: `scan` (the lin21 harness's
plain scan, median of 21 repeats), `fastleaf` (median of 21 repeats, setup excluded), then six
solver runs in an order rotated by the target index: `merged` (fastcdcl.rs unmodified), `mw`
(the copy, W = 1, switches off), `del` (keep the 64 newest learnt clauses), `leaf` (learning-free
leaf), `delleaf` (both), `merged` again (`merged-b`, the A/A control).

The two scan denominators:
- `scan`: `scan21` of `base_lin21_verbatim.rs`, bit-serial field, x-only.
- `fastleaf`: the port in `mw.rs` of the baseline leaf of scan_leaf.rs (batched t stage,
  half-trace elimination, root split, membership and group verification), with the field
  polynomial `find_irr(n)` so that the targets are the same. It needs the half-trace, so n odd:
  **it exists at l = 5, 7, 9, 11, 13 only.** It decides the point form (R = P_i + P_j + P_k over
  rational base points, zero abscissae skipped), which can differ from ALG0 on rare targets; its
  agreement with the pair oracle is reported as a count and is not an exactness condition of
  the solver arms.

Exactness conditions (a failure voids the decision): every scan and solver row has the pair
oracle's verdict (`oracle_sat` of the target file), hence the unmodified fastcdcl's; every SAT
model is in V^3, satisfies the chain equations, lifts and has a signed sum equal to the target.

### Decision rule (as given in the task, with one stated restriction)

For each of `del`, `leaf`, `delleaf`: slope = least-squares slope of
log2(median solver wall / median `fastleaf` wall) against l over the measured cells in 9..13,
which are **l = 9, 11, 13** (restriction: no fast-leaf denominator at even l); medians over the
non-decomposable uniform targets.
- "ratio flattens": some arm has |slope| <= 0.03 per unit of l and ratio at l = 13 <= 60;
- "does not flatten": every one of the three arms has slope > +0.05;
- otherwise inconclusive.
The same slopes against the plain scan over l = 9, 10, 11, 12, 13 are reported next to them and
are not part of the rule.

### Predictions (after the development runs above)

1. Conflicts: `mw` equal to `merged` on every target; `del`, `leaf`, `delleaf` equal to
   `merged` on at least 95 % of the non-decomposable targets; medians about 1.5 * 2^l.
2. Clause-propagation share at l = 13: `merged` 18 to 30 %, `del` at most 5 %, `leaf` and
   `delleaf` at most 4 %.
3. Ratio to `fastleaf` at l = 13: `merged` in [120, 200]; `del`, `leaf`, `delleaf` in
   [90, 160]. So the threshold 60 is not met and "ratio flattens" is not expected. Slope of the
   three arms over l = 9, 11, 13: in [-0.10, +0.06]; `merged`: in [0, +0.12]. Expected rule
   outcome: **inconclusive** (flat or falling slope, ratio above 60).
4. Ratio to the plain `scan` at l = 13: `merged` in [15, 27], `delleaf` in [11, 20].
5. `fastleaf`: 230 to 300 ns per leaf at n = 29, 380 to 520 at n = 41.
6. A/A (`merged-b` / `merged`): per-cell median in [0.9, 1.1]; single targets in [0.6, 1.6].

## ECC2K-130 series

Curve y^2 + xy = x^3 + 1 (a = 0, b = 1) over F_2[z] / (z^131 + z^8 + z^3 + z^2 + 1). Sources:
the curve from `ecc2k130/codegen/curves.py` (`CurvePb`: "y^2 + xy = x^3 + 1 over a
polynomial-basis GF(2^m)", used by `research/nagao_relations/solver_16/weil.py`); the polynomial
from `ecc2k130/benchmarks/sparse-basis-compound/PROTOCOL.md` (f_1) and confirmed by the A values
above. The summation polynomial uses b = 1; a enters only the point arithmetic. Subspace
V = polynomials of degree < d (polynomial basis), d = 6..13; d = 14 as an extension cell.
Targets per d: `n131 k130targets --d d --seed 20261116 --uniform 8 --planted 2` (uniform:
random rational abscissa outside V, expected non-decomposable; planted: abscissa of a sum of
three random base points).

Arms, one process per target, order rotated by the target index:
- `solver`: `fastcdcl_mw` with W = 3 in the `delleaf` configuration (chosen now, from the
  development runs: no clause growth, least clause propagation), and `solver-b`, the same
  again (A/A);
- `fastleaf`: the same port at W = 3 (median of 21 repeats at d <= 10, of 5 above);
- `pairs`: plain pair enumeration in point form: every unordered pair of base points (both
  signs, i <= j), test x(R - P_i - P_j) of degree < d, batched affine additions, one run. On
  every target for d <= 13; at d = 14 on targets 0 and 8 only (its wall does not depend on the
  target). The panel's pair count C(A, 2), A = nonzero abscissae in V that lift, is the unit in
  which conflicts are reported; the number of point pairs this arm tests (about 4 times that) is
  reported beside it.

Reported per d over the uniform targets: A, C(A, 2), median conflicts and conflicts as multiples
of A and of C(A, 2), median wall of the three arms, the two wall ratios, A/A.
Checks: every arm UNSAT on every uniform target and SAT on every planted one; every solver model
in V^3, lifting, with a signed sum equal to the target; every scan and pair hit verified in the
group.

What the over-determination changes (n = 131 >> 3d, against the balanced n = 3l + 2): a leaf's
linear system has 3d - 1 unknowns (s and p) and 131 equations, so a non-decomposable leaf is
inconsistent with probability about 1 - 2^(3d - 131): no consistent leaves, no kernel
enumeration, no root split, where the balanced series keeps about 1 leaf in 8. The fast leaf
only builds d + 1 columns of 131 - 2d + 2 bits and fails in elimination. The solver detects the
contradiction in the gate rows but has already inserted the 131-row gates before it, on rows of
31 words (15 blocks of 131 variables); its work per conflict is set by n, not by d. The margin
could be used to cut each leaf to about 3d + 20 equations; neither arm does that here.

### Predictions (after seed-777 development runs at d = 6, 7, 8, 10, 12)

1. Conflicts per uniform target: 2.8 to 3.2 times A at d = 8 and 2.9 to 3.15 times A at d = 12
   (about 1.5 * 2^d: two leaves per liftable x1, one conflict per other x1). As a multiple of
   C(A, 2): 0.042 to 0.050 at d = 8 (the panel's CryptoMiniSat on S'4: 36.7) and 0.0028 to
   0.0032 at d = 12. No learnt clause on uniform targets.
2. Wall, solver / pairs: 4 to 11 at d = 8; 0.35 to 0.75 at d = 12 (crossing 1 between d = 10
   and d = 12, since pairs grow 4x per d and the solver 2x).
3. Wall, solver / fastleaf: 170 to 330 at d = 8; 150 to 260 at d = 12 (no trend toward parity).
4. Slopes of log2(wall) per d: solver 0.95 to 1.05, fastleaf 1.0 to 1.1, pairs 1.8 to 2.0.
5. A/A: per-cell median in [0.9, 1.1].

## Not covered

Even-n fast-leaf denominators (l = 6, 8, 10, 12); d > 14; restarts, Gray-code order, a
strictly learning-free solver (the leaf rule still learns where two non-priority decisions are
on the trail); truncating the over-determined leaf to fewer equations in either arm; a host
without a second heavy job; repeated runs per target; external solvers; m > 3; any size at which
this matters for an index-calculus cost; the gap to Pollard rho (unchanged: the scan remains the
best arm here).
