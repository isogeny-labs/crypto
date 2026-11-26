# Provenance description envelope

This note counts the schema-3 packing object and complete schema-2 child
family used by the finite refinement controls. The native envelope functions
derive JSON and canonical-binary ceilings directly from serialized structural
atoms, arrays/objects, and named fields. `[pcc:unverified]`

## Packing provenance

Let `M` be the parent edge count, `V` the total number of source vertices on
both sides, `K` the packet count, and `P` the total number of stored packet
digit pairs. The packing object has the following ceilings:

```text
A_pack = 6 + 4M + 2V + 2P
C_pack = 7 + M + K + P
F_pack = 12 + 4M
```

Here `A` counts primitive atoms, `C` counts arrays/objects, and `F` counts
named fields. With `B` the maximum actual integer bit width, at least one, the
implemented bounds are

```text
JSON bytes <= 64 (A_pack + C_pack + F_pack + 1),
binary bits <= A_pack B + 64 (C_pack + F_pack + 1).
```

The count includes endpoint vertex/port indices, labels and creation
precisions for all vertices including isolates, the optional source degree
cap, and every packet digit pair. `[pcc:unverified]`

## Complete child family

For one child `c`, let `Q_c` be its serialized `beta` width, `O_c` its retained
old-vertex count, `K_c` its packet-vertex count, and `E_c` its edge count. Its
structural counts are

```text
A_c = 2 + Q_c + 4 O_c + 2 K_c + 4 E_c,
C_c = 8 + O_c + E_c,
F_c = 9 + 2 O_c + 4 E_c.
```

The family envelope sums these quantities over the serialized children and
uses the same JSON/binary formulas. It counts stable parent side/index tags,
all degree and creation-precision arrays, and every child endpoint/port.

For an accepted complete family, let `T` be the tuple count, `R` the radix,
`Q` the common tuple width, `V` the source vertex count, and `K` the packet
count. Exact family replay proves

```text
number of children                 = T R,
sum_(groups) O_c per tuple         = V,
sum_(groups) E_c per tuple         = 2M,
K_c                                = K.
```

Therefore the complete child-family counts simplify to

```text
A_child = T R (2 + Q + 2K) + 4 T V + 8 T M,
C_child = 8 T R + T V + 2 T M,
F_child = 9 T R + 2 T V + 8 T M.
```

Thus child provenance is `T` times a polynomial structural factor; it does
not introduce a second tuple exponential. Source isolates enter through `V`
and remain part of the original description length rather than being hidden
under the edge count. Connecting this exact structural formula to the
schema-5 absorption inequality is still a separate asymptotic step.
`[pcc:unverified]`

## Native control

The admitted three-edge control has four retained tuples and two groups:

```text
packing JSON bytes                    414
packing JSON bound                  4,672
packing canonical-binary bit bound 2,564
child certificates                      8
child-family JSON bytes              3,764
child-family JSON bound             37,440
child-family binary bit bound       21,824
```

Every factorization producer path checks its actual serialized packing and
complete child-family bytes against these structural envelopes before
returning. The four factorization controls cover the correlated fixture and
all 596 three- through five-edge certificates; the executed diagnostic uses
the same path. No practical Curve25519 attack follows. `[pcc:unverified]`
