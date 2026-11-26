# Mode-to-group factorization identity

This note isolates the algebra implemented by `modes`, `child`,
`PacketTensor::entry`, and the product over radix groups in `evaluate_at`.
It is a producer proof target, not an independent or kernel-checked
refinement. `[pcc:unverified]`

## Direct deformation semantics

Give every old edge `e` two independent incidence bits: `x_e` at its left
endpoint and `y_e` at its right endpoint. For a packet `P`, let `s(e)` be an
edge's packet slot and define `[pcc:unverified]`

```text
K_P(x,y) = product_(e in P)
  (sum_(f in P) W(s(e),s(f))*y_f)                 when x_e=1,
  (1-sum_(f in P) W(s(e),s(f))*y_f)               when x_e=0.
```

The direct deformed contraction is

```text
D(W) = sum_(x,y)
  product_(left vertices v)  T_v(x on ports(v))
  product_(right vertices v) T_v(y on ports(v))
  product_(packets P) K_P(x,y).
```

At the zero deformation sample, `W` is the identity matrix and this reduces
to the original contraction. At a general sample it is the semantic target
that the mode/group construction must reproduce. The native
`direct_deformed_contraction` oracle implements this formula directly: it
does not call `modes`, `child`, `next_tuple`, `PacketTensor::entry`, or
`evaluate_at`.

## Packet polynomial

Fix one packet, one active coordinate, a Cauchy center `t`, and a sample `z`.
Let `x_r` be the bit on packet row `r` and `y_c` the bit on packet column `c`.
After the scalar Cauchy series is grouped into modes, write the selected row
sum as `[pcc:unverified]`

```text
S_r(y) = local_r(y) + sum_mu c_(mu,r) L_mu(y),
local_r(y) = sum_c W_local(r,c) y_c,
L_mu(y) = sum_c d_(mu,c) y_c.
```

Mode `mu` has exactly one packet, one target group containing its row
coefficients `c_(mu,r)`, one source group containing its column form
`L_mu`, and positive weight `w_mu`. The row factor selected by `x_r` is

```text
A_r + sigma_r sum_mu c_(mu,r) L_mu(y),
A_r = local_r(y)       and sigma_r =  1  when x_r=1,
A_r = 1-local_r(y)     and sigma_r = -1  when x_r=0.
```

Introduce formal variables `Y_mu`. Ordinary coefficient extraction gives
the finite identity `[pcc:unverified]`

```text
product_r (A_r + sigma_r sum_mu c_(mu,r) L_mu(y))

 = sum_beta
     coefficient_[Y^beta]
       product_r (A_r + sigma_r sum_mu c_(mu,r) Y_mu)
     * product_mu L_mu(y)^beta_mu.
```

The exponent of any `Y_mu` is at most the number of rows in its target group.
This is the `group_bounds` condition used by `next_tuple`.

## Correspondence with Rust

| Algebraic object | Rust object |
| --- | --- |
| `A_r` | `PacketTensor::entry` local `constants[row]` |
| `sigma_r c_(mu,r)` | signed `incoming.coefficients[row]` |
| `beta_mu` | `Incoming::exponent` and `Outgoing::exponent` |
| coefficient of `Y^beta` | depth-first row-choice traversal |
| `L_mu(y)` | `Outgoing::terms` evaluated on column bits |
| `L_mu(y)^beta_mu` | outgoing `ring.pow` |
| target/source placement | `mode.target` / `mode.source` branches in `child` |

For a fixed tuple `beta`, every old tensor vertex is placed in the single
child selected by its label digit, with its complete ordered port list.
Each packet child contains exactly the row or column ports whose old endpoint
has that group. Consequently the product of the group contractions enumerates
the transformed row/column port configurations once and multiplies the packet
coefficient and source-form factors from the displayed identity.
`[pcc:unverified]`

## Precision truncation

The scalar series control gives every mode of weight `w_mu` a coefficient
divisible by `2^w_mu`. A tuple contributes a multiple of

```text
2^(sum_mu beta_mu*w_mu).
```

It therefore vanishes modulo `2^p` once the tuple weight reaches `p`.
`next_tuple` enumerates exactly the nonnegative exponent vectors below this
weight, subject to the target-row degree bounds. The native enumeration
control now compares production traversal with an independent Cartesian
oracle over 83,970 exhaustive small-state configurations, plus two fixtures
with distinct packets and four physical group slots. Every admissible tuple
appears once and no inadmissible tuple appears. `WEIGHTED_TUPLE_ITERATOR.md`
states the general cached-state and restricted-odometer proof; independent or
kernel-checked refinement remains open.
`[pcc:unverified]`

## Quantified producer theorem

