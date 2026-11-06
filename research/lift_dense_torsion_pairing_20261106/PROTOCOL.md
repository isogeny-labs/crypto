# Protocol: 2-torsion pairing of lift-dense lines (frozen before data)

2026-11-06. Follow-up to
[`lift_dense_factor_base_20261027`](../lift_dense_factor_base_20261027/README.md)
(the Oct 27 lead). This file is committed before any computation in this
directory runs. Changes after the freeze go in a dated amendment below.
Nothing here has been run yet.

## Question

E : y² + xy = x³ + ax² + b over F_{2^N}, with N = d·n and q = 2^d. Let T = (0, √b) be the
rational 2-torsion point. Then x(P + T) = √b / x(P). The Oct 27 lead builds all-lift
lines u·F_q with Tr_{N/d}(u) = Tr_{N/d}(√b/u) = 0 and Tr(a) = 0, and counts
up to 2^n relations per decomposition trial. Neither its yield nor its
u-count looks at T.

The claim to check came from another session and is an unverified
assertion:

1. if u ∈ b^{1/4}·F_q, translation by T maps the line to itself;
2. the line's columns then come in pairs;
3. the gain there is about 2×, not 2^n;
4. the u-count used for costs should exclude such u.

## Algebra (to be checked numerically, not assumed)

- x ↦ √b/x maps u·F_q onto (√b/u)·F_q. The two lines are equal iff
  √b/u² ∈ F_q, that is, iff u ∈ b^{1/4}·F_q^*. Over F_q^*-scaling that is
  exactly one line, L_T = b^{1/4}·F_q.
- L_T is valid (both traces zero) iff Tr_{N/d}(b^{1/4}) = 0. If u = b^{1/4}c
  with c ∈ F_q, both traces equal Tr_{N/d}(b^{1/4})·c^{±1}.
- In the odd-order part of E(F_q), P and P + T project to the same element.
  Columns are the logarithms of [2^e]P with 2^e ‖ #E, so they merge in pairs
  x ↔ √b/x. The fixed abscissa x₀ = b^{1/4} has 2P₀ = T, which puts P₀ in the
  2-power torsion: its column is O and is dropped.
- A relation P₁ + … + P_m = R stays a relation when +T is applied to an even
  number of its points, so its row in the odd part repeats. On L_T a raw
  relation is one of up to 2^{m−1} copies of the same row.

## Instrument

This is a new std-only Rust crate, `pairing/`, in this directory. It does not
call or copy the Oct 27 code. Its field arithmetic is carry-less
multiplication with reduction, using no log tables. The same polynomials as
the Oct 27 `FIELDS` table are used, so that study's integer (a, b, u) values
can be read. Irreducibility is checked independently, by x^{2^N} ≡ x and
gcd(x^{2^{N/p}} − x, f) = 1 for each prime p | N. Each P + T is computed by
group addition and compared with the formula √b/x.

A second binary, `c2pnb`, has its own multi-limb GF(2^N) arithmetic and runs
on the real c2pnb208w1 and c2pnb368w1 parameters. It reads the Oct 27
parameter dump, `reports/run-20261027T0356Z/c2pnb_input_binary-curve-params.txt`.

Every count is exact enumeration, except the relation rows, which use seeded
targets. Seeds come from a fixed string per cell, using SplitMix64. There is
no timing, so no isolation or benchmark claim is made.

## Cells and arms

The cells (N, d) are the Oct 27 line cells: (12,6), (16,8), (20,10), (18,9)
with n = 2; (15,5), (21,7), (18,6) with n = 3; and (16,4), (20,5) with n = 4.

Each curve has Tr(a) = 0 (a drawn uniformly with absolute trace 0).

| arm | b | lines measured | role |
|:--|:--|:--|:--|
| `prior` | the Oct 27 Amendment 3 curves (`lines_amend3/rows.jsonl`) | their recorded twisted `u` | does the Oct 27 study's chosen line lie on L_T? |
| `tstable` | uniform b, conditioned on Tr_{N/d}(b^{1/4}) = 0; 4 curves per cell | L_T, plus up to 3 other valid lines on the same curve (within-curve negative control) | effect |
| `generic` | uniform b, conditioned on Tr_{N/d}(b^{1/4}) ≠ 0; 4 curves per cell | up to 4 valid lines (none can be L_T) | negative control |
| `std` | the standard line F_q on every `generic` curve | F_q (not all-lift) | baseline, as in Oct 27 |
| `poscontrol` | b ∈ F_q^*; 2 curves per cell | F_q, which is T-stable because √b ∈ F_q (the FGHR / Koblitz case) | positive control: the detector must see the pairing on a line known to be T-stable |

Not every n has every arm: with n = 2 a valid line exists only when √b ∈ F_q,
and then it is L_T = F_q. When an arm cannot occur in a cell, it is reported
as "not applicable" in that cell.

## Measured quantities

- k is the number of lifting abscissae on the line.
- D is the number of distinct columns: x([2^e]P) over the line's lifting P,
  excluding O.
- On each line, every lifting x is tested for whether √b/x lies on the line
  and lifts.
