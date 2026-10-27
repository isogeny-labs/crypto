# Cell G6 protocol: Frobenius and 2-torsion structure at m = 9..20 (design, revision 2, written before any data)

Status: prepared, not run. **Revision 2 (2026-10-27, still before any data and before the freeze) amends revision 1 after an independent
review; every change is listed in section 14.** This file fixes the question, claim classes, arms, controls, metrics, sizes and decision rule
before the first data point. After `tools/experiment_protocol.py freeze` any change is a dated deviation appended to
section 14, never an edit.

Notation (as in G5): **n is the field degree** (toy n = 19, 23), **m is the number of summands (slots)**, E = #E(F_{2^n}), r the
prime subgroup order, T = (0, sqrt b) the 2-torsion point, sigma the absolute 2-Frobenius, iota the translation P -> P + T.
ECC2K-130 is a = 0, b = 1, n = 131 (ord_131(2) = 130). Nothing here discharges the m = 83 gate of AGENTS.md 8a; every transfer
to n = 131 is extrapolation (section 12).

## 1. Question and what the scout changed

Question (revision 2): does any Frobenius or 2-torsion structure change the **relation yield** (coverage, efficiency, dispersion, attempt
dependence) of an ordered-slot factor base at toy n, measured against the same instrument without the structure at matched slot-size
profile? Two halves of revision 1 were removed because the design cannot answer them: (a) the **cost** half (per-relation decomposition
cost): NI and UP have identical operation counts by construction, so cost differs only through yield; section 9 keeps an accounting
table and no cost claim; (b) the **"as m grows"** half: at fixed toy n, m moves the slot size (up to 15 points at m = 9, 3 at m = 12..14,
1 at m >= 16), so a slope over m is a slope over slot size. Section 4.5 adds cells that vary the slot size at fixed m; nothing here is a
statement about m as such, and nothing transfers to n = 131 (section 12).

The scout (read-only) corrected three premises that a naive cell would have inherited:

1. **"l is about n/m = 1..3 at toy n" holds only for the ordered-slot (rotated-partition) design.** The symmetric design (all
   summands from one subspace V, m! orderings quotiented) has Y = 2^(m l - n) / m!, so unit yield needs l = (n + log2 m!) / m,
   about 4.2 to 4.6 for every m in 9..20 at n = 23. G6 therefore uses the ordered-slot design, the one `n83_rotated_m10` uses
   (slot i takes the normal-basis conjugates with index = i mod m, dimensions summing to n). Claim S5 below is its arithmetic.
2. **Nothing in the repository solves a binary chained decomposition at m >= 7**; the SAT dispatch stops at m = 6, the algebraic
   engines at 64/128/512 unknowns, and the only m = 10 code is a never-reported n = 83 gate. A solver cell at m = 9..20 does not
   exist to be prepared. G6 is an exact-enumeration census, not a solver cell.
3. **The scout's "16384 patterns at m = 18" is a slip**: the number of intermediate-identity patterns is 2^(m-2), which is
   1024 at m = 12, 16384 at m = 16 and 65536 at m = 18 (unit test `sizes_anchors_and_degrees`).
4. **(Review, revision 2) In the class-(0,1) cells the NI factor base is an exact Koblitz digit chain, and its yield statistics are
   derived, not measured.** There slot i is {T, +-sigma^i P} (size 3), consecutive slots are Frobenius images, sigma acts as tau with
   tau^2 -+ tau + 2 = 0, and the structural identities of Z[tau] force collisions among the 3^L digit strings. The expected NI yield is
   an order of magnitude below the generic one as an algebraic fact (section 4.4). Revision 1 predicted EQUIVALENT within 10 percent;
   that prior is withdrawn.

Success criterion (CLAUDE.md): this cell cannot show an improvement over the index-calculus baseline. It runs no solver, so it
reports no distance to a matched rho (section 9). At most it settles which "holds for all m, measured at m <= 3" statements
survive a measurement at m >= 9 on toy groups, and which structure effects cannot be probed at toy n.

## 2. Claim classes (controlled-experiment skill)

