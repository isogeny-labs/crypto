# Paired weighted-symmetry transport — 2026-11-02

Status: derived identical-generator construction, implemented transport and
separate driver integration; syntax/AST checks. Numerical trials: zero.

## Source and exact reference

Prime coefficient field p>3; source Z_a A-A Z_b=GK^T, with validated nonzero
metric D and DA=A^T D. The reference is the published metric-pivot `move`:
apply row swap P, then the same column swap P. Existing frozen sources remain
unchanged. The source of the metric is the public weighted feature compiler.
Selection validates its inverse metric; permutations preserve that identity.
No extra validation inversion or field extension is introduced here.

[Bostan et al. (2017), equation (2) and Section 1](https://cs.uwaterloo.ca/~eschost/publications/BoJeMoSc17.pdf)
provides the established Sylvester displacement and cyclic-shift conventions.
The particular update below is obtained by composing the existing local
row/column transports; no new displacement theory is claimed.

## Composition with signs fixed by the reference

For swap indices i!=j, set u=e_i-e_j, P=I-u u^T. For B=PAP:

    Z_a B-B Z_b
      = P(GK^T)P + (Z_a P-P Z_a) A P + P A (Z_b P-P Z_b).

The commutators are

    Z_a P-P Z_a = u (Z_a^T u)^T - (Z_a u) u^T,
    Z_b P-P Z_b = u (Z_b^T u)^T - (Z_b u) u^T.

Hence a width-(w+4) generator pair is

    G' = [ P G, u, -Z_a u, P A u, -P A Z_b u ],
    K' = [ P K, P A^T Z_a^T u, P A^T u, Z_b^T u, u ].

These are exactly the columns, including ordering and signs, returned by
`swap_columns(swap_rows(A,i,j),i,j)`. In particular the final right correction
is negative P A Z_b u, not its opposite. No cancellation or numerical
approximation is used in this equality.

## Reusing selected columns

Let c_i=Ae_i and c_j=Ae_j. Then

    Au = c_i-c_j,
    (A^T u)_l = d_l (d_i^{-1}(c_i)_l-d_j^{-1}(c_j)_l).

Thus only A^T Z_a^T u and A Z_b u require structural source applications.
Both are applied to the original width-w source, rather than applying the
second-stage queries to the width-(w+2) row-swapped source. The final P is an
ordinary vector permutation.

`move` initializes its cache with the selected pivot columns. Each chosen
pivot column is therefore present when it is moved to the front; at most the
other swap column is missing. That missing column is queried once. After a
swap, every cached column transforms by

    new column at index P(t) = P * old column at index t.

Both cache keys and vector coordinates must change. This preserves the
selected columns across a two-coordinate pivot and can preserve a newly
queried column for its second swap. Cache size starts at cut (one or two)
and grows by at most one per nonidentity swap: at most four columns here.
Identity swaps do nothing and issue no queries. No zero-prefix inference or
zero-column-query elision is assumed.

The one-use selection is tied to source identity/signature, as in the
reference. Internal cache/metric provenance and unchanged source generators
are preconditions; this is not a public cache certificate. Input-metric
checks made by the selector are not repeated inside this transport. Standalone
calls to the private `_swap` are outside the driver integration contract.

## Derived per-swap accounting

Let h be missing cache columns: h in {0,1} in this integrated path.

| Quantity | Sequential reference | Paired transport |
|---|---:|---:|
| Full source applications | 4 | 2+h |
| Source widths used | w,w,w+2,w+2 | w for all |
| Transpose / forward applications | 2 / 2 | 1 / (1+h) |
| Output width | w+4 | w+4 |
| Field products outside queries | 4 | 3n+4 |
| Field additions outside queries | 2n+2 | 4n+1 |

The new products are four wrapped-shift products plus 3n for A^T u. Additions
are one for -1 and n each for Au, the inner weighted difference, -Z_a u and
-P A Z_b u. Permutations, cache copying, canonical checks, dictionary work and
index search have additional host costs. No inversion is needed here.

The preserved fused kernel records 4w+8 native products per successful
width-w source application. Sequential total is 16w+48; paired total is
(2+h)(4w+8). For h=1 the derived saving is 4w+24 native products; for h=0 it
is 8w+32. The tradeoff outside queries is +3n field products and +(2n-1)
field additions. These are different units. Operand sizes, encoding cost,
cache allocation and runtime require measurement; no net speed claim follows.
For two swaps, apply the formula again at its actual input width w+4.

## Complete driver integration and preservation

`driver_paired.py` is byte-identical to the complete weighted-symmetry driver
apart from replacing its transport import. `paired.move` returns the same
source/cut/columns/inverse/metric/order interface and emits its own accurate
transport receipts. In exact arithmetic the generator pair after each swap
matches the reference pair. Hence deterministic compression and subsequent
pivot choices receive identical generator data, under the preserved primitive
correctness assumptions. Source event paths, query records, counters and
transport traces intentionally differ; they are not byte-equality targets.

The static audit reverses the single import substitution and compares complete
ASTs. It binds the reference transports, selector and fused kernel hashes.
It does not execute the new transport or establish its correctness dynamically.
All numerical, corruption-control, heap and performance trials remain zero.

## Validation obligations and next step

Retain all upstream cohort cases and unchanged identity controls. Compare
both final generator arrays, not merely their reconstructed matrix. Include
scalar and pair pivots, n=2, boundary/adjacent/distant swaps, nonuniform metrics,
h=0 and h=1, second-swap cache reuse, zero queried columns, changed/forged caches,
source mutation, reciprocal/metric corruption and consumed selections.
Independently recompute every query width and native/field ledger; compare
full driver solutions/witnesses and root equations. Charge failed controls,
cache lifetimes and heap bytes. Existing cost admission forbids new trials.

Next derive provenance-bound metric views to avoid repeated inverse checks,
and assess whether selector zero-prefix information can safely remove missing
zero-column queries without changing the frozen search cohort. Neither saving
is included in this component's accounting.
