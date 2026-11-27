# Frozen bounded relation panel

Frozen 2026-11-25 before the first panel run. The fixture is public and
deterministic: `p=127`, `E:y²=x³+x+26`, `#E(F_p)=131`, `P` the first
lexicographically sorted nonidentity point, and `Q=[37]P` (37 is withheld
from the relation solver and used only for validation). This panel measures
mechanisms and exact costs at a small field; it is not a scaling inference.

## Arms and common base

- `small_x`: first two target-curve x values with rational lifts.
- `pplus1_torus`: a norm-one group of order 128; a fixed coset of its order-8
  subgroup, traced to eight x values through `z+z^-1`.
- `unrelated_aux`: `y²=x³+x`, order 128; a fixed coset of an order-8 subgroup,
  projected to x values.
- `twist_raw`: target's quadratic twist by the first nonsquare `d`; a coset
  of an order-5 subgroup, with x normalized to the `d*y²=f(x)` model. The
  expected target lift count is zero; preserve this censored arm.
- `twist_shifted`: the same twist coset, followed by the first field shift
  `x -> x+c` producing at least two target-curve x values. Charge the shift
  scan and record `c`.

For each eligible arm take the first **two** distinct target-liftable x values
in numeric order and both signs: exactly two matrix columns and four signed
points. Keep the full raw fiber cardinality and target-lift count. The
truncation is a matched-size relation-yield instrument; it is not the full
PKM tower polynomial system and does not establish its solve cost.

## Relation and cost protocol

Use `m=2`. Precompute all 16 signed pair sums, then query the same 1024
`(a,b)` pairs from Python `random.Random(20261125)` for every arm. Every
hit yields `aP+bQ=F_i+F_j`, checked with independent point addition.
Retain all hits, duplicate rows, unsuccessful queries, and a row's origin.
Track rank of the coefficient system in `(k, log_P F_1, log_P F_2)` over
`F_131`; stop at the first full-rank system and validate `k=37`. If 1024
queries fail to determine `k`, report censoring. Charge construction,
pair-table work, query point operations, modular row reductions, wall and
CPU times separately. Record the full stream and all attempted work.

As a same-target control, run plain three-way Pollard rho for 16 deterministic
seeds and verify each recovered scalar. Its implementation and units are
explicitly distinguished from optimized native negation-map rho. The primary
comparison is exact qualifying relation/rank yield at equal base size; no
performance crossover follows from this small Python panel.

## Falsifiers and decision rule

Reject an arm before the solver when its smooth subgroup or fiber fails
exact order/cardinality checks or fewer than two target x values lift. For
remaining arms, require correct point identities and full-rank recovery.
Compare independent rank per *complete* counted work and cold wall/CPU, not
raw roots alone. A further native implementation requires a sound relation
map and a credible fully charged cost case beyond this bounded panel.

## Sources

- Semaev 2004/031, summation-polynomial relation condition.
- Petit–Kosters–Messeng, PKC 2016 §§3.1–3.5, published factor-base fibers
  and partial cost expression.
- `docs/ic/factor-base-encyclopedia/README.md` §§0.1, 0.2, A5: existing
  Kummer, Dickson torus and isogeny arms and prior measured boundaries.
- Research KB revision `750797a540fb4c38837c67b12df84d5b`, refreshed
  2026-11-25T03:19:48Z, sources `source:601058f8defc3b0c82b523447ce8819c`
  and `source:362976dae6e255e22be8c13a8c7a2e2f`. The KB predates some
  current commits; this checkout's source and primary paper govern.