| id | claim | class | how it is settled |
|---|---|---|---|
| S1 | x(P + T) = sqrt(b) / x(P) for P != T, so iota acts on the x-chain as x -> sqrt(b)/x | proved identity (3 lines: with l = (y + s)/x and s^2 = b, l^2 = y/x + x + a, so x3 = l^2 + l + a + x = s/x) | unit test over all points of four curves |
| S2 | for prime n the only iota-stable F_2-subspaces of F_{2^n} are {0, b^(1/4)} and the whole field | derived. Dimension 2 by hand (y/x0 would be a root of z^2+z+1, outside F_{2^n} for odd n). General d by the classical fact that an additive subgroup closed under inversion containing 1 is a subfield (Hua), not re-proved here. | unit test, exhaustive over all subspaces of dimension 1..3 at n = 7. Consequence: subspace slots cannot carry the 2^(m-1) stabiliser; only non-subspace slots can (G4 territory) |
| S3 | for b, a in F_2, x is on the curve iff x^2 is, so a normal-basis slot {conjugate indices i, i+m, ...} is the Frobenius image of slot i-1 whenever the index sets have the same shape; slot sizes then take only as many values as there are shapes and "difference patterns" | proved + tested | unit tests `on_curve_status_is_constant_...` and `ni_full_slots_are_frobenius_rotations_...` |
| S4 | iota quotient identity: with k iota-stable slots S = {+-P, +-(P+T)} and the plain slots {+-P}, H_iota[R] = 2^(k-1) (H_plain[R] + H_plain[R+T]) and H_iota[R] = H_iota[R+T] | proved identity, exact audit | `g6census iota-audit` (n = 13, m = 9..12, k up to m) and unit test |
| S6 | for a Frobenius chain (every slot {T} or {T, +-sigma^k P}, size-3 slots consecutive Frobenius images) the census statistics p, mu, disp are exact functions of the Z[tau] digit multiset: p = distinct/E, disp = (sum_R H^2)/3^L - mu, valid when ord(P) exceeds the norm bound of section 4.4 | derived exact computation, cross-checked against the group law (n = 13, both a) and against the actual NI builder (n = 19) and, at run time, on every chain instance | unit tests `tests/digits.rs`; `DIGIT_MODEL_DISAGREES` voids a cell |
| S5 | the ordered-slot Frobenius-interleaved design cannot reach the target factor-base size window at large m on toy n (sizes jump by whole bits per shared on-curve bit) | derived arithmetic (section 4.3) + measured exactly by the size census (no group sums) | `feasible_NI` against `feasible_U` per cell |
| C1 | at matched slot-size profile, the Frobenius-interleaved normal-basis design (NI) has the same realised yield efficiency e = P(H >= 1) / (1 - exp(-mu)) as a generic design (UP) | **two classes.** Chain instances (S6): exact computation, the answer is "no", by the amount section 4.4 states. All other instances: measured, paired, equivalence, **non-chain pairs only** | section 4.4 / section 8 |
| C2 | same for the dispersion Var(H)/mean(H) | as C1 | section 4.4 / section 8 |
| C3 | same for the orbit Fano factor of the n Frobenius-conjugate attempts (are the n conjugates of a target n independent attempts under one fixed base?) | measured, paired, equivalence, non-chain pairs. **Not a test of Proposition T**: the interleaved slot family is almost sigma-invariant by construction (V and sigma^-1 V share 10 of 12 slots at n = 23, m = 12), so a large NI value is expected whatever the truth | section 8 |
| C4 | the partition rule (interleaved NI, contiguous NC, random NR) does not matter inside a normal basis | measured, unpaired, descriptive | section 8 |
| C5 | the type-II optimal normal basis (n = 23) behaves like a random normal basis | single instance, descriptive | section 8 |
| SC | slot size, not m, drives the NI/UP log-ratio at toy n | measured, descriptive, cells n = 13, 17 at m = 9, 10 against n = 19, 23 (section 4.5) | `analyze` section 2b |
| X1 | anything at n = 131, or any solver cost (SAT, Groebner, degree of regularity) at m >= 9 | not measured; extrapolation | section 3 |

C1 to C3 are the decision claims. **Claim ids.** C1..C5, S1..S6 are local to G6. The audit of this campaign (`coverage.json`, `audit_20261026.json`) has its own C1..C5 with different
content; whenever a G6 claim is quoted outside this directory (atlas, report, PR) it carries the prefix `G6-` (G6-C1 .. G6-C5), and the audit's
claims are never quoted without `audit-`. S1, S3, S4, S6 are identities or derivations, not experiments; they are unit-tested and re-checked by the run, and
a violation is `INSTRUMENT_FAIL`, never a finding. "STRUCTURE changes yield" is not claimed anywhere: the statistical verdict label is
`YIELD_DIFFERS`, and chain instances are labelled `CHAIN_DERIVED`.

## 3. What the instrument is, and what it cannot reach

**Instrument.** For a slot system (S_1..S_m), S_i = all points with x in a coordinate subspace V_i (x = 0 gives T, every other
good x gives +-P), compute **H[R] = number of ordered tuples (P_1..P_m) in S_1 x .. x S_m with P_1 + .. + P_m = R, for every
group element R at once**: two meet-in-the-middle halves, every pair added, one counter per canonical point key (2x + [y > x^y]).
It is exact: no sampling, no solver, no model. The total over R equals the product of the slot sizes (checked every instance).
The ordered tuples count sign choices, so H(R) is the number of "decompositions" in the ordered-slot sense (the x-chain counts
x-tuples; they differ only on sporadic double sign patterns, not measured here).

