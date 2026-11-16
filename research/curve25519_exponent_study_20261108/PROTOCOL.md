# Exponent study: does the subgroup-base algebraic route have an exponent lever? (protocol)

Frozen before any measurement; the sha256 of this file is in `data/frozen.sha256`.
Branch `claude/exponent-study-20261108`. Owner preregistration of 2026-11-08.
Conductor is not installed on this machine (the hooks warn and let edits through).
ecdb (no `ECDB_TOKEN`, no AWS access) and research-kb (no MCP server in this session) were
unreachable; curve facts come from `curve25519_family`'s own derivation. Sources read instead:
`research/curve25519_leads_wave4_20261104/l11_odd_kummer_design/PROTOCOL.md`,
`research/curve25519_leads_wave2_20261103/t3_m3_engine/README.md`,
`research/curve25519_rho_parity_bound_20261104/ALGEBRAIC_SCREEN.md`.

## 1. Question

Algorithm 1 of the algebraic ECDLP attack on prime curves, with the cost model of its section 3.2:
total ~ P(p,m) + (m! p / (deg L)^(m-1)) T(E,m,L) + (deg L)^2, deg L = |V| = p^a. Balancing gives
m = 3 at a = 1/4 (total p^(1/2) T) and m = 4 at a = 1/5 (total p^(2/5) T).
An exponent lever against the generic p^(1/2) exists at m = 4 only if T(E,4,L) grows more slowly than
|V|^(1/2), and at m = 3 only if T = O(1). The question is the exponent of T(E,m,L) in |V| for
m = 2, 3, 4 on multiplicative-subgroup bases of the Curve25519-family miniatures.

## 2. System (the formulation implemented)