Assume a validated bipartite graph, a packing in which every old edge occurs
in exactly one packet slot, a working ring of precision `p`, and the scalar
Cauchy mode-series identity. Treat every old tensor entry as an arbitrary ring
function of its ordered ports; no Boolean, independence, or bounded-arity
assumption is needed here. Then non-strict `evaluate_at` equals the direct
deformed contraction `D(W)` modulo `2^p`. `[pcc:unverified]`

The producer proof is the following finite rearrangement.

1. Expand every packet factor `K_P` row by row. Same-group weights remain in
   the local affine term. Every crossing weight is its finite mode series;
   distributivity collects a global exponent vector `beta`.
2. For fixed `beta`, the coefficient of each mode is selected from rows in its
   unique target group, while its substituted linear form contains columns in
   its unique source group. The packet-polynomial identity above therefore
   assigns each factor to exactly one group.
3. Every old vertex has one label digit, so all of its ordered ports and their
   joint tensor entry remain in one group. This retains correlations among
   parallel ports; it never factors a vertex entry into per-port values.
4. Every left/right incidence belongs to exactly one old vertex and one packet
   vertex in that group. Isolated old and packet vertices remain as scalar
   factors. Consequently the assignment variables of different groups are
   disjoint after `beta` is fixed.
5. Finite distributivity (equivalently, Fubini for finite sums) turns the sum
   over all incidence assignments into the product of group contractions.
   Each factor is exactly the corresponding `child` graph: old tensors keep
   their complete port order, and the new packet tensor contains the local,
   incoming-target, and outgoing-source factors for that group.
6. Summing these products over every admissible `beta` reconstructs `D(W)`.
   The weighted iterator theorem visits each retained vector once. Every
   omitted vector has total 2-adic weight at least `p`, so its complete term is
   zero in the working ring.

The argument is insensitive to whether an old tensor is Boolean, equality, or
an immutable packet tensor: it only uses its entry as an opaque function of
the retained ordered port vector. Creation-precision lifting is therefore a
separate semantic premise, covered by its own native mutation controls.
`[pcc:unverified]`

This closes the producer-side algebraic composition of C1 items 1–3. What
remains is an independent or kernel-checked refinement showing that every
live Rust object and loop implements the stated finite rearrangement, plus the
separate interpolation, Hermite, and creation-precision refinements.
`[pcc:unverified]`

## Evidence and remaining gate

The exhaustive packet oracle independently checks the coefficient-extraction
identity on all 64 assignments of a mixed linear/quadratic fixture. A separate
global oracle compares the extracted modes-to-child product with `D(W)` in
1,536 configurations. It exhausts all 256 endpoint-table pairs for each of
three parallel-port label/sample cases and for each of the other three vertex
partitions: split-left, split-right, and split-both. The cases include local
and crossing groups, zero, unit, and nontrivial extension sample values, and
vertices on both sides of the group partition. The mutation gate additionally
detects both a removed mode and a wrong target group against the direct oracle.
End-to-end controls then recover the original direct tensor count at one and
three active levels and across two taper coordinates.

A second global control exhausts all five set partitions of three labeled
edges on each side, all 64 delta-basis tensor configurations for every
partition pair, and four full-label schedules: 6,400 cases. For a fixed graph
shape, both the direct and extracted contractions are separately linear in
each old tensor table. The products of per-vertex delta functions form a basis
of those tables, and the sum of all vertex arities is three on each side.
Thus the 64 basis configurations determine the factorization map for arbitrary
old tensor entries on every tested partition/label shape, rather than merely
sampling 64 truth tables. `[pcc:unverified]`

A third control covers four labeled edges. The 225 left/right partition pairs
collapse to 33 graph-shape orbits under simultaneous edge relabeling. For each
orbit and four label schedules, an independent direct packet-kernel map is
compared with a symbolic map built from production modes, tuple enumeration,
children, and packet entries. Each map contains all 256 old-incidence delta
basis coefficients, giving 33,792 exact coefficient comparisons. The symbolic
map enumerates each child assignment once and records its old-incidence mask,
rather than rerunning a complete contraction for every delta tensor. This
reduced the measured Rust 1.78 debug runtime from 536.50 seconds to 3.96
seconds while preserving the exact coefficient set. `[pcc:unverified]`

The same symbolic kernel scales to five edges. All 2,704 left/right partition
pairs collapse to 91 graph-shape orbits; four label schedules and all 1,024
incidence-basis coefficients give 372,736 exact comparisons. The complete
Rust 1.78 debug control passes in 51.71 seconds, below the 90-second scaling
target. `[pcc:unverified]`

These controls make the algebraic components load-bearing, but do not quantify
over every admitted graph or establish a typed refinement from every live Rust
child back to this polynomial. C1 items 1–3 close only after that independent
or kernel-checked link is supplied. `[pcc:unverified]`
