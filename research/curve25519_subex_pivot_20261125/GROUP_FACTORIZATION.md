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

## Evidence and remaining gate

The exhaustive packet oracle independently checks the coefficient-extraction
identity on all 64 assignments of a mixed linear/quadratic fixture. A separate
global oracle compares the extracted modes-to-child product with `D(W)` for
all 256 pairs of arity-two Boolean tables in each of three label/sample cases,
768 configurations total. The cases include local and crossing groups, zero,
unit, and nontrivial extension sample values, and correlated parallel ports.
The mutation gate additionally detects both a removed mode and a wrong target
group against the direct oracle. End-to-end controls then recover the original
direct tensor count at one and three active levels and across two taper
coordinates.

These controls make the algebraic components load-bearing, but do not quantify
over every admitted graph or establish a typed refinement from every live Rust
child back to this polynomial. C1 items 1–3 close only after that independent
or kernel-checked link is supplied. `[pcc:unverified]`