**Cannot reach** (stated so no cell is read past its reach):

* Any solver cost: SAT time, Groebner degree of regularity, the 2^(m-2) pattern multiplicity, memory limits. Block A gives the
  encoding's exact sizes as functions of m (section 7) but measures no solving.
* The effect of m itself. At fixed toy n the slot size moves with m (section 1); section 4.5 separates the two only partly. The Frobenius
  collision mechanism of section 4.4 needs equal digits in consecutive slots, probability about 1/|S|^2 per window, so it is a **slot-size
  effect that is negligible at production slot size** (2^8..2^9 x-values per slot at n = 83).
* n = 131 (or n = 83) anything. Toy n = 19, 23 force 2^m toward r and l_i = 1..2; sporadic overlap ((2^m - 2)/r per solution per
  lag) is trustworthy only below m about log2(r) - 5 (about 12 at n = 19, about 15 at n = 23) and is not measured here (G5's business).
* m >= n (Burnside artefact: Frobenius fixes multisets when n | m); the cell list keeps m < n.
* Orbit-union / symmetric factor bases at m >= 9: at toy n a single full Frobenius orbit (2n points) is already oversized by
  2^8 (n = 23, m = 9) to 2^26 (m = 20), see `sizes::single_orbit_log2_yield`; at n = 131 the same base is undersized and needs
  K = 3 orbits at m = 20. The regime where orbit-union bases are sane cannot be enumerated.
* Frobenius-stable subspace slots (n = 31 has dimensions 5, 6; ECC2K-130 has none) and iota-stable non-subspace slots at large m
  inside the census (only the identity audit, k <= 12 at n = 13).
* The variant that excludes x = 0 (the 2-torsion leg T). With T excluded every dim-1 slot is empty with probability 1/2 and, for the
  Frobenius design, all dim-1 slots are empty or non-empty together (S3). Not run; the choice is a design flag worth a later cell.
* Targets restricted to the prime subgroup G (index 4 for a = 0). H is measured over the whole group; the G-stratum is a
  quarter of the targets and is not separated.
* m = 13, 15, 17, 19 (not sampled; no interpolation claimed) and the production n = 83 rotated gate.

## 4. Design

### 4.1 Slots and arms

Slot dimensions are the balanced split of n into m parts (first n mod m slots one larger): n = 23, m = 10 gives (3,3,3,2x7), the
exact analogue of the n = 83 gate's (9,9,9,8x7). Identical in every arm.

| arm | basis | partition | role |
|---|---|---|---|
| NI | random normal basis (seeded) | interleaved: slot i = {c : c = i mod m} | **structured arm** (the `n83_rotated_slots` rule) |
| UP | generic basis, redrawn per slot until each slot has **exactly the point count of the paired NI slot** | independent subspaces that together span F | **matched null**: same profile, hence same factor-base size and expected yield; no Frobenius structure |
| UP2 | as UP, independent seed | | A/A |
| DUP | UP with the slot pair of closest size made identical | | positive control (exchange symmetry) |
| NC | random normal basis | contiguous blocks | secondary |
| NR | random normal basis | random coordinate partition | secondary |
| ONB | type-II optimal normal basis (n = 23 only; unique up to order) | interleaved | single instance; paired with ONBUP (generic, ONB profile) |

Pairing: NI draw idx and UP draw idx have identical slot sizes slot by slot (asserted in a test). So the contrast is NI minus UP on
the same profile: every difference is the relation among the slots and inside them, not their sizes. n = 19 has no type-II ONB
(39 = 3 x 13), so ONB exists only at n = 23.

### 4.2 Why a profile-matched null is the right one

Because of S3 the sizes of NI slots are not independent: all full slots of one shape are Frobenius rotations of each other and
have the same size. A generic null drawn without matching would differ from NI in factor-base size before it differed in
structure, and "matched factor-base size and yield" would be false. The census records both populations (`feasible_NI`,
`feasible_U`) so the size effect is measured as its own result (S5) instead of being confounded into C1 to C3.

### 4.3 Hand arithmetic for S5 (to be confirmed by the census, not a result)

Let A = [beta is on the curve], B = [beta + beta^(2^m) is], with a = 0, b = 1, n = 23, E about 2^23. A dim-1 slot has size 1 + 2A, a
dim-2 slot {i, i+m} has size 1 + 2(2A + B). Then log2 mu = sum_i log2 |S_i| - 23:

| m | slots | A = 0 | A = 1 | in window [-10, 5]? |
|---|---|---|---|---|
| 12 | 11 dim-2, 1 dim-1 | -23 or -5.6 | 4.1 or 9.5 | yes for (A,B) = (0,1) and (1,0) |
| 14 | 9 dim-2, 5 dim-1 | -23 or -8.7 | 5.8 or 10.2 | (0,1) only (just inside) |
| 16 | 7 dim-2, 9 dim-1 | -23 or -11.9 | 7.5 or 10.9 | **none** |
| 18 | 5 dim-2, 13 dim-1 | <= -15 | 9.2 | **none** |
| 20 | 3 dim-2, 17 dim-1 | <= -18.2 | >= 10.9 | **none** |

A shared bit moves the factor-base size by 2^10 to 2^20 at once, while a generic design draws each slot independently. At toy
n and m >= 16 the production partition rule therefore cannot be matched in size to a target window; those cells are expected
`UNMATCHABLE` and give a size census only (the exact `feasible_NI` / `feasible_U` counts). This is a **toy-n artefact** (about 1.2 bits
per slot at n = 23): it says nothing about the production partition at n = 83 or 131, where slots hold 2^8..2^9 x-values. In total 13 of
the 22 original cells are enumerable (n = 23 at m <= 14; n = 19, a = 0 and 1, at m <= 12); the 9 `UNMATCHABLE`
cells (n = 23 m = 16, 18, 20; n = 19 a = 0, 1 at m = 14, 16, 18) are census-only, and informative NI-versus-UP statistics exist only at
m <= 14 (m = 14 at n = 23 only). At m = 9..11 dim-3 slots give more shape classes (up to about 16), so "four sizes per m" holds from m >= 12.

### 4.4 The Frobenius chain: an exact, derivable prediction (review finding B1)

In the class-(0,1) cells (A = 0, B = 1 in the notation of 4.3) every dim-2 slot has size 3 and every dim-1 slot has size 1, so the NI
slots are {T, +-sigma^i P_gamma} for i < L (L = number of dim-2 slots, consecutive Frobenius images) and {T} for the rest. On the group
sigma acts as the Koblitz endomorphism tau with tau^2 - t tau + 2 = 0 (t = -1 for a = 0, +1 for a = 1), so every tuple sums to
(sum_{k<L} e_k tau^k) P plus T-legs with e_k in {-1, 0, 1} (0 is the T leg). The parity of the number of T legs is a function of the
Z[tau] value, so distinct R correspond to distinct values of D = sum e_k tau^k, and the multiplicity of R is that of D, **provided no nonzero
difference of two digit vectors annihilates P**. An annihilating delta has norm at least ord(P), and N(delta) <= (2 sum_{k<L} 2^(k/2))^2
(about 4.6e4 at L = 11), so the model is exact whenever ord(P) exceeds that bound (checked per instance). Then, exactly:
p = distinct/E, mu = 3^L/E, disp = (sum_R H^2)/3^L - mu, eff = p/(1 - exp(-mu)).

Verified: the digit model reproduces the group-law sums of `curve::add` at n = 13 (543 distinct of 2187 tuples at L = 7, 257 of 729 at L = 6,
both a; `tests/digits.rs`), and the real NI builder at (n, m) = (19, 12) (class (0,1) draws, L = 7, 543 distinct, p and disp equal to the prediction
to 1e-9). Predictions at the cells (E taken as 2^n; the run uses the exact E; UP is the near-Poisson generic null, eff about 1 - mu/2, disp about 1):

| cell, class (0,1) | L | tuples 3^L | distinct | distinct / tuples | size-biased multiplicity | predicted disp (NI) | predicted eff NI vs UP |
|---|--:|--:|--:|--:|--:|--:|--:|
| n = 23, m = 12 | 11 | 177147 | 9711 | 0.0548 | 31.5 | 31.5 | 0.0554 vs 0.989 (x 1/17.9) |
| n = 23, m = 14 | 9 | 19683 | 2335 | 0.1186 | 13.9 | 13.9 | 0.1188 vs 0.999 (x 1/8.4) |
| n = 19, m = 12 | 7 | 2187 | 543 | 0.2483 | 6.1 | 6.1 | 0.2488 vs 0.998 (x 1/4.0) |

So where an NI draw is a chain, the NI/UP contrast is **not close to 1**: yield efficiency is lower by x4 to x18 and dispersion higher by x6 to
x31, an algebraic fact. Consequences fixed here, before data: (i) chain instances are an **exact-computation stratum**, reported as
`CHAIN_DERIVED` with the measured ratio, never as a test and never as evidence about structure at production slot size; (ii) the instrument
cross-check is `digits::predict_slots` against the measured p and disp of every chain instance, tolerance 1e-7 relative; a mismatch is
`DIGIT_MODEL_DISAGREES` and voids the cell (instrument or model failure); (iii) the statistical claims C1..C3 are evaluated on the non-chain
pairs only; a cell with fewer than 8 non-chain pairs reports `NO_STATISTICAL_PAIRS` plus its derived chain figures. The same mechanism
applies to any slot size, but needs equal digits in consecutive slots (probability about 1/|S|^2 per window), so it fades as |S| grows.

### 4.5 Slot size is confounded with m: the slot-size control cells

Cells n = 13 and n = 17 (a = 0) at m = 9 and m = 10 are added to n = 19, 23 at the same m. At fixed m the mean log2 slot size s = log2(T)/m
then takes at least four values (about 1.3, 1.7, 1.9, 2.3 bits of dimension per slot). `analyze` prints (section 2) the correlation r(m, s)
per curve, the slope of ln(NI/UP) over m and over s, and (section 2b) the per-m table across n. This **reduces, does not remove,** the
confound (two m values, four n values, non-chain pairs only, many cells expected to have few statistical pairs); the verdict on "m" is
therefore restricted to: "slope over m, with r(m, s) printed; where |r| > 0.9 no attribution". The cells are cheap (E = 2^13 and 2^17).

## 5. Controls (all on the same code path as the arms)

* **Positive control (detector sees an effect):** DUP versus UP, paired, on the dispersion. Exact exchange symmetry doubles the
  multiplicity of off-diagonal tuples, so Var/mean is about 2 - 1/s for a duplicated slot of size s (**1.667 for size 3**, the size at
  n = 23 m = 14 and n = 19 m = 12, i.e. ln 0.511 against the threshold ln 1.5 = 0.405: a thin margin) and not 2. `build_dup` takes the two
  closest-size slots and, among ties, the larger ones; if both have one point the control is a no-op and is **skipped** (`DUP_NOOP`).
  Required: verdict DIFFERENT with ln-ratio > ln 1.5. Otherwise the cell is `VOID(POSITIVE_CONTROL_NOT_DETECTED)`; fewer than 8 DUP pairs
  (no DUP, over the cap, no-op) is `VOID(POSITIVE_CONTROL_TOO_FEW)`, which is "the control is unavailable", **not** an instrument failure.
  The control is on the dispersion only and its effect (x1.67..x2) is far above the 10 percent margin: there is **no on-path positive control
  for eff or fano**, so an EQUIVALENT on those rests on the formula checks (`fano_tamper`, conservation) and on resolution (section 8).
* **Positive control for the orbit statistic:** every instance also reports `fano_tamper`, the Fano factor of the histogram with
  each orbit replaced by n copies of its first member (perfect dependence, value exactly n, unit-tested). Required: >= n/2.
* **A/A (noise floor):** UP2 versus UP, paired, on all three statistics. A verdict DIFFERENT voids the cell (`AA_DIFFERENT_*`).
* **Negative / null population:** UP is itself the null population drawn from the right distribution (generic subspaces of the
  same sizes); the secondary contrasts and the type-II ONB are placed against it, not against a model.
* **Instrument invariants, every instance:** conservation (sum of H = product of slot sizes), 256 planted random tuples all have
  H >= 1, exact agreement with brute force on tiny fixtures, rotation covariance H_{sigma V}[sigma R] = H_V[R], negation symmetry,
  orbit partition of the group, Fano of a sigma-stable base equals n (all unit tests; a run-time violation is `INSTRUMENT_FAIL`), and the
  digit-model cross-check of section 4.4 on every chain instance.

## 6. Frozen constants and selection rule

Seeds: `derive_seed(20261027, cell, arm, idx)`; draws are indexed, arms never share a stream. All constants are in `src/cell.rs` and
asserted by a test.

* Size census: 3000 draws per arm (NI, NC, NR, generic U). Only slot sizes are computed. The generic U arm exists only to measure
  `feasible_U`.
* Window: log2 mu in [max(-10, log2(1024/E)), 5], where mu = (product of slot sizes) / E. The lower floor guarantees at least
  1024 targets with H >= 1 in expectation; the upper limit keeps the tuple count at most 2^28 (hard cap, refused above).
* Selection: the first `k_eff` NI draws (by index) in the window. the largest `k_eff <= 32` (floor 8) with `cell_tuple_units(k_eff) * 2^median(log2 T) <= 2^33`, where `cell_tuple_units(k)` counts every enumerated
  instance: NI, UP, min(8,k) DUP, min(16,k) UP2, 2 for the ONB pair, plus 2 min(32,k) NC/NR instances weighted by 2^0.5 (they sit up to half a bit
  above the median). This is 180.5 units at k = 32 and 56.6 at k = 8 (revision 1 used 4.75 k, 152 and 38 units: it undercounted by 19 to 49 percent).
  The budget is exceeded only where the floor of 8 pairs binds (up to 56.6 x 2^28 = 2^33.8 tuples). Fewer than 8 feasible draws: the cell is `UNMATCHABLE`, nothing is enumerated, the sizes file
  and `meta` record why.
* Per selected index: NI, its profile-matched UP, UP2 (first min(16,k_eff)), DUP (first min(8,k_eff); skipped if over the cap).
* Secondary NC, NR: up to min(32, k_eff) draws whose log2 mu is within 0.5 of the median of the NI selection (unpaired).
* ONB (n = 23): one instance and its profile-matched partner, if within the cap.
* Cell list: n = 23, a = 0, m in {9,10,11,12,14,16,18,20}; n = 19, a in {0,1}, m in {9,10,11,12,14,16,18}; slot-size control n in {13, 17},
  a = 0, m in {9, 10}. 26 cells. a = 1 and n = 19 are replications (cofactor 2, smaller field), not the target.

## 7. Block A (exact formulas) and Block B (iota identity audit)

**Block A**, `g6census sizes`, no group. Chained-S3 SAT: variables (m-1)(3n^2 + 4n) + n(2m-1), AND gates 3n^2 (m-1), CNF clauses
9n^2 (m-1), native XOR rows 5n(m-1) + n, 2^(m-2) separate pattern instances; rotated DAG inputs n(4m-4) + (m-2); algebraic
chain unknowns (m-2)n + m l and equations (m-1)n; iota stabiliser 2^(m-1); S3 total degree 4 and Weil-descent bit-degree 3 at
every m (computed from the monomial list). Pinned by the source's own values (n = 83, m = 6: 105,908 variables; n = 83, m = 10: 2,996
rotated inputs). The AND/clause/XOR counts are read from the source, not built; rows with m > 6 are extrapolation (the source cap
stops at m = 6). Also tabulated: the symmetric design's l for unit yield, single-orbit yield and orbits needed, the number of
orbit classes log2(C(n+m-1, m)/n), the overlap probability.

