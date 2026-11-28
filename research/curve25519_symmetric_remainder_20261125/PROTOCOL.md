# Protocol: quadratic quotient auxiliaries for a symmetric Curve25519 relation system

**Frozen on 2026-11-25 before implementing either arm or observing any study row.**

Study id: `EXP-C25519-SYMREM-001`. Parent task revision:
`e9c13c712a`. The study uses native Rust and the Isogeny Labs `crypto`
repository only; no JavaScript is part of the instrument, analysis, or report.

## 1. Question and claim boundary

The active goal is a classical prime-field ECDLP algorithm with expected total
work `r^o(1)`. This is only a relation-system stage diagnostic. It asks whether
keeping the quotient in each multiplication modulo the cubic root polynomial
as explicit auxiliary variables changes the F4 staircase enough to justify a
larger experiment.

The permutation quotient itself is prior repository work. For three distinct
factor-base roots `w_i`, write

```text
q(X) = product_i (X - w_i) = X^3 - e1 X^2 + e2 X - e3.
```

Membership in `mu_d` is the single divisibility statement
`q(X) | X^d - 1`. The existing symmetric-tower instrument computes remainders
after eliminating the quotient. This study compares that direct presentation
with a split presentation in which each modular multiplication has an explicit
quotient. The proposed mechanism is a representation change, not a new
relation family.

A passing result is not evidence of a subexponential ECDLP algorithm. Relation
probability, relation count, setup, filtering, sparse linear algebra, individual
logarithms, and target recovery remain unpriced.

## 2. Fixed curve and factor base

- Curve: `curve25519_family_instance(20)`.
- Field and subgroup: `p = 1,048,573`, prime subgroup order `r = 131,203`.
- Montgomery parameter: `A = 926`; short-Weierstrass model
  `y^2 = x^3 + 63,700 x + 872,826`.
- Subgroup generator: `(349,837, 122)`.
- Montgomery-to-short-Weierstrass abscissa shift:
  `lo = A / 3 = 349,833 (mod p)`.
- Factor-base abscissae: `x = lo + g w`, where `g = 1` and
  `w in mu_d`; qualification uses `d in {7, 9, 19}`. Each `d` divides
  `p - 1`.
- Decomposition arity: `m = 3` distinct abscissae. Repeated-abscissa strata
  are explicitly out of this system and out of any completeness claim.

If `e1,e2,e3` are the elementary symmetric functions of the `w_i`, the
elementary symmetric functions of the curve abscissae are fixed as

```text
E1 = 3 lo + g e1
E2 = 3 lo^2 + 2 lo g e1 + g^2 e2
E3 = lo^3 + lo^2 g e1 + lo g^2 e2 + g^3 e3.
```

Both arms use the same symmetrized `S4` relation in `E1,E2,E3` and the target
abscissa. They therefore differ only in the encoding of `q | X^d - 1`.

## 3. Frozen arms

### D: direct remainder elimination (baseline)

Use binary square-and-multiply to compute `X^d mod q`, reducing symbolically
after every multiplication and eliminating the quotient. Set the final
remainder equal to `1`. This generalizes the existing 2-power symmetric-tower
construction to the odd orders in this protocol. No intermediate residue or
quotient variable remains in the final polynomial system.

### Q: explicit quotient auxiliaries (candidate)

Use the identical binary exponentiation schedule. For every modular
multiplication introduce a new residue
`R = r0 + r1 X + r2 X^2` and quotient `H = h0 + h1 X`, and impose
`A B - H q - R = 0` coefficient by coefficient. For
`A = a0 + a1 X + a2 X^2` and `B = b0 + b1 X + b2 X^2`, the five frozen
equations are

```text
a2 b2                                      - h1      = 0
a1 b2 + a2 b1                    - h0 + h1 e1       = 0
a0 b2 + a1 b1 + a2 b0 + h0 e1 - h1 e2 - r2        = 0
a0 b1 + a1 b0         - h0 e2 + h1 e3 - r1         = 0
a0 b0                         + h0 e3 - r0          = 0.
```

Thus every membership-gadget equation has total degree at most two. The final
residue is constrained to `(1,0,0)`. Constants and the initial residue `X` do
not receive unnecessary variables. All auxiliary equations and variables are
charged; none may be eliminated before F4.

The symmetrized `S4` polynomial stays identical in both arms. Circuitizing
`S4` is a different lead and is not allowed in this study.

## 4. Correctness gates

Correctness is decided before cost:

1. Unit tests evaluate every D and Q equation at the same planted roots,
   residues, and polynomial-division quotients for every frozen `d`.
2. For each frozen `d`, enumerate every unordered distinct triple from
   `mu_d`. The direct and quotient systems must accept exactly the same
   coefficient triples `(e1,e2,e3)`.
3. On a separate tiny prime curve, compare system consistency against
   exhaustive signed group truth for every subgroup target abscissa.
4. Every measured planted target uses three distinct factor-base abscissae,
   satisfies every specialized equation, and re-adds to the recorded target
   point after signs are restored.
