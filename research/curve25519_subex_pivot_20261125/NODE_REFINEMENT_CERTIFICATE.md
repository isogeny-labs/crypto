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

The report returns actual maximum child edges and degree and says whether the
fixed radix-64 strict source guards hold. `verify_strict` rejects unless that
flag is true. This separates a valid finite algebraic/refinement object from a
strict theorem node rather than promoting one to the other.
`[pcc:unverified]`

## Native control

The admitted three-edge conditional-packing fixture serializes and parses a
complete node bundle. The composed verifier accepts it, including normalized
precision, while reporting strict source refinement false because its radix-2
children do not meet the eightfold-shrink and `2L<=delta` guards. The strict
entry point rejects with `node refinement strict source guards`. Clearing the
required normalized-precision flag rejects at the context gate.
`[pcc:unverified]`

## Remaining construction boundary

The checker is generic over finite certificate size: any supplied radix-64
node satisfying every interface is accepted by the same code. It does not
construct a uniform arbitrary-size sequence of such nodes, prove that the live
unreachable `usize` branch emits the bundle, or prove the external contraction
theorem. The schema-6 symbolic transcript supplies compatible scalar schedules
and description accounting, but realizing those schedules remains the next
construction problem. No practical Curve25519 attack follows.
`[pcc:unverified]`
