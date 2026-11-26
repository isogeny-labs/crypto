# Coefficient-map refinement lemma

This note states the reusable algebra behind the triangular native verifier.
It is a producer-side proof target, not an independently checked or
kernel-checked theorem. `[pcc:unverified]`

## Incidence basis

For an old graph with `M` edges, give each edge two independent Boolean
incidences: one at its left endpoint and one at its right endpoint. Let

```text
I = I_left disjoint-union I_right,   |I| = 2M.
```

A mask `q in {0,1}^I` selects one product of delta-table basis functions for
the old vertices. Because every old tensor appears linearly in a tensor
contraction, the values on these masks determine the contraction for arbitrary
old tensor tables. `[pcc:unverified]`

Fix one active coordinate, Cauchy sample, packet packing, and working ring
`R=(Z/2^p Z)[U]/f`. Define three functions from incidence masks to `R`.

## Map D: direct packet kernel

For packet `P`, row incidence `x_e`, column incidence `y_f`, and kernel weight
`W`, set

```text
K_P(q) = product_(e in P)
  (sum_(f in P) W(s(e),s(f))*y_f)      when x_e=1,
  (1-sum_(f in P) W(s(e),s(f))*y_f)    when x_e=0.

D(q) = product_P K_P(q).
```

This is `direct_packet_basis_map`. It uses no generated modes, exponent tuple,
child graph, or packet tensor. `[pcc:unverified]`

## Map E: independent mode expansion

For a packet row `e`, retain `W(s(e),s(f))` directly when the row and column
labels have the same active digit. For crossing columns, replace the weight by
the generated finite series

```text
sum_mu c_(mu,e) * d_(mu,f),
```

where `mu` has this packet and target row group, `c` is its row coefficient,
and `d` is its source-form coefficient. Evaluate the resulting row factor at
the incidence mask and multiply over rows and packets. Call the result `E(q)`.

This is `mode_expanded_basis_map`. It consumes the generated mode records but
does not call `child`, `next_tuple`, or `PacketTensor::entry`.
`[pcc:unverified]`

## Map H: production child product

For each retained exponent vector `beta` and group `g`, construct the
production child. Enumerate the child edges once. Each child edge is incident
to one retained old vertex port, hence names a unique bit of `I`; the child
assignment therefore has a unique partial incidence mask `q_g`.

Ignore the old delta-table value after recording `q_g` and multiply the new
packet-tensor entries on that assignment. This gives

```text
C_(beta,g)(q_g).
```

The old vertices are partitioned by their active label digit, so the group
incidence sets are pairwise disjoint and cover `I`. Define

```text
H(q) = sum_beta product_g C_(beta,g)(q restricted to I_g).
```

This is `extracted_basis_map`; its packet values come from production child
construction and `PacketTensor::entry`. `[pcc:unverified]`

## Refinement lemma

Under the scalar Cauchy mode-series identity, exact packet coverage, positive
mode weights, and the weighted-tuple iterator invariant,

```text
D(q) = E(q) = H(q)   for every q in {0,1}^I.
```

The producer proof has four steps. `[pcc:unverified]`

1. `D=E`: split each row sum into same-group and crossing columns. The scalar
   mode-series identity replaces every crossing kernel weight exactly modulo
   `2^p`; substitution into each affine row factor and multiplication preserve
   equality.
2. Expand `E` distributively and collect the count of each mode into `beta`.
   Target-row coefficients and source linear forms separate exactly as in the
   packet-polynomial coefficient identity.
3. For fixed `beta`, finite Fubini factors the incidence sum by group. Whole
   old vertices, rather than individual ports, belong to a group, so all port
   correlations are retained. The child-edge/old-incidence correspondence is
   a bijection, making each group factor exactly `C_(beta,g)`.
4. A term indexed by `beta` is divisible by
   `2^(sum_mu beta_mu*w_mu)`. Terms of weight at least `p` vanish in `R`; the
   iterator visits every lower-weight, degree-admissible tuple exactly once.
   Thus the retained sum is `H`.

Equality on every incidence delta basis implies equality for arbitrary old
tensor tables by separate multilinearity in each table. `[pcc:unverified]`

## Rust correspondence and independent-check boundary

| Lemma object | Native implementation |
| --- | --- |
| `D` | `direct_packet_basis_map` |
| `E` | `mode_expanded_basis_map` |
| child-mask bijection | `child_basis_map` |
| `H` | `extracted_basis_map` |
| retained `beta` values | production `next_tuple` |
| packet coefficient | production `PacketTensor::entry` |

The bounded controls establish triangular equality through five edges, 91
five-edge graph-shape orbits, four label schedules, and 372,736 basis
coefficients. They do not quantify over all `M`, all labelings, or all
creation-precision histories. `[pcc:unverified]`

An independent proof kernel should accept a finite structural certificate and
check, without invoking production child construction:

1. every old edge occurs in one packet slot;
2. every incidence belongs to one group mask;
3. each generated mode has one target row group and one source column group;
4. the scalar crossing series equals the kernel weights modulo `2^p`;
5. the packet coefficient identity for each retained `beta`; and
6. the weighted iterator covers exactly the retained tuples.

Supplying that checker, or a theorem-assistant refinement of the same six
items, is the remaining independent boundary for this lemma.
`[pcc:unverified]`

## Native certificate checker

The test-only `factorization_certificate` module now implements all six checks
on a plain in-memory certificate. Production code supplies packet slots,
incidence groups, kernel-weight tables, generated modes, retained tuples, and
black-box packet-entry tables. The checker independently verifies coverage,
group placement, crossing series, Cartesian tuple completeness, and every
packet coefficient before accepting the certificate. It never calls
production `child` while checking. `[pcc:unverified]`

Four mutations are load-bearing: duplicate packet coverage, target-group
reassignment, removal of a retained tuple, and alteration of one packet-table
entry are each rejected. During implementation the checker detected and
localized an indexing error in the new certificate producer: packet truth
tables initially used global child-edge identifiers instead of local packet
port indices. The corrected producer and independent checker now agree.
`[pcc:unverified]`

The positive certificate is now serialized to JSON and parsed into fresh data
before verification. Its exact wire size is 2,162 bytes under the pinned Rust
dependencies. A schema-version mutation and a noncanonical ring coordinate are
rejected before the six algebraic checks, an overflowing structural count fails
closed, and reserialization is byte-for-byte stable. `[pcc:unverified]`

A second positive control serializes and verifies all 25 left/right set-partition
pairs of three labeled edges under four label schedules. The 100 certificates
occupy 317,734 bytes in total (3,177.34 bytes on average); the maximum is 10,136
bytes for partition pair `[0,1,1]/[0,1,1]` under schedule 3. The complete warm
Rust 1.78 debug sweep verifies in 0.34 seconds. `[pcc:unverified]`

This is not yet an external or theorem-assistant kernel. Producer and checker
still compile in the same test-only Rust module, there is no standalone checker
executable, and the positive controls remain bounded through three edges.
`[pcc:unverified]`
