# Group-child topology provenance certificate

The schema-1 child-provenance certificate independently reconstructs one
finite group-child graph skeleton from a checked schema-2 packing-provenance
object and its packet slots. The verifier imports no graph, deformation,
packet-tensor, or contraction implementation. `[pcc:unverified]`

## Reconstructed objects

Packing provenance now includes both endpoint vertex and port indices for every
parent edge. The child verifier first checks that each parent's used ports are
unique and contiguous. For the selected radix group it then reconstructs:

1. retained old vertices, ordered as left-side vertices followed by right-side
   vertices, including selected isolates;
2. every retained old vertex's complete degree and preserved port indices;
3. one packet vertex for every parent packet, including isolated packets;
4. row incidences followed by column incidences within each packet;
5. packet ownership and the exact sequential packet-port index; and
6. the complete child edge order.

The producer serializes only the actual child's old/packet degree vectors and
edge endpoints. After a deterministic JSON round trip, the standalone verifier
reconstructs the expected skeleton and requires exact equality.
`[pcc:unverified]`

## Native control

The admitted three-edge conditional-packing control constructs both radix
group children at the zero mode tuple. Each child's edge count agrees with the
standalone replay. Incrementing one packet-port index fails with
`child provenance replay`. Existing port-mass controls separately cover 25,490
group-child calls across parallel ports, isolates, several radices, and packet
geometries using direct source-object comparisons. `[pcc:unverified]`

## Remaining boundary

This closes finite child topology: vertex retention, isolates, endpoint/port
identity, packet replication, and edge ordering. It does not independently
bind tensor identity, tensor entry semantics, creation precision, or mode
coefficients, nor does it prove a uniform arbitrary-size refinement or the
external contraction theorem. No practical Curve25519 attack follows.
`[pcc:unverified]`