5. Every solution extracted by the instrument, if extraction is feasible, is
   lifted and group-verified. A solver-consistency verdict alone is never
   reported as a relation.
6. Existing symmetric-tower snapshot tests must remain unchanged.

Because `X^d - 1` is square-free in these fields, a monic cubic divisor is
square-free. Repeated roots are not silently recovered by this chart. A future
complete oracle would need separately frozen multiplicity strata `[2,1]` and
`[3]`.

Any failed gate makes the instrument invalid. Failed rows and witnesses are
retained; the seed, cells, and thresholds are not changed to repair an outcome.

## 5. Qualification cells and accounting

- Engine: repository `f4_fp`, grevlex, release mode, one process and
  `RAYON_NUM_THREADS=1`; no learned trace, guessing, or cross-target reuse.
- Degree ceiling: 24. Per-system wall cap: 300 seconds.
- Seed: `20261125`.
- Per `(d, arm)`: 8 paired random subgroup targets followed by 4 paired planted
  distinct decompositions. The exact target scalar and affine point are stored.
- Primary unit: counted F4 `F_p` multiply-adds (`field_ops`).
- Secondary units: solving degree, last productive degree, maximum rows and
  columns, steps, variable/equation/monomial counts, specialization work,
  peak RSS, and wall time. Wall time is descriptive only.
- A timeout, memory kill, allocation failure, or degree-ceiling result is
  censored and retained with its partial counters; no work value is imputed.

Ratios are paired by exact target. At `d = 19`, compute the geometric mean of
`Q/D` `field_ops` over random targets completed by both arms and a descriptive
95% paired bootstrap interval with 10,000 resamples and seed `20261125`.

## 6. Frozen stop/go target

Arm Q qualifies for scaling only if all of the following hold:

1. every correctness gate passes, every planted row is exact, and invalid
   returned relations equal zero;
2. Q has no greater censoring fraction than D at any `d`;
3. at least 6 of the 8 random `d = 19` pairs complete in both arms;
4. the upper endpoint of the paired 95% interval for the geometric-mean
   `Q/D` `field_ops` ratio at `d = 19` is **below 0.50**;
5. Q's solving degree is at least one lower than D's on at least 6 of the 8
   random `d = 19` targets, with a censored Q target never counted as lower;
6. Q's measured specialization work and peak RSS are each at most 2 times D's
   on every completed `d = 19` pair.

This is the optimization goal: more than a 2x counted-work gain plus a robust
one-degree staircase reduction at the largest qualification cell. Missing any
condition closes this arm as a negative qualification. A smaller constant
factor may be documented but does not authorize larger cells.

## 7. Gated scaling stage

Only after Q passes section 6 may the unchanged instrument run
`d in {21, 27, 57}`, using the same seed policy, 8 random and 4 planted targets,
and resource caps. No implementation change is allowed between qualification
and scaling except a frozen addendum for a resource cap that is filed before
the affected row.

For descriptive scaling, fit `ln(field_ops)` on `ln(d)` per arm over all
completed cell geometric means, and fit `ln(Q/D)` on `ln(d)` over paired cells.
An advantage is called *strengthening* only if the upper two-sided 95% interval
on the ratio slope is below zero. This is a finite-ladder diagnostic, not an
asymptotic exponent claim.

No `m = 4` experiment is authorized here. It requires a separate protocol
whose minimum entry gate is section 6 plus a strengthening ratio. Any total
ECDLP claim additionally requires all omitted phases to have expected work
`r^o(1)`.

## 8. Evidence and prior-work receipt

Retain exact revision, Rust toolchain, host limits, commands, binary hash,
per-target JSONL, every censored/failed row, analyzer source, and hashes. The
analyzer and report must be native Rust/Typst or another non-JavaScript native
tool. Export `experiment.export/v1` planned/attempted/completed/excluded flow
and keep the result `[pcc:unverified]` unless separately registered and signed.

Repository sources checked before freeze:

- `research/pkm_symmetric_tower_20261026/README.md` and
  `examples/pkm_symmetric_tower.rs`: the existing root-polynomial remainder
  construction and its degree staircase;
- `research/pkm_symmetric_lead1_20261026/README.md`: the larger `m=3` and first
  `m=4` symmetric cells;
- `src/cryptanalysis/ic_framework/subgroup_semaev.rs`: per-summand subgroup
  chains and the Curve25519 exponent baseline;
- `src/cryptanalysis/vieta_coset.rs`: exact Vieta remainders and the repeated-
  root warning;
- `src/cryptanalysis/symmetrized_semaev.rs` and
  `src/cryptanalysis/koblitz_sym_s4.rs`: symmetric summation-polynomial code;
- `research/pkm_odd_tower_f4_20261114/`: odd/mixed per-summand towers.

The required research-KB wrapper was found, but its catalog executable and a
sibling `crypto-autoresearcher` checkout were unavailable in this executor.
Fallback exact, negative-result, application, and adjacent-mechanism searches
were run over the repository. They found the root-polynomial idea as prior art
but no explicit-quotient modular-multiplication arm. This is a search result,
not a novelty claim; cross-repository novelty remains unresolved.
