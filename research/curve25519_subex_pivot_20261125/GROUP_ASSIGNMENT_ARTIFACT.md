# Endpoint-group assignment artifact

The schema-1 group-assignment projection binds the exact row and column group
arrays already stored in an accepted schema-2 factorization certificate. Its
header is projection schema, radix, parent edge count, left-array count, and
right-array count, followed by both arrays in parent-edge order.
`[pcc:unverified]`

The native exporter rejects a wrong certificate schema, zero radix, array
lengths unequal to the parent edge count, any group outside the radix, and
checked output-length overflow. The finite conditional-packing control checks
that the emitted slices equal the live `left_groups` and `right_groups` arrays;
an out-of-range mutation is rejected. `[pcc:unverified]`

`lean/GroupAssignmentArtifact.lean` mirrors that field order. Its checked
parser requires exact declared lengths, one group per endpoint side of every
parent edge, positive radix, and every group in range. Kernel proofs establish
unchecked and checked canonical round trips, successful-parse well-formedness,
and exact typed row/column lookup values. `[pcc:unverified]`

The imported arrays instantiate `GroupFactorization.endpointGroup` on
`Fin edgeCount x Bool`. The generic fiber theorem then gives the exact endpoint
mass

```text
sum_group |{ endpoint incidence assigned to group }| = 2 * edgeCount.
```

This binds the native splitting arrays to the formal group-fiber identity. It
does not yet bind packet ownership, scalar/mode coefficients, child tensor
values, or the external contraction theorem, and it is not an unconditional
subexponential result. `[pcc:unverified]`
