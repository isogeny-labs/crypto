# L5 — direct membership enumeration at k=50 (2026-11-24)

**Lead:** L5 in `crypto-autoresearcher/docs/open-leads-20261123.md`
(parametrized-base solver question; target cell `|B| ≈ (30p)^{1/3} ≈ 50`).

**Question:** the k=50 Semaev cell is beyond a 15-minute generic-F4 budget
(L13: default window target did not finish in 900 s). Is a longer budget
required, or is there a cheaper method?

**Answer:** for `m = 3` at this cell, generic F4 is not required. Enumerating the
explicit x-set costs `O(k^3)` field operations and answers the decision/count
problem in **under 0.2 s per family**:

| family | k | reachable target x-coords | min ordered point triples | x-space S4 solutions at that target | enumeration s | count s |
|---|---:|---:|---:|---:|---:|---:|
| window | 50 | 2099 | 12 | 204 | 0.057 | 0.105 |
| pullback | 50 | 2099 | 12 | 219 | 0.058 | 0.106 |

This is not a Gröbner-basis run: it builds no basis, reports no `d_reg`, and
makes no `msolve` ideal-degree claim.

## Method

The base is an explicit set of `k` x-coordinates on `E: y^2 = x^3 + x + 1` over
`F_4099`. For `m = 3`:

1. Enumerate all `k^3` ordered point triples `(P_i, P_j, P_l)`, accumulate the
   target x-coordinate `x(P_i+P_j+P_l)` and its ordered triple count. This also
   gives the exact point-decomposition count for every reachable target.
2. Pick the target with the fewest ordered point triples (12 here).
3. Count x-space solutions by evaluating `S4(a,b,c,xQ)` for all `k^3` x-triples
   and counting zeros. This is the same S4 used by the msolve route.

`S4 = 0` is necessary but not sufficient for a point decomposition, so the
x-space count (204/219) is larger than the point-triple count (12). The direct
enumeration returns both: the exact point triples and the S4 x-space count.

## Cross-check at k=30

The same S4 evaluator at k=30 reproduces the L25 control value:
`xQ = 3106` gives 30 x-space solutions for both window and pullback, matching
the msolve ideal degree 30 reported in PR #5150. The standalone k=30 run also
reports min-point-count targets (1 point triple) with 43/82 x-space solutions,
showing again that S4 has extraneous x-solutions.

## Exact commands

```sh
python3 scripts/l5_direct_enumeration.py 50
python3 scripts/l5_direct_enumeration.py 30   # cross-check / context
```

Raw output: `research_notes/l5_direct_enumeration_k50_20261124/l5_enum_k50.json`
and `l5_enum_k30.json`.

## What this establishes

- At `p = 4099`, `k = 50`, `m = 3`, one curve, the Semaev cell can be decided
  and counted directly in under 0.2 s per family; the >900 s F4 timeout is not a
  fundamental barrier for this cell.
- The direct method also returns the exact ordered point-decomposition count
  (12 for the minimum target), which F4 does not directly give.
- A cheaper method exists for L5's stated `m = 3`, `|B| ≈ 50` cell, so buying a
  longer F4 budget is not necessary to answer the existence/count question.

## What this does not establish

- Not a solver for `m >= 4`: the enumeration is `O(k^m)`, so the advantage is
  specific to `m = 3`.
- Not a relation-yield, descent, or full IC cost claim; only the Semaev cell's
  decision/count step is measured.
- Not a `d_reg` measurement and not a comparison to `msolve`'s ideal degree.
- One prime, one curve, one seed. No cryptographic-`p` claim.
- Python wall times on a single core; no optimized implementation.

## Calibration / units

- Exact arithmetic over `F_4099`; no probabilistic success rate.
- Wall seconds, Python 3.12, single core, `sympy.sqrt_mod` only for point
  enumeration (0.057 s includes it).
- `f4_fp` `solving_degree` remains calibrated 8/8 (PR #5134); this record makes
  no `f4_fp` claim.
