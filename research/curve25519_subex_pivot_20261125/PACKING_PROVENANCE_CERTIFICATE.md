# Packing provenance certificate

The schema-1 packing-provenance certificate binds an accepted schema-2
factorization certificate to a finite labeled bipartite edge skeleton. Its
standalone verifier imports neither graph nor packing implementation code.
`[pcc:unverified]`

## Serialized boundary

The provenance object carries:

- the packet radix, packet exponent `a`, and active coordinate `i`;
- each edge's left and right endpoint vertex index in source order;
- labels for every left and right vertex, including isolates; and
- the unordered digit-pair pattern attached to every emitted packet.

Given those fields, the verifier reconstructs `L=2^a`, the `R^a` label
alphabet, all `R^(2a)` ordered label classes, class membership in edge order,
the padded class height, every unordered orientation pattern, every copy index,
and every packet slot. Empty packets are suppressed exactly as in the source
schedule. The reconstructed packet array and pair patterns must equal the
serialized factorization/provenance objects byte-for-byte after a deterministic
JSON round trip. `[pcc:unverified]`

For every edge, the verifier also derives the active left and right group digit
from its endpoint label and requires exact equality with the factorization
certificate's group arrays. It reports whether the replayed packet count meets
`K L<=3M`; algebraic controls may replay correctly while remaining explicitly
non-admitted at source capacity.

## Bounded evidence

Every producer path in the bounded factorization controls now serializes,
parses, independently replays, and verifies this provenance before returning a
certificate. The corpus covers all 596 three- through five-edge partition
certificates, the correlated two-edge fixture, and the executed non-strict
recursive diagnostic. A packet-pair mutation fails with
`packing provenance replay`. `[pcc:unverified]`

## Remaining boundary

This closes finite provenance from a labeled edge skeleton to packet slots and
active factorization groups. It does not independently reproduce the source's
conditional-expectation label selection, prove the variance and group-balance
bounds, construct an asymptotic graph family, or prove the external contraction
theorem. Those are the next realization obligations; no practical Curve25519
attack follows. `[pcc:unverified]`
