# Two-summand cycle closure: exact boundary

The canonical pair-table family law used “the first graph cycle” as a proxy
for the first eliminable relation dependency.  That proxy is false without a
parity/sign qualification over the odd prime scalar fields used by ECDLP.

## Result

Let `B` be the edge-by-vertex matrix of accepted two-summand relations.  In the
unsigned model every edge row has two `+1` entries.  Over a field of
characteristic other than two, a connected component `C` has

```text
rank B_C = |V(C)| - 1   if C is bipartite,
rank B_C = |V(C)|       if C is non-bipartite.
```

Equivalently, its left-nullity is `|E|-|V|+1` in the bipartite case and
`|E|-|V|` in the non-bipartite case.  Therefore:

- a lone even cycle (and a pair of parallel edges, a length-two cycle) gives a
  dependency;
- a lone odd cycle does not; the triangle incidence determinant is `2`, hence
  is invertible modulo every odd prime;
- a self-pair `P+P` is a loop row with coefficient `2`, not a dependency, over
  an odd scalar field;
- in a signed/folded relation graph, “even” is replaced by “balanced” for the
  route's actual endpoint signs;
- a graph dependency recovers the target logarithm only when its weighted
  target coefficient is nonzero.

The relevant random variable is consequently the first **useful balanced
circuit with nonzero target projection**, not the first graph cycle.

## Consequence for the family law

Write `rho(K)` for the accepted-edge stopping law on `K = F/(2k)` columns.  The
pair-table accounting identity is

```text
ops(F) = F^2/(4t) + 2 c #E rho(F/(2k)) / F^2.
```

The former expression `F^2/(4t) + c#E/(2kF)` is the specialization
`rho(K)=K/2`; it is not a bound.  If `rho(K)=Theta(K^beta)`, minimizing gives
`F*=Theta(#E^(1/(4-beta)))` and normalized exponent
`2/(4-beta)-1/2`.  Thus the reported `r^(1/6)` follows only at `beta=1`.
The graph algebra does not itself prove `beta=1`, concentration near `K/2`, or
nonzero target projection. [pcc:unverified]

## Evidence and limits

- Zaslavsky, *Signed graphs*, Discrete Applied Mathematics 4 (1982), Theorem
  8B.1 and Corollary 8B.3, gives the signed-incidence dependence matroid over
  fields of characteristic not two.  Primary PDF SHA-256:
  `fa66ad267542e6b7cdbeef4706f7ca61d2f64fc8a478ed414d3428d4a3a12a4d`.
- [`CycleClosure.lean`](../../papers/ic-negative-results/lean/IcNegative/CycleClosure.lean)
  kernel-checks the triangle and loop countermodels, square and parallel-edge
  positive controls, and the target-projection bridge on Lean 4.34.1.
- [`verify.py`](verify.py) independently enumerates the three finite matrices
  over `F_3` using only Python's standard library.

This closes the deterministic incidence-algebra part of
`GAP-RP-CYCLE-CLOSURE-LAW`.  It does not establish a natural edge ensemble,
its stopping-time distribution, or the distribution of target projections.
Those remain open and must be measured on complete, non-stopped relation
streams.  [pcc:unverified]

## Reproduce

```sh
python3 research/ic_cycle_closure_20261125/verify.py
(cd papers/ic-negative-results/lean && lake build IcNegative.CycleClosure)
tools/lean/proof-gate.sh papers/ic-negative-results/lean
```
