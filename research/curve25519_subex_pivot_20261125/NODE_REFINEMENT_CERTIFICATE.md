# Composed node-refinement certificate

The schema-1 node-refinement certificate is a standalone composition boundary
for one finite recursive node. It bundles existing schema-versioned objects and
imports only their verifiers, not graph, packing, deformation, packet-tensor,
or contraction implementation code. `[pcc:unverified]`

## Verification order

The verifier requires normalized precision and checks, in order:

1. the complete schema-2 factorization certificate, including mode tuples and
   every packet coefficient table;
2. source dimensions `(M,K,L,tau,D,p)` with `KL<=3M` and
   `pu<=34MD`;
3. schema-3 conditional packing provenance, including actual source degree,
   variance, capacity, packet replay, and every-coordinate group balance;
4. the complete schema-2 Cartesian child family, including topology, tensor
   references, creation precision, table keys, and `2M` endpoint coverage; and
5. exact equality of edge, packet, packet-length, exponent, coordinate, tuple,
   and group dimensions across those interfaces.

The report returns actual maximum child edges and degree, recomputed class
count/maximum/padded height, the explicit `LD<=M` result, whether
`M>=2D^2delta` and `M>=LD` imply uniform packing closure, and whether exact
child replay satisfies `child*radix<=2*height*D`. For radix 64 it composes those
facts into a separate packing-derived eightfold-shrink result and rejects any
disagreement with observed child shrink. `verify_strict` requires that derived
result, source closure, and child degree refinement. This separates a valid
finite algebraic/refinement object from a strict theorem node rather than
promoting one to the other.
`[pcc:unverified]`

The successful summary also has a canonical eight-natural projection:
schema, radix, parent edges, packet length, label classes, maximum class,
padded height, and maximum child edges. A native checker independently
recomputes the rounded height and the two exact inequalities. The matching
Lean `NodeShrinkArtifact` rejects trailing fields, proves checked projection
round trips, and derives eightfold shrink only after the compact fields are
aligned with a separately strict-parsed `PackingLabelSource`. The summary is
therefore arithmetic evidence, not a replacement for the full source trace.
`[pcc:unverified]`

The native exporter also emits a joint projection: those eight values followed
by the complete packing-label projection. The formal bundle importer splits at
the fixed eight-field boundary, checks compact arithmetic, runs the full strict
packing parser, and internally checks parent edge count, label-class count, and
maximum class equality. Its soundness theorem gives bundle validity for every
successful parse, and every valid or successfully parsed bundle yields
eightfold shrink with no caller-supplied alignment equality.
`[pcc:unverified]`

## Native control

The admitted three-edge conditional-packing fixture serializes and parses a
complete node bundle. The composed verifier accepts it, including normalized
precision, while reporting source-guard closure and strict source refinement
false because the small radix-2 object is below the asymptotic guards and its
children do not meet the fixed radix-64 interface. It still reports exact
row/column incidence accounting and the rounded-height child bound true. The
compact projection preserves the exact parent/child counts and its arithmetic
checker rejects the radix-2 context. The strict entry point rejects with
`node refinement strict source guards`. Clearing the required
normalized-precision flag rejects at the context gate.
`[pcc:unverified]`

## Remaining construction boundary

The checker is generic over finite certificate size: any supplied radix-64
node satisfying every interface and both source guards is accepted by the same
code. `PACKING_GUARD_CLOSURE.md` gives the uniform producer induction, while
the strict root and schema-6 transcript supply compatible schedules.
`COMPOSITION_BOUNDARY_CERTIFICATE.md` now checks that compatibility and
falsifies the stronger identification: the schema-6 normalized local witness
has different `M,K` from the strict root. A natural-number formal endpoint now
consumes the exact `LD<=M` and `child*64<=2*height*D` facts reported here, and
the compact formal importer pins their field order and arithmetic acceptance.
The joint formal importer also removes caller-supplied summary/source alignment.
Proving that every native joint projection equals the corresponding formal
bundle across compiler, allocation, conversion, and serialization semantics is
still needed; the live `usize` branch cannot allocate a strict-root bundle, and
the external contraction theorem remains unproved. No practical Curve25519
attack follows.
`[pcc:unverified]`
