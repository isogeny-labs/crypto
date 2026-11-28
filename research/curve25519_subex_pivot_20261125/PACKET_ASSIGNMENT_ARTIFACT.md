# Packet-owner assignment artifact

The schema-1 packet-assignment projection derives one packet owner for every
parent edge from the accepted schema-2 factorization certificate. Its header
is projection schema, packet count, parent-edge count, and owner count,
followed by the owner array in parent-edge order. `[pcc:unverified]`

The native exporter requires at least one packet and rejects an edge outside
the parent range, duplicate ownership, omitted edges, and checked output-length
overflow. The focused conditional-packing control reconstructs the same owner
array from live packet slots, checks the complete projection, and rejects a
duplicate-owner mutation. `[pcc:unverified]`

The joint incidence-assignment projection adds an explicit group-projection
length, then concatenates the complete endpoint-group and packet-owner
projections. `lean/PacketAssignmentArtifact.lean` parses both components,
checks their individual shapes and ranges, and requires a common parent-edge
count. Kernel proofs establish canonical round trips, successful-parse
validity, exact typed packet lookup, and equality of the row- and column-side
packet owner of each edge. `[pcc:unverified]`

The imported endpoint-group and packet-owner functions instantiate
`GroupFactorization.complete_child_family_factorization` for arbitrary old
terms and packet entry functions. This closes the finite assignment-index
split, but it does not import or authenticate the packet entry values,
scalar/mode coefficients, child tensor values, native-to-formal execution
semantics, or the external contraction theorem. It is not an unconditional
subexponential result. `[pcc:unverified]`
