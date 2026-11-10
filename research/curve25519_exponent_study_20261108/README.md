# Exponent study: does the subgroup-base algebraic route have an exponent lever? (2026-11-10)

**Class.** An engine measurement in counted units (F_p multiplications of the F4 row reduction,
`F4Report.field_ops`, from `curve25519_family f4sub`). It measures the cost of one decomposition
system against the base size. It is not an end-to-end solve, not an ecbench session, and not a
leaderboard or `benchmarks/` row. Every measured number below is an unverified assertion; no pcc
claim was registered. Rho remains ahead on every track [pcc:unverified].

**Question (owner preregistration, 2026-11-08).** Algorithm 1 of the algebraic ECDLP attack on
prime curves (Petit, Kosters, Messeng, PKC 2016) with the cost model of its section 3.2: total
about P(p,m) + (m! p / (deg L)^(m-1)) T(E,m,L) + (deg L)^omega, with deg L = |V|. With omega = 2
and |V| = p^a, m = 3 balances at a = 1/4 and m = 4 at a = 1/5. An exponent lever exists at m = 4
only if T(E,4,L) grows more slowly than |V|^(1/2), and at m = 3 only if T is flat in |V|. This
study fits the exponent s of T against |V| on the symmetrised system over multiplicative-subgroup
bases of the Curve25519-family miniatures.

**Answer.** No lever was found. At m = 2 the preregistered fit gives s = 2.77 [2.31, 3.23] at
k = 20, 2.81 [1.81, 3.80] at k = 22 and 2.57 [0.91, 4.24] at k = 24 [pcc:unverified]. At m = 3 and
m = 4 the preregistered estimator could not be applied: the complete cells span less than 4x in
|V| before the first censored cell (an out-of-memory exit at 3.5 GB), so the preregistered verdict
is "undetermined" [pcc:unverified]. Every other reading of the same cells puts s far above the
thresholds. The m = 3 sensitivity fits that include d = 3 and 7 give 5.54 [2.30, 8.78] at k = 20
and 5.76 [3.77, 7.75] at k = 22 [pcc:unverified]. The m = 4 cost rises by a factor of 3.4e5 from
|V| = 3 to |V| = 9 [pcc:unverified]. Neither reading comes near the m = 3 threshold (s < 0) or
the m = 4 threshold (s < 0.5).

Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen on 2026-11-08 before any measurement (sha256 in
[`data/frozen.sha256`](data/frozen.sha256), verified unchanged). Prior studies:
[`PRIOR_STUDIES.md`](PRIOR_STUDIES.md), an addendum written after the grid had run. Report:
[`REPORT.pdf`](REPORT.pdf). Figure: [`exponent_fit.svg`](exponent_fit.svg).

## 1. System and engine

- **Module.** [`src/cryptanalysis/ic_framework/subgroup_semaev.rs`](../../src/cryptanalysis/ic_framework/subgroup_semaev.rs),
  commands `curve25519_family f4sub` and `subcensus`. It is a separate module rather than a new
  `Membership` variant of `pkm_semaev.rs`: the tower engine's bit-mask monomials hold exponents 0
  or 1 only, so they cannot hold the odd-degree chain rules y^3 = y'. No path of `pkm_semaev` or
  `f4_fp_tower` changed.
- **Base.** V = lo + g mu_d, d | p - 1 odd, d = e_0 e_1 ... e_{L-1} (primes ascending, with
  multiplicity), lo = A/3 (the Montgomery frame), g = w^gidx with w the smallest primitive root.
- **Per-summand constraint.** This is the power-map chain of the paper: x_i = lo + g y_{i,0},
  y_{i,j}^{e_j} = y_{i,j+1} for j < L - 1, and the closing relation y_{i,L-1}^{e_{L-1}} = 1. The
  paper's closing map is 1 - x^{p_n'}; this is the same relation.