**Block B**, `g6census iota-audit --n 13 --a A --m M --k K`, exact: S4 for (a, m, k) = (0, 9..12, k = m and k = 3) and (1, 9, 9).
It also prints the tuple and query counts: an iota-stable base costs 2^k times the plain tuples for the plain relations of the
T-coset of the target, i.e. it is the plain base queried at R and R + T. **Reading (a prior until the run):** after cofactor
projection R and R + T give the same relation, so this is a relabelling at best; the audit proves the equality, the cost ratio
shows it is a loss unless an iota-aware enumerator quotients it.

## 8. Metrics and decision rule (implemented exactly in `src/analyze.rs`)

Per instance: eff = P(H >= 1) / (1 - exp(-mu)); disp = Var(H) / mean(H) over all E group elements; fano = Var(K) / (n p (1-p))
with K = number of Frobenius conjugates of a point having H >= 1, over all size-n orbits (1 = independent attempts).

Contrast of ln-values. Paired (NI vs UP on idx; UP2 vs UP; DUP vs UP; at least 8 pairs, else TOO_FEW): d = mean difference,
se = sd / sqrt(N), df = N - 1. Welch (NC, NR against the UP rows in the same mu window; ONB descriptive): df = smaller N minus 1.

* **EQUIVALENT** if d + t_tost(df) se < ln 1.10 and d - t_tost(df) se > -ln 1.10 (TOST, margin 10 percent, one-sided 95 percent t).
* **DIFFERENT** if |d| > t_diff(df) se **and** |d| > ln 1.10. t_diff is the exact Student-t quantile for a family-wise 0.05 over **338 tests
  (26 cells x 13 classified contrasts per cell: primary 3, A/A 3, DUP 1, NC 3, NR 3)**, per-test alpha 1.48e-4, rounded up to one decimal
  (df 7: 7.5 [exact 7.412], 10: 6.0 [5.916], 15: 5.1 [5.035], 20: 4.7 [4.668], 30: 4.4 [4.342], 60: 4.1 [4.053]; unit test). Revision 1 used a
  198-test family whose description did not match the tests computed and a df-7 value (6.8) below the exact 6.803/6.814.
