# Round 7 (RR): Riemann–Roch norm-divisibility system under F4

## Prediction

Written 2026-11-04T05:55:06Z (`date -u`), before any RR system was generated
or solved. [pcc:unverified]

Predicted F4 solving degree D_RR (msolve 0.6.5, grevlex, 1 thread), against
the cherry tree T on the same planted instances:

| cell | D_T (reference) | predicted D_RR |
|---|---|---|
| m3 t4 | 6 | 6 |
| m3 t5 | 6 | 6–7 |
| m3 t6 | ≥ 7 | 7 |
| m5 t4 | ≥ 9 (cut off) | 8–9 (likely cut off) |
| m5 t5 | — | ≥ 9 (cut off) |

Reasoning: the inputs are quadratic, but N ≈ (2m−1)t + 2m is 2–3× the T
variable count, and the tower levels couple through U at every level, so I
expect no D gain at m = 3 and at most one degree at m = 5, with wider
matrices than T. (m3 t3 has no planted instance in `../instances.txt`; it is
added only if a planted t = 3 instance can be built from the same data.)

Kill criterion (from `../round7_beyond/DIRECTIONS.md`, rank 1):

- **killed** if D_RR ≥ D_T in every comparable cell, **or** if D_RR's m-slope
  (m3 → m5 at t = 4) is ≥ 1.5 per summand;
- **trade, not improvement** if D_RR < D_T somewhere but the widest matrix is
  larger in every such cell;
- **improvement** only if D_RR and the widest matrix are both smaller than T's
  in at least one m ≥ 5 cell.

## Construction (what was actually solved)

Generator [`genrr.sing`](genrr.sing) (Singular 4.3.2; sha256 `3254c832…df459e2`),
inputs hashed in [`SYSTEMS.sha256`](SYSTEMS.sha256). Odd arity m = 2h − 1,
curve and planted targets from `../instances.txt` (m = 3) and
`../instances_r3.txt` (m = 5), k = 0. Coordinates x = g·W, so the planted
roots of U are the w_i ∈ μ_{2^t}. [pcc:unverified]

- Unknowns: A(X) = X^h + Σ a_i X^i (monic, deg h); C(X) = Σ c_i X^i
  (deg ≤ h − 2); U(W) = W^m + Σ u_i W^i (monic, deg m); tower remainders
  V_j = Σ r<j>_i W^i (deg ≤ m − 1) and quotients Q_j.
- Norm identity: coefficients in X of
  A(X)² − f(X)C(X)² − (X − x_R)·g^m U(X/g), f = X³ + aX + b.
- Target sign: A(x_R) − y_R·C(x_R) = 0, so div(A + Cy) = (−R) + ΣP_i − (m+1)O
  and ΣP_i = R exactly (no ±R ambiguity, unlike the x-only T arm).