- **Target and form.** The target X = x(aG) is a constant, with a uniform in [1, r - 1]. The form
  `sym` (primary) adds e_k - sigma_k(x_1..x_m) for k = 1..m and the Semaev polynomial
  S_{m+1}(x_1..x_m, X), written in e_1..e_m by interpolation. The interpolation is checked on 12
  extra points. The form `plain` substitutes sigma_k(x) for e_k (ablation).
- **Engine.** `f4_fp::f4`, grevlex, max degree 400. There is no guessing (G = 0): T is the cost of
  one complete system.
- **Unit tests.** Six tests in the module pass. They cover: the summation polynomials vanish on
  sums (m = 2, 3, 4); S~ equals S (m = 2, 3, 4); the m = 2 and m = 3 systems are consistent exactly
  when the target is a sum of m base points, brute-forced on p = 1013 with small subgroups; the
  bit-interval base matches brute force. They were run from the release test binary built at the
  module's commit (6 passed, 0 failed).

## 2. Grid and census

The grid is as frozen in section 4 of the protocol: 6 targets per cell, seed 20261108 + 1000 m + d,
a cap of 1200 s per target, and 3.5 GB of memory per process. Each (k, m, form) chain runs d
ascending and stops at its first censored cell. The census (`data/census.json`, from
`scripts/census.py`) is exact. Column definitions: "columns" is the number of abscissae of V on
the curve; "base points" counts the signed base points; "covered" counts the distinct points of
the order-r subgroup that are sums of exactly m base points.

Curves: k = 18 (p = 262,133, r = 32,887), k = 20 (p = 1,048,573, r = 131,203), k = 22
(p = 4,194,301, r = 524,731) and k = 24 (p = 16,777,213, r = 2,097,349). The values are measured,
from `data/census.json`.

The full per-cell table, with census, T, solving degree and largest matrix width, is in
[`data/tables.md`](data/tables.md). The primary series are below. T is the mean `field_ops` per
system. L is the number of chain levels. All values are measured, from `cells/*.json`.

| k | m | d = \|V\| | L | columns | covered (m) | T | solving degree | status |
|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 20 | 2 | 3 | 1 | 2 | 2 | 1,648 | 4 | complete |
| 20 | 2 | 7 | 1 | 2 | 2 | 11,225 | 8 | complete |
| 20 | 2 | 9 | 2 | 4 | 8 | 52,244 | 5 | complete |
| 20 | 2 | 19 | 1 | 12 | 40 | 96,745 | 20 | complete |
| 20 | 2 | 21 | 2 | 8 | 20 | 434,893 | 9 | complete |
| 20 | 2 | 27 | 3 | 16 | 68 | 1,789,928 | 5 | complete |
| 20 | 2 | 57 | 2 | 34 | 314 | 3,876,409 | 21 | complete |
| 20 | 2 | 63 | 3 | 28 | 200 | 22,317,287 | 9 | complete |
| 20 | 2 | 73 | 1 | 36 | 328 | 1,869,761 | 74 | complete |
| 20 | 2 | 133 | 2 | 76 | 1,452 | 35,142,880 | 22 | complete |
| 20 | 2 | 171 | 3 | 108 | 3,024 | 383,236,931 | 21 | complete |
| 20 | 2 | 189 | 4 | 96 | 2,288 | 393,654,078 | 10 | complete |
| 20 | 2 | 219 | 2 | 120 | 3,552 | 61,803,451 | 75 | complete |
| 20 | 2 | 399 | 3 | 216 | 11,254 | 2,016,013,118 | 22 | complete |
| 20 | 2 | 511 | 2 | 274 | 17,640 | 1,463,328,780 | 76 | complete |
| 20 | 2 | 513 | 4 | 278 | 18,564 | 4,027,107,095 | 22 | complete |
| 20 | 3 | 3 | 1 | 2 | 0 | 19,010 | 4 | complete |
| 20 | 3 | 7 | 1 | 2 | 4 | 3,303,793 | 8 | complete |
| 20 | 3 | 9 | 2 | 4 | 0 | 147,635,021 | 7 | complete |
| 20 | 3 | 19 | 1 | 12 | 144 | 220,436,708 | 20 | complete |
| 20 | 3 | 21 | 2 | 8 | 56 | 2,787,948,088 | 11 | complete |
| 20 | 3 | 27 | 3 | 16 | 344 | M27 | - | S27 |
| 20 | 4 | 3 | 1 | 2 | 4 | 423,322 | 8 | complete |
| 20 | 4 | 7 | 1 | 2 | 4 | 24,975,907 | 8 | complete |
| 20 | 4 | 9 | 2 | 4 | 24 | 142,058,000,763 | 9 | complete |
| 20 | 4 | 19 | 1 | 12 | 504 | - | - | censored (out of memory) |
| 22 | 3 | 3 | 1 | 3 | 0 | 19,010 | 4 | complete |
| 22 | 3 | 5 | 1 | 3 | 4 | 201,745 | 6 | complete |
| 22 | 3 | 11 | 1 | 7 | 32 | 20,063,347 | 12 | complete |
| 22 | 3 | 15 | 2 | 9 | 52 | 1,759,715,051 | 9 | complete |
| 22 | 3 | 25 | 2 | 19 | 576 | 79,458,456,936 | 10 | complete |
| 22 | 3 | 31 | 1 | 11 | 84 | 2,016,009,937 | 32 | complete |
| 22 | 3 | 33 | 2 | 17 | 420 | 91,887,410,934 | 15 | complete |
| 22 | 3 | 41 | 1 | 23 | 1,020 | 7,342,801,721 | 42 | complete |
| 22 | 3 | 55 | 2 | 29 | 2,076 | - | - | censored (out of memory) |