* Degenerate case: if se = 0 (every pair difference identical) the verdict is EQUIVALENT if |d| < ln 1.10, else DIFFERENT, bypassing the t tables
  (implemented in `classify`; undocumented in revision 1).
* otherwise **UNRESOLVED**, with the minimum detectable ratio of this rule: exp(max(ln 1.10, (t_diff(df) + 0.9) se)), the effect that the
  rule detects with 80 percent power (0.9 rounds up the t quantile 0.896 at df 7). Revision 1 printed exp(2.8 se), an unadjusted normal figure that
  understated the resolvable effect by about 1.8x to 2.7x in log-ratio.
* **Multiplicity.** DIFFERENT is Bonferroni-gated; EQUIVALENT (TOST, one-sided 95 percent) is **not corrected across cells**: with up to
  26 x 2 statistical equivalence claims, about 2.6 false equivalence claims are expected if all true effects sat just outside the margin. Read
  NO_CHANGE_WITHIN_DELTA per cell, not as a family.

Cell validity: conservation, planted tuples, tamper >= n/2, digit-model agreement on chain instances, DUP detected, no A/A DIFFERENT; else VOID with
the reasons. **Stratification.** NI/UP pairs whose NI instance is a chain (section 4.4) are removed from the tests and reported as `CHAIN_DERIVED`
with the mean ratio. Cell verdict (valid cells) on the non-chain pairs: yield claim = YIELD_DIFFERS(eff, disp) if either is DIFFERENT,
NO_CHANGE_WITHIN_DELTA if both EQUIVALENT, else UNRESOLVED(MDE); with fewer than 8 non-chain pairs, NO_STATISTICAL_PAIRS(k). Attempt claim from
fano alone, same stratum. Across cells (section 2 of the summary): the slope of ln(NI/UP) over m and over the mean slot size s per curve and
statistic, plus r(m, s), **descriptive only** (at least 4 cells with statistical pairs) and not attributable to m (section 4.5).
**Resolution at the mu extremes.** For mu << 1, eff is about 1 - mu/2 and disp - 1 is O(mu) for any non-degenerate structure; for mu >> 1,
eff is about 1 and fano is NaN when p = 1. A +-10 percent margin on ln(eff) is therefore nearly automatic outside mu in about [0.3, 3]
(DUP at mu = 1 changes eff by only about 23 percent). A cell whose selected median log2 mu < -3 carries LOW_MU and one with median log2 mu > 2
carries HIGH_MU: EQUIVALENT there is weak evidence, whatever its interval. Per-pair differences pool the mu classes that coexist at one m (NI
sizes take few values) and the paired t on such a bimodal sample is not stratified by class; this is a known weakness, not corrected.