- Membership (prototype's quotient/remainder tower, `prime_one_shot_20261029`
  README "The selected construction"): V_0 = W, V_{j−1}² = Q_j·U + V_j
  (j = 1..t), V_t = 1, i.e. W^{2^t} ≡ 1 mod U. Levels where W^{2^j} still has
  degree < m are left unexpanded with no variables (the same convention as
  `../gen.sing` uses for arm T's remainder chain), so the prototype's all-zero
  first-level quotients are dropped.
- Every input equation has degree 2.
- Variant **RRs**: the prototype's chart witnesses added — s·U(x_R/g) − 1
  (target not a residual abscissa) and a Bezout witness S·g^mU(X/g) + T·C = 1
  (U, C coprime). Squarefreeness of U needs no witness because X^d − c is
  separable.
- Planted assignment: y_i = √f(x_i) via `factorize`; y_R one root; all 2^m
  summand sign patterns tried with the chord law, exactly one gives the sum
  (x_R, y_R) in every cell. A, C by linear interpolation through
  {P_i, −R} (unique), U = Π(W − w_i), V_j and Q_j by division. Every equation
  was evaluated at this point and vanished (generator aborts otherwise);
  A² − fC² = (X − x_R)·g^mU(X/g) was also checked as a polynomial identity.
  [pcc:unverified]

Run: `msolve -g 2 -t 1 -v 2` (msolve 0.6.5), one job at a time, `ulimit -v
1572864` (1.5 GiB), 1200 s cap (`../run.sh` with TLIMIT/VLIMIT), parsed with
`../parse.sh`. Records: [`rows.tsv`](rows.tsv), logs in [`logs/`](logs/)
(hashes in `logs/SHA256SUMS`). m3 t3: no planted instance exists in
`../instances.txt`, so it was not run. m3 t7 was generated (planted check
passes) but not run.

**Positive control** [pcc:unverified]: the prototype's F₁₇ m3 t4 remainder
system (`continuation/canonical/outputs/m3_t4_remainder.ms`, all 33 variables
unknown) gives quotient dimension **28** under msolve 0.6.5, matching the 28
recorded point models (`logs/control/`). (The unformatted `inputs/` copy with
nested parentheses parses to the unit ideal, so only the canonical `outputs/`
file is a valid control.)

## Results

T references from `../rows.tsv`, round 2 and round 6 (T had 6–13.5 GiB, RR
1.5 GiB). "cap" = msolve died at the 1.5 GiB address-space limit (exit 139,
hash-table/allocation failure); D for those cells is the highest degree
reached, a **lower bound**. [pcc:unverified]

| m | t | N_RR (N_T) | input deg RR (T) | D_RR | widest RR | wall s | qdim RR (T) | D_T | widest T |
|--:|--:|---|---|---|--:|--:|---|---|--:|
| 3 | 4 | 18 (13) | 2 (4) | **4** | 2,801 | 0.47 | 1 (3) | 6 | 7,077 |
| 3 | 4 RRs | 22 | 2 | 4 | 4,276 | 0.90 | 1 | | |
| 3 | 5 | 23 (16) | 2 (4) | **5** | 31,870 | 353 | 1 (3) | 6 | 23,720 |
| 3 | 5 RRs | 27 | 2 | ≥ 5 (cap) | ≥ 47,383 | 70 | — | | |
| 3 | 6 | 28 (19) | 2 (4) | ≥ 5 (cap) | ≥ 46,999 | 348 | — | ≥ 7 | ≥ 57,152 |
| 5 | 4 | 23 (23) | 2 (4) | ≥ 6 (cap; degree-6 round started, 123,917 × 144,917) | ≥ 144,917 | 75 | — | ≥ 9 (cut off) | ≥ 999,374 |
| 5 | 4 RRs | 30 | 2 | ≥ 5 (cap) | ≥ 94,540 | 13 | — | | |
| 5 | 5 | 32 (28) | 2 (4) | ≥ 5 (cap) | ≥ 67,668 | 14 | — | ≥ 9 (6 GiB cut) | ≥ 124,757 |
| 5 | 5 RRs | 39 | 2 | ≥ 5 (cap) | ≥ 14,469 | 9 | — | | |

Secondary diagnostic (not the reference instrument): with `-m 1000` (pairs
per matrix capped) m5 t4 completes several degree-6 rounds with new elements
and then hits the cap again (D ≥ 6, 84,571 columns); m3 t6 still dies in
degree 5 (`logs/pairs1000/`). [pcc:unverified]

Quotient dimension: in both completed cells it is 1, the planted relation
only (the y_R equation removes T's ±R/x-only multiplicity). No nonreduced or
nonrational extra solutions appeared, so the chart saturations were not needed
to read D; RRs at m3 t4 reproduces D = 4 with 1.5× wider matrices.
[pcc:unverified]

Per-degree width at m5 t4 (same instance): T's widest matrix by degree is
1,042 (deg 5), 2,442 (deg 6), 6,718 (deg 7), 148,679 (deg 8); RR's is 27,674
(deg 5) and ≥ 144,917 (deg 6). At m = 5 RR's matrices are 25–60× wider than
T's at equal degree. [pcc:unverified]

## Reading against the kill criterion

- **Not killed by "D_RR ≥ D_T in every cell"**: D_RR < D_T at m3 t4 (4 vs 6)
  and m3 t5 (5 vs 6). [pcc:unverified]
- **m3 t4 is a strict win on both D and width** (4 vs 6; 2,801 vs 7,077;
  0.47 s vs 3.7 s). At m3 t5 D is lower but the widest matrix is 1.34× wider
  (wall 353 s vs 544 s): a trade. RR's depth slope at m = 3 is +1 per level
  (4 → 5), steeper than T's (6 → 6 → ≥ 7), so the D advantage is shrinking
  with t. [pcc:unverified]
- **m-slope**: m3 t4 → m5 t4 is ≥ (6 − 4)/2 = 1 per summand; the kill
  threshold (≥ 1.5, i.e. D_RR(m5 t4) ≥ 7) is **undecided** under 1.5 GiB.
  [pcc:unverified]
- **Improvement criterion (m ≥ 5 cell with smaller D and narrower widest
  matrix)**: **undecided**. D_RR(m5 t4) ≥ 6 is compatible with D_RR < 9 ≤ D_T,
  but RR is already 25–60× wider than T at equal degree, so a narrower widest
  matrix than T's would require D_RR ≤ 7 and an RR degree-7 width below T's
  ≈ 10⁶. [pcc:unverified]
- Against the prediction: D_RR was lower than predicted at m = 3 (4, 5 vs
  predicted 6, 6–7); the m = 5 cells could not be decided at this memory cap.
  [pcc:unverified]

Deciding cell: m5 t4 RR at ≥ 6–12 GiB (the round-6/round-7-memory host
class), same command. Status: **live, undecided at m = 5; trade at m3 t5;
win at m3 t4.** [pcc:unverified]