- Relation rows use m = n, with targets R uniform in E(F_q) \ {O, T}:
  2000 per curve for n = 2, 400 for n = 3, 100 for n = 4. All lines on a
  curve share the same targets.
  - **raw** is the number of distinct multisets {P_i} of factor-base points
    (both signs) with ΣP_i = R.
  - **rows** is the number of distinct multisets {[2^e]P_i}.
  - Both are summed over the targets.
- The relations-per-column figure is ρ = rows / (targets · D).

## Predictions (decision rules)

**P1 (exact).** On every curve in every arm, the number of valid T-stable
lines is 1 if Tr_{N/d}(b^{1/4}) = 0 and 0 otherwise. When it is 1, it equals
b^{1/4}·F_q. A T-stable line means one with √b/u² ∈ F_q. Any mismatch
falsifies P1.

**P2 (exact on T-stable lines).** On every T-stable all-lift line:

- √b/x lies on the line and lifts for every lifting x;
- x(P + T) = √b/x(P) by group addition;
- D ≤ (k − 1)/2, with equality on at least 90 % of these lines. Fewer
  columns can come only from chance merges by other 2-power torsion.

On `poscontrol` (F_q, not all-lift), D ≤ (k − [x₀ lifts])/2.

**P2′ (negative control).** On non-T-stable lines (`tstable` within-curve
controls, `generic`, `std`), D ≥ k − 2 on at least 95 % of lines.

**P3 (rows).** Pool raw/rows per cell and arm.

- On T-stable lines it lies in [0.6·2^{n−1}, 2^{n−1}].
- On non-T-stable lines it is ≤ 1.05.
- `poscontrol` must show raw/rows ≥ 1.3. If it does not, the row detector
  is broken and P3 is void.

**P4 (relations per column, the cost-relevant ratio).** Prediction from
the algebra above and the Oct 27 yields:

- ρ(L_T)/ρ(std) ≈ 2, with band [1.3, 3];
- ρ(generic valid line)/ρ(std) ≈ 2^{n−1}, with band [0.6·2^{n−1}, 1.6·2^{n−1}].

This is the claim "about 2×, not 2^n". It is a ratio of measured exact
counts on toy fields, not an ECDLP cost.

**P5 (c2pnb, exact).**

- Reproduce the Oct 27 u-counts: 180 for c2pnb208w1 and 330 for c2pnb368w1,
  with u ∈ F_{2^16}.
- u and uc with c ∈ F_{2^4}^* give the same line, so those counts are 12
  and 22 lines.
- The T-stable u among them number either 0 or 15 (one line). There are 15
  exactly when Tr_{N/Q}(b^{1/4}) = 0.
- The corrected count is 180 − 15s and 330 − 15s, where s ∈ {0, 1} is
  computed, not predicted.
- Also record whether √b ∈ F_Q, which would make the standard line T-stable.

**P6 (prior arm).** The number of Oct 27 Amendment 3 twisted-line rows
whose u is T-stable is reported exactly. No prediction is made beyond
"≈ 1/q of curves". For n = 2, every existing twisted line is L_T, and Oct 27
found none.

## What would change the Oct 27 lead

- If P1–P4 hold, the Oct 27 README needs a correction:
  - its 2^n per-trial yield holds as a raw tuple count on every valid line;
  - on L_T, rows per column fall to about 2× standard;
  - the u-count used for costs should drop the L_T u's.

  How large that change is depends on P5 and P6. If L_T is a small share of
  the valid lines, the Oct 27 cost-model rows change little.
- If P3 or P4 fails, the "about 2×" claim is wrong as stated.

## Controls, A/A, analyst

- **Controls.** `poscontrol` is the positive control. `generic`, `std` and
  the within-curve lines are the negative controls.
- **A/A.** The full run executes twice from a clean build. The two
  `rows.jsonl` files must be byte-identical (sha256).
- **Analyst.** The analyst is the same session as the producer. All
  verdicts come from the summarizer's coded rules above, not from reading
  the data.

## Amendments

**A1 (2026-11-06, after data; it changes no decision rule).** These are
recorded deviations, not new rules.

1. The first run (`reports/run-20261106T0229Z/`) counted rows as signed
   column vectors after ± cancellation. The frozen definition above is
   distinct multisets {[2^e]P_i}. Commit `62a40ce4cf` adds the frozen
   definition as `rows`, and every verdict uses it. The vector count is kept
   as `rows_vec` and is reported only as post hoc. The first run is kept
   unchanged and marked superseded.
2. The positive control `poscontrol` (F_q with b ∈ F_q) has 0 relations at
   n = 3 and 4. With Tr(a) = 0 the curve is F_{2^N}-isomorphic to
   y² + xy = x³ + b over F_q, so its F_q-abscissae lie in two subgroups of
   size about q. This is a design fault found after the data. Under the
   rule above, P3 is void in those cells.
3. Post-hoc tables, which are not decision rules:
   - ρ_vec ratios, comparing L_T with the other valid lines in the same cell;
   - P2 equality split by n.