No post-hoc threshold change. A cell that fails its controls is reported as VOID, not rerun with altered parameters.

## 9. Accounting per covered target, the 0 / 0.1 / 1 GAE band (AGENTS.md 5a) and the rho distance

**The cost half of the question is dropped (revision 2).** NI and UP have the same slot profile, hence the same table sizes, queries q and build
additions per instance, and the same mu; their price per tuple is identical and a difference can arise only through p. Revision 1's
"cost per relation" columns were therefore identical for the two arms except the last, and counted structural duplicate tuples (about 95 percent
of the NI tuples at n = 23 m = 12 class (0,1), section 4.4) as separate relations. `analyze` now prints, as **accounting only**, the cost per
covered target (a target with H >= 1, i.e. a distinct relation) in group-addition equivalents with native work (lookups and inserts) at
kappa in {0, 0.1, 1}: amortised q (1 + kappa) / p, single target (adds_build + q + kappa (q + stored)) / p. Their NI/UP order is the order of
1/p, kappa-invariant by construction, and is not a cost finding. None is compared with rho.
**Distance to rho:** not applicable, no solver runs and no complete discrete log is produced (success criterion needs a measured
solver improvement; this cell cannot supply one, and no leaderboard or benchmarks row follows from it).

## 10. Resolution

Statistics are exact over the whole group (about 2^23 targets), so the sampling noise inside one instance is small (relative
noise of P(H >= 1) about 1/sqrt(1024) = 3 percent at the lower window floor, much smaller at mu about 1); the noise that matters
is between instances, and the paired design cancels the profile. MDE (section 8, the effect the rule detects with 80 percent power) is printed per contrast. With 8 pairs (high-mu cells) the
interval is wide and UNRESOLVED is the expected outcome; the protocol says so in advance. The type-II ONB is a single instance:
descriptive only. 26 cells, 13 classified contrasts per cell: the Bonferroni critical values pay for DIFFERENT; EQUIVALENT is not corrected (section 8).