The m = 2 series at k = 22, 24 and 18, the plain-form ablations and the coset cells are in
`data/tables.md`.

## 3. Fitted exponents

The preregistered estimator is OLS of ln T on ln |V| over the complete cells with d >= 9, with a
two-sided 95% t-interval. It requires at least 4 cells spanning at least 4x in |V|. Its results are
in `data/analysis.json` (from the frozen `scripts/analyse.py`).

| series | cells in the fit | s | 95% interval | sensitivity fit (all d) |
|:--|--:|--:|:--|:--|
| k = 20, m = 2, sym | 14 | 2.77 | [2.31, 3.23] | 2.80 [2.47, 3.12] (16 cells) |
| k = 22, m = 2, sym | 9 | 2.81 | [1.81, 3.80] | 2.72 [2.23, 3.21] (11) |
| k = 24, m = 2, sym | 4 | 2.57 | [0.91, 4.24] | 2.43 [1.89, 2.97] (5) |
| k = 20, m = 2, plain | 6 | 3.20 | [1.67, 4.72] | 3.31 [2.65, 3.97] (8) |
| k = 18, m = 2, sym | 3 cells in all | no fit | - | local slopes 2.19, 3.91 |
| k = 20, m = 3, sym | F20 | no fit (span < 4x) | - | 5.54 [2.30, 8.78] (5) |
| k = 22, m = 3, sym | 6 (span 3.7x) | no fit (span < 4x) | - | 5.76 [3.77, 7.75] (8) |
| k = 20, m = 3, plain | 3 before censoring | no fit | - | 6.48 [2.45, 10.52] (5) |
| k = 20, m = 4, sym | 1 (d = 9) | no fit | - | local slopes 4.81 (3 to 7), 34.4 (7 to 9) |

The table values are measured (fits derived from the cells) [pcc:unverified].

**Exploratory (not preregistered): the chain length matters.** T depends strongly on how d
factors, not only on |V|. At equal |V|, a composite d (more chain levels) costs more than a prime
d. For example, at k = 20, m = 2, T is 1.87e6 at d = 73 (L = 1) and 2.23e7 at d = 63 (L = 3)
[pcc:unverified]. Some negative local slopes in the preregistered series come from this. Fits
restricted to one chain length (`scripts/tables.py`, `data/exploratory_fits.json`) are tighter:

- m = 2, L = 1 gives 2.20 [2.15, 2.25] at k = 20 (d = 3, 7, 19, 73), 2.19 [2.14, 2.23] at k = 22
  (5 cells) and 2.21 [2.13, 2.28] at k = 24 (3 cells) [pcc:unverified].
- m = 2, L = 2 gives 2.43 [2.10, 2.77] at k = 20 (6 cells) [pcc:unverified].
- m = 3, L = 1 gives 4.95 [4.51, 5.40] at k = 22 (d = 3, 5, 11, 31, 41) [pcc:unverified].

For prime d the solving degree is d + 1. The cost is then that of one dense equation of degree
about d in each chain variable.

**Comparison with the tower base (L10).** L10 measured m = 2 on the tower base with guessing
G = t - 1. There T grows as about |V|^1.5 (mpt 17,592 / 52,816 / 146,464 at t = 4, 5, 6). At
G = 0, the same curve (k = 20) and the bit-interval base, the tower engine's `f4cost` gives
87,013 / 4,046,086 / 127,284,443 mul-adds per target at t = 3, 4, 5 [pcc:unverified]. That is
local slopes of 5.5 and 5.0 in |V| = 2^t (derived from `data/calib_tower_bits_t*.json`).
Without guessing, the subgroup base (s about 2.8) therefore grows more slowly than the bit-interval
base (s about 5). Both are far above the 1/2 that a lever at m = 4 needs.

## 4. Balanced totals under the paper's model (derived)

The relation phase costs p^(1 - (m-1)a) T(p^a) = p^(1 - (m-1)a + s a), and linear algebra costs
p^(2a). When s >= m - 1, the relation-phase exponent 1 + (s - (m - 1)) a does not decrease with
a. No base size then gives a total below p^1, and the balance 2/(m + 1 - s) has no meaningful
solution.

- **m = 2.** The measured s is 2.57 to 2.81, which is at least m - 1 = 1. No base size improves on
  p^1 (derived).
- **m = 3.** Every available reading (4.95 to 5.76) is at least m - 1 = 2. No base size improves
  on p^1 (derived).
- **m = 4.** Below p^1 would need s < 3. Below the generic p^(1/2) would need s < 1 at the
  re-balanced a, or s < 0.5 at the paper's a = 1/5. The two local slopes measured are 4.81 and
  34.4 (derived).

These extrapolations from fixed small p are derived, not measured.

## 5. Verdicts on the preregistered thresholds and predictions

| item | preregistered rule | outcome |
|:--|:--|:--|
| lever at m = 4 | s < 0.5 with upper 95% bound < 0.5 | **undetermined** by the estimator: one complete cell with d >= 9, d = 19 censored. Not met on any reading (local slopes 4.81, 34.4) |
| lever at m = 3 | s < 0 with upper bound < 0 | **undetermined** by the estimator: span 3.7x at k = 22 and RS20 at k = 20, below 4x. Not met on any reading (sensitivity fits 5.54 and 5.76, lower bounds 2.30 and 3.77) |
| looser derived m = 4 break-even | s < 1 | not met on any reading |
| prediction m = 2: s = 1.5 (range 1.0 to 2.3) | | **failed**: 2.77 [2.31, 3.23] at k = 20, outside the range |
| prediction m = 3: s = 3.5 (range 2.5 to 5) | | not evaluable by the estimator. The readings 5.54 and 5.76 are above the range; the L = 1 fit, 4.95, is inside it |
| prediction m = 4: s >= 4.5 or capped beyond d = 9 or 21 | | **held**: censored at d = 19 |
| plain vs sym: \|s_plain - s_sym\| < 0.5 | | **held**: 3.20 vs 2.77, a difference of 0.43 |
| coset dependence: T within 1.5x across gidx | | **held**: at d = 27, T is 1,789,928 / 1,789,919 / 1,789,928 for gidx = 0, 1, 2; at d = 63 it is 22,317,287 / 22,317,373 / 22,317,372 |
| calibration: field_ops / f4_muladds within 5x | | **held**: 3.39, 2.58 and 1.41 at t = 3, 4, 5 |
| calibration: that ratio constant to 30% | | **failed**: it falls from 3.39 to 1.41 |
| owner's expectation: no lever | | consistent: no reading comes near a threshold |

