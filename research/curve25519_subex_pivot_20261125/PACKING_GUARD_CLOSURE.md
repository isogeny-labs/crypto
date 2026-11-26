# Packing closure from the source guards

The source guards already imply every packing capacity and group-balance bound
needed for recursive closure. No symmetry assumption on the child graph is
required. This is a producer-side algebraic proof, not an independent formal
refinement or a practical Curve25519 attack. `[pcc:unverified]`

## Definitions

At one active node write

```text
R       label radix (64 in the strict schedule),
q=R^a   vertex-label alphabet,
D=q^2   ordered pair-label classes,
L=2^a   packet length,
M       source edges,
delta   global degree cap,
c_gamma size of pair-label class gamma.
```

The active `Setup` branch checks

```text
M >= 2 D^2 delta,
M >= L D,
2L <= delta.
```

The child induction supplies maximum source degree at most `delta`.
`[pcc:unverified]`

## Conditional-expectation variance

Scale the conditional objective by `D=q^2`. At the final labeling it is

```text
D sum_gamma c_gamma^2.
```

Initially, an ordered edge pair with no shared endpoint contributes 1. A pair
sharing at least one endpoint contributes at most `D`. The number of ordered
pairs charged through endpoints is at most

```text
sum_vertices degree(v)^2 <= delta * sum_vertices degree(v) = 2M delta.
```

Greedy first-minimum conditional expectation never increases the objective, so

```text
D sum_gamma c_gamma^2 - M^2 <= 2 M delta D.
```

Equivalently,

```text
sum_gamma (c_gamma-M/D)^2 <= 2 M delta.
```

This is the inequality checked independently by the schema-3 packing
provenance verifier. Its quadratic replay is separate from the optimized
incidence-Gram producer. `[pcc:unverified]`

## Per-class cap

The strongest source guard gives

```text
2 M delta <= M^2/D^2.
```

If one class had `c_gamma>2M/D`, its single squared deviation from `M/D`
would exceed `M^2/D^2`, contradicting the complete variance budget. Hence

```text
c_gamma D <= 2M                       (1)
```

for every class. This implication uses no child symmetry, orbit transitivity,
or probabilistic concentration. `[pcc:unverified]`

## Padding and packet count

Let `c=max_gamma c_gamma` and let

```text
height = L ceil(c/L).
```

Using (1) and `LD<=M`,

```text
height D <= (c+L)D <= 2M+LD <=3M.    (2)
```

For a packet pattern with `j` unequal digit pairs, the source constructor uses
`copies=2^(a-j)`, so at most `height/copies` packets survive. Summed across all
unordered patterns,

```text
K L <= height * sum_patterns 2^j
    =  height * (R+2 binom(R,2))^a
    =  height * R^(2a)
    =  height D
    <= 3M.                            (3)
```

Empty-packet suppression can only decrease `K`. Thus the padding and packet
capacity checks follow from the source guards. `[pcc:unverified]`

## Group loads and shrink

For one coordinate and radix digit, either side's endpoint load is the sum of
exactly `D/R` pair classes. By (1),

```text
one-side group load <= (D/R)(2M/D) = 2M/R.
```

This is stronger than the implementation's required `3M/R`. A group child has
the selected row and column endpoints, hence

```text
M_child <= 4M/R = M/16 <= M/8        when R=64.   (4)
```

Old child vertices retain their parent degree. Every auxiliary packet vertex
has at most `2L` ports. The third guard therefore preserves maximum degree
`delta`. `[pcc:unverified]`

## Recursive induction

At the root, the complete-multigraph family supplies a graph satisfying the
degree and source guards. For any tuple/group child:

1. (4) supplies strict shrink and the degree argument preserves `delta`;
2. if the next `Setup::parameters` call returns `None`, production uses the
   exact fallback and no further packing claim is needed;
3. if it returns parameters, it has rechecked `M>=2D^2delta`, `M>=LD`, and
   `2L<=delta`, so the same proof applies again.

Thus old/packet children do not need to remain in the symmetric root family.
The source guards themselves close packing, capacity, group balance, shrink,
and degree at every active depth. What remains outside this lemma is the
factorization/refinement theorem, the symbolic cost composition's independent
formalization, and the external prime-field contraction theorem.
`[pcc:unverified]`

## Executable binding

The schema-3 packing verifier now reports `maximum_class` and
`source_guard_closure_admitted`. When the serialized edge skeleton supplies a
degree cap and satisfies both `M>=2D^2delta` and `M>=LD`, it additionally
requires the derived `maximum_class*D<=2M` and per-side `2M/R` group caps.
Small allocated controls report the guard as unavailable rather than pretending
to be strict asymptotic witnesses. `[pcc:unverified]`