## 11. Runtime (a projection, not a measurement)

Per tuple: one affine addition (a few random log-table lookups) plus one counter increment, about 60 to 250 ns (unmeasured). A
typical matchable n = 23 cell: about 150 instances x 2^23..2^24 tuples = 1.3e9..2.6e9 tuples, 2 to 11 CPU minutes, under about
250 MB per job (field tables 100 MB, orbit map 33 MB, counters 64 MB). UNMATCHABLE cells cost seconds (the size census); the four n = 13, 17
cells cost seconds to minutes. **Bound per cell:** `cell_tuple_units(k_eff) * 2^median(log2 T) <= 2^33` except where the floor of 8 pairs binds,
then at most 56.6 x 2^28 = 2^33.8 tuples (about 1.5e10). **Worst case:** 22 cells of n >= 19 each at that bound = 3.3e11 tuples = 5.6 CPU-h at
60 ns, 23 CPU-h at 250 ns (revision 1 stated 15 CPU-h; its rule undercounted instances). The driver's printed WORST (every instance at the 2^28 cap,
154 instances) is a far larger unreachable ceiling and is not this figure. The typical total of 1 to 2 CPU-h is unmeasured; the high-mu n = 23 cells
may dominate it. The driver's pre-flight times an **n = 19** cell, which fits in cache, and projects the n = 23 cells (100 MB of tables) from it: the
gate is not conservative for n = 23, so the director applies a margin (at least x3 on ns per tuple) when freezing `G6_MAX_CPU_MIN` and the deadline.
Block B costs a few seconds.

