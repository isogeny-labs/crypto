# Group-child provenance certificate

The schema-2 child-provenance certificate independently reconstructs one
finite group child from checked schema-3 packing provenance and schema-2
factorization objects. The verifier imports no graph, deformation,
packet-tensor, or contraction implementation. `[pcc:unverified]`

## Reconstructed objects

Packing provenance includes endpoint vertex/port indices and parent tensor
creation precision. The child verifier first checks that each parent's used
ports are unique and contiguous. For the selected radix group it then
reconstructs:

1. retained old vertices, ordered as left-side vertices followed by right-side
   vertices, including selected isolates;
2. every retained old vertex's parent side/index, complete degree, preserved
   port indices, and creation precision;
3. one packet vertex for every parent packet, including isolated packets;
4. row incidences followed by column incidences within each packet;
5. packet ownership and the exact sequential packet-port index; and
6. the complete child edge order, packet creation precision, and the exact
   factorization entry-table key `(beta, group, packet)`.

Before serialization the producer requires every retained old tensor to be the
same `Arc` as its parent source and every new right tensor to have packet kind.
After a deterministic JSON round trip, the standalone verifier reconstructs
the expected source references and topology, requires old precision to equal
the parent manifest and packet precision to equal the factorization precision,
and binds every packet vertex to the complete entry table already checked by
the factorization verifier. `[pcc:unverified]`

## Native control

The admitted three-edge conditional-packing control constructs both radix
group children at the zero mode tuple. Each child's topology, source references,
creation precisions, and packet-table keys agree with standalone replay.
Mutating a packet port, parent precision, packet precision, or the mode tuple
fails closed. In addition, every factorization producer path serializes and
replays every `(beta, group)` child after completing its entry tables. This
covers the correlated fixture, all 596 three- through five-edge certificates,
and the executed diagnostic. A standalone family verifier requires exact
Cartesian tuple/group keys, rejects duplicates and omissions, checks exactly
`2M` child incidences per tuple, and binds exactly one packet table per packet
in every child. Existing port-mass controls separately cover 25,490 group-child
calls across parallel ports, isolates, several radices, and packet geometries
using direct source-object comparisons. `[pcc:unverified]`

## Remaining boundary

This closes the finite checked interface from parent tensor references and
topology through creation precision to the selected packet coefficient tables.
The packet values and mode coefficients are established by the separately
accepted factorization certificate; arbitrary old tensors remain referenced,
not re-serialized. It does not prove a uniform arbitrary-size object
refinement or the external contraction theorem. No practical Curve25519 attack
follows. `[pcc:unverified]`