Failed predictions stay failed. The table values are measured [pcc:unverified].

## 6. What a lever would need

Under the paper's model, m = 4 would need a decomposition system whose cost grows more slowly than
|V|^(1/2), and m = 3 would need one whose cost does not grow at all. Measured here, the cost grows
faster than |V|^2 at m = 2 on every series. Trivial enumeration of V^(m-1), with a root test for
the last summand, costs |V|^(m-1). That is already |V|^3 at m = 4, six times the needed exponent.
A lever would therefore need a solver that does not enumerate or reduce over the |V|^m quotient of
the chain ideal. This study found no such solver. The m = 2 slope of the prime-d cells, about 2.2,
is above even the enumeration exponent 1. This suggests the F4 route over the chain ideal costs
more than enumeration on this base (derived).

## 7. Distance to rho

This study does not produce an end-to-end cost. As an illustration only, the modelled cost
E = columns · r · T / covered at k = 20, m = 2 ranges from 3.98e5 x (d = 3) to 1.46e10 x (d = 513)
the 1.5 sqrt(r) rho reference (derived; field_ops units; native operations unpriced, so the w
band of AGENTS.md section 5a is not applied). It is not comparable with L10's mul-add figures
beyond the calibration ratio of 1.4 to 3.4. Rho remains ahead.

## 8. Not done and open issues

- No m = 3 or m = 4 exponent could be fitted under the frozen rule. A wider span at m = 3 needs
  more than 3.5 GB of memory per process (d = 55 at k = 22 exited out of memory) or a sparser
  linear-algebra back end. That is a resource limit, not a result.
- The prime-d versus composite-d dependence was not anticipated in the protocol. A follow-up
  should fix L, or regress on L, in its preregistration.
- The `ecdlp_exponent_assessment_20261018` PDFs live in the artifact store and were not
  retrievable on this machine.
- The research-kb, ecdb and Conductor tools were unavailable in this session: no MCP server and
  no token, and Conductor is not installed. Sources checked: the repository files named in
  `PROTOCOL.md` and `PRIOR_STUDIES.md`.
- No pcc claims were registered. Every result is an unverified assertion until checked.

## Files

| file | what |
|:--|:--|
| `PROTOCOL.md`, `data/frozen.sha256` | the frozen protocol and the hashes of the protocol, the scripts and the binary |
| `PRIOR_STUDIES.md` | the addendum on the five earlier exponent studies |
| `scripts/run_grid.py` | the grid runner (resumable) |
| `scripts/analyse.py` | the preregistered estimator; writes `data/analysis.json` |
| `scripts/census.py` | the exact census; writes `data/census.json` |
| `scripts/tables.py` | `data/tables.md` and the exploratory fits at fixed L (`data/exploratory_fits.json`) |
| `scripts/figure.py` | `exponent_fit.svg` and `.png` |
| `cells/*.json` | one file per cell, with per-target field_ops, solving degree, rows and columns |
| `data/calib_*.json` | the calibration cells (bit-interval base, both engines) |
| `data/run.log` | the runner's log |
| `REPORT.typ`, `REPORT.pdf` | the report |

Reproduce: `cargo build --release --bin curve25519_family`, then from this directory
`python3 scripts/run_grid.py`, `python3 scripts/census.py`, `python3 scripts/analyse.py`,
`python3 scripts/tables.py` and `scripts/figure.py`.