Module `src/cryptanalysis/ic_framework/subgroup_semaev.rs`; command `curve25519_family f4sub`.
Base V = lo + g mu_d, d = e_0 e_1 ... e_{L-1} (primes ascending with multiplicity, smallest at the
bottom). Per summand i = 1..m the chain variables y_{i,0..L-1}:
x_i = lo + g y_{i,0}; y_{i,j}^{e_j} = y_{i,j+1} (j < L-1); y_{i,L-1}^{e_{L-1}} = 1.
These are ordinary generators (they are not tower rules: the tower engine's bit-mask monomials cannot
hold odd-degree levels). Target X = x(aG), a uniform in [1, r-1], is a constant.
`sym` (primary): variables y_{i,j} then e_1..e_m; generators = chain rules, e_k - sigma_k(x_1..x_m)
(k = 1..m) and S~(e_1..e_m; X), the Semaev polynomial S_{m+1}(x_1..x_m, X) written in the elementary
symmetric polynomials. S~ is obtained by interpolation on the monomials e^a with sum a_k <= 2^(m-1)
from numerical S_{m+1} (closed forms for m = 2, 3; Res_U(S_4(x1,x2,x3,U), S_3(x4,X,U)) for m = 4),
with 12 extra check points that certify symmetry and degree. `plain` (ablation): e_k substituted by
sigma_k(x), chain variables only. Order: grevlex, variables as listed (chain variables first, e last).
Engine: `f4_fp::f4` (degree-bounded F4, normal selection, Gebauer-Moeller criteria), max_degree 400,
no guessing (G = 0: T is the cost of ONE complete system, as in the paper's model). A run is complete
when F4 ends with 1 in the ideal or all pairs processed.

## 3. Measured quantities (counted units only)

Per cell: `field_ops` (F_p multiplications of the row reduction, `F4Report.field_ops`) per target,
`solving_degree_max`, `max_rows`, `max_cols`, input terms (all generators and S~ alone), number of
variables. T(E,m,L) := mean field_ops over the cell's complete targets. No wall-clock claim is made;
the wall-clock cap is only a censoring rule. This unit is not the tower engine's `f4_muladds`; the
calibration cells (section 5) report the ratio of the two on the bit-interval base, where both engines
run the same system with G = 0.

## 4. Grid (fixed now)

Frame `mont` (lo = A/3 in Weierstrass x, the Montgomery u-frame), coset g = w^gidx, w the smallest
primitive root, gidx = 0 unless stated. 6 targets per cell (seed 20261108 + 1000 m + d), cap 1200 s per
target. Rule: per (k, m, form) run d ascending and stop at the first cell with a capped target or an
out-of-memory exit (3.5 GB limit per process); larger d are recorded "not attempted" and the capped cell
is censored (never fitted).

* k = 20 (p = 1,048,573, p - 1 = 4 * 3^3 * 7 * 19 * 73), primary. m = 2: d in {3, 7, 9, 19, 21, 27, 57,
  63, 73, 133, 171, 189, 219, 399, 511, 513} (span 171x). m = 3 and m = 4: the same list ascending to the
  cap. form sym; form plain for m = 2, 3 with d <= 63. Cosets: gidx = 1, 2 at d = 27 and 63 (m = 2).
* k = 22 (d | 3 * 5^2 * 11 * 31 * 41): m = 2, d in {3, 5, 11, 15, 25, 31, 33, 41, 55, 75, 93}; m = 3, d
  ascending to the cap. k = 24: m = 2, d in {3, 23, 69, 89, 267}. k = 18: m = 2, d in {13, 71, 923}.
* Census (`subcensus`): |V|, columns (abscissae of V on the curve), base points, exact m-sum coverage of
  the order-r subgroup for m = 2, 3 (4 where the sum set fits) for every attempted cell with d <= 513.
* Calibration: bits base, t = 3, 4, 5, m = 2, k = 20, sym, G = 0 in this engine and
  `f4cost 20 bits t 0 6 <seed> 600 2 full 0 - 0` in the tower engine.

## 5. Estimator and verdict rules

For each (k, m, form) with at least 4 complete cells whose |V| spans at least 4x: ordinary least squares
of ln T on ln |V| over the complete cells with d >= 9 (the fit range; d = 3, 7 are boundary cells and are
reported in a sensitivity fit including them), slope s with a two-sided 95% t-interval (n - 2 degrees of
freedom). Cells that capped are listed and excluded. Fewer cells or smaller span: no fit; report
two-point local slopes and "undetermined".
Balanced-total exponent under the paper's model (omega = 2): with |V| = p^a, relation phase
p^(1 - (m-1)a) T(p^a) = p^(1 - (m-1)a + s a), linear algebra p^(2a); equal at a = 1/(m + 1 - s);
total exponent 2/(m + 1 - s) (if s < m + 1 - 4 = m - 3 this passes below 1/2). Generic p^(1/2) is the
comparison. (With s measured at fixed small p this extrapolation is derived, not measured.)
Thresholds (owner's, at the paper's fixed a): lever at m = 4 iff the fitted s < 0.5 with the upper 95%
bound < 0.5; lever at m = 3 iff s < 0 with the upper bound < 0. Re-balancing a for the measured s moves
the m = 4 break-even to s < 1 (2/(5 - s) < 1/2); this looser derived threshold is reported next to the
verdict, and does not replace the preregistered one. m = 2 is the tower comparison: tower-base T at the same G = 0 would be
~|V|^1.5 (L10: mpt 17,592 / 52,816 / 146,464 at t = 4, 5, 6 with guessing).

## 6. Predictions (written before data)

m = 2, sym: s = 1.5 (range 1.0 to 2.3). m = 3, sym: s = 3.5 (2.5 to 5). m = 4, sym: s >= 4.5 if
measurable (most likely capped beyond d = 9 or 21). Reasons: the single equation S~ must be combined
with an ideal of quotient dimension d^m (chain rules), so F4 works in spaces whose size grows like a
power of d^(m-1) at the solving degree; the m = 2 tower evidence is 1.5; T3 measured m = 3 at 2,300x
to 6,600x the m = 2 cost per relation. Expected outcome: no lever at m = 3 or 4 (owner's expectation
as well). plain vs sym: |s_plain - s_sym| < 0.5. Coset dependence: T within a factor 1.5 across
gidx. Calibration: the ratio field_ops / f4_muladds is within a factor 5 and constant to 30% across
t = 3, 4, 5. A failed prediction stays failed.

## 7. Correctness gates before measurement

Unit tests in `subgroup_semaev.rs` (summation polynomials vanish on sums, S~ equals S, the system is
inconsistent exactly when no tuple of V^m is a root, for sym and plain, m = 2, 3, bits base) and the
existing `f4cost` identity on the tower engine (an unchanged binary path).