## 12. Variable ledger and threats

| variable | handling |
|---|---|
| instrument | one code path for every arm; unit tests against brute force |
| host, layout, order | outputs are deterministic functions of the seed and the commit; wall time is not a metric (pre-flight only) |
| seeds | derived per (cell, arm, idx); paired arms share idx, not streams |
| spot preemption | a cell is rerun whole from the same command; no state to resume |
| analyst | the decision rule is the program; rows are not inspected before it runs |
| slot-size profile | matched exactly by construction for NI/UP; NC, NR matched only by mu window and flagged unpaired |
| x = 0 leg (T) | included in every slot, as in a subspace factor base; the excluded variant is not run |
| cofactor / G stratum | whole-group targets; G-stratum not separated |
| m close to log2 r | not a statistic here; sporadic-overlap statements are not made |
| slot size | confounded with m at fixed n (section 4.5); the slot-size control cells vary it at fixed m; chain collisions are a slot-size effect |
| toy n | S5: the production rule is unmatchable at m >= 16 (n = 23) and m >= 14 (n = 19), by the same arithmetic; extrapolation to n = 131 is not supported |

## 13. Predictions (derived or prior; none is a result)

* **Derived (section 4.4):** wherever NI is a chain (class (0,1): n = 23 m = 12, 14; n = 19 m = 12, and any other cell and size profile in which
  every slot has 1 or 3 points), NI and UP **differ by x4 to x18 in yield efficiency and x6 to x31 in dispersion**. The verdict there will be
  `CHAIN_DERIVED`; it is an algebraic consequence of tau^2 -+ tau + 2 = 0 acting on tiny slots, not a finding about structure and not evidence
  about production slot size. Revision 1's prior (EQUIVALENT within 10 percent, dispersion near 1) and its ordering of suspects (instrument
  first) are withdrawn for these cells: here a large difference is the expected value, and the instrument is tested by the digit-model
  agreement, not by the size of the difference.
* **Prior, non-chain NI draws** (profiles with bigger or irregular slots, m = 9..11 and the control cells): EQUIVALENT is plausible but weak
  at the mu extremes (section 8). If a non-chain stratum reports YIELD_DIFFERS, check in this order: the digit-model and conservation
  checks, whether mean log2 mu of NI and UP are equal (profile), the pooling of mu classes, then structure.
* C3: large for NI by construction (the slot family is almost sigma-invariant); a value does not test Proposition T.
* S5: UNMATCHABLE at n = 23 for m >= 16 (toy-n artefact). S4 holds.
* SC: the NI/UP log-ratio tracks slot size, not m.

## 14. Deviation log

Revision 2 (2026-10-27, after an independent review, still before any data; the protocol is not yet frozen with
`tools/experiment_protocol.py freeze` because the checkpoint gate blocks runs in the preparation container). Changes, all pre-data:

1. Question: cost half and "as m grows" attribution dropped (sections 1, 9).
2. Claims: S6 added; C1/C2 split into an exact-computation stratum (chain instances) and a statistical stratum (non-chain); G6- prefix for ids (section 2).
3. Section 13 priors corrected; verdict label STRUCTURE_CHANGES_YIELD renamed YIELD_DIFFERS; new labels CHAIN_DERIVED, NO_STATISTICAL_PAIRS.
4. Instrument: new module `digits` and the run-time cross-check (rows gain `pred_p`, `pred_disp`); `DIGIT_MODEL_DISAGREES` void reason.
5. Slot-size control cells (n = 13, 17 at m = 9, 10): cell list 22 to 26; `analyze` prints r(m, s), slopes over s, and the fixed-m table.
6. Decision rule: t_diff family 198 to 338 tests with exact quantiles rounded up; MDE redefined as the rule's 80 percent power effect; se = 0 branch
   documented; HIGH_MU flag added; EQUIVALENT multiplicity stated.
7. Positive control: DUP expectation 2 - 1/s; tie-break to larger slots; `DUP_NOOP` skip.
8. Budget rule: `k_eff` from `cell_tuple_units` (all instances, NC/NR window factor); worst-case runtime restated (23 CPU-h at 250 ns).
9. Pricing: amortised and single-target per-relation columns replaced by per-covered-target accounting.
Unchanged by this revision and still open: `protocol.json` is not written or frozen; no `experiment.export/v1` step in the run card; arm labels are not
masked in analysis; the pipeline has been run only on tiny fixtures, never on real-size scratch output.
