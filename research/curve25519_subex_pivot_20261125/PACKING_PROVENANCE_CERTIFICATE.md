# Packing provenance certificate

The schema-3 packing-provenance certificate binds an accepted schema-2
factorization certificate to a finite labeled bipartite edge skeleton. Its
standalone verifier imports neither graph nor packing implementation code.
`[pcc:unverified]`

## Serialized boundary

The provenance object carries:

- the packet radix, packet exponent `a`, and active coordinate `i`;
- each edge's left and right endpoint vertex and port index in source order;
- labels for every left and right vertex, including isolates;
- creation precision for every source vertex tensor; and
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

When the provenance object claims source-selected labels and supplies the
degree cap `delta`, the verifier also independently replays the complete greedy
conditional-expectation selection. It starts with unknown labels, visits left
then right vertices in source order, evaluates every label candidate using the
exact edge-pair objective, preserves first-minimum tie breaking, and assigns
zero to isolates. The reconstructed label arrays must match exactly. It then
reconstructs every left/right source degree and requires it to be at most
`delta`, then checks the variance inequality

```text
array * sum_class count(class)^2 - M^2 <= 2 M delta array,
```

requires `KL<=3M`, and checks the radix group-port balance at every coordinate.
`[pcc:unverified]`

`PACKING_GUARD_CLOSURE.md` derives a stronger implication. When the same finite
object also satisfies the active source guards `M>=2D^2delta` and `M>=LD`, the
variance bound forces `maximum_class*D<=2M`. The verifier reports this strict
guard closure and additionally checks the resulting per-side group load
`<=2M/R`. The class cap gives padded mass and packet count at most `3M`.
Allocated small controls correctly report that the asymptotic guards are not
available. `[pcc:unverified]`

## Bounded evidence

Every producer path in the bounded factorization controls now serializes,
parses, independently replays, and verifies this provenance before returning a
certificate. The corpus covers all 596 three- through five-edge partition
certificates, the correlated two-edge fixture, and the executed non-strict
recursive diagnostic. A packet-pair mutation fails with
`packing provenance replay`. A dedicated admitted three-edge control uses the
actual conditional-expectation packing path; both the label-selection and all
source-bound replays pass. A label mutation fails with
`packing provenance conditional labels`, and lowering the claimed cap below an
actual degree fails with `packing provenance source degree cap`.
`[pcc:unverified]`

The verifier exposes one overflow-checked `label_projection` function with a
canonical header, four fields per edge, then all row and column labels. Every
producer path constructs the projection before accepting the schema-3 JSON
round trip; this includes all 596 three- through five-edge corpus cases. The
admitted control exports `packing_label_witness.csv` from the same function.
The transparent Lean parser in `lean/PackingLabelArtifact.lean`
checks exact lengths and bounds, proves typed endpoint values are the serialized
indices, proves typed row and column labels equal the serialized naturals
without modulo wrapping, requires nonzero radix and an active coordinate below
the packet exponent, and replays the same strict-first-minimum algorithm. The
kernel-checked trace is rows `[0,1,0]` followed by columns `[0,0,1]`; both sides
therefore exercise a nonzero choice. A generic checked-import theorem rewrites
accepted serialized labels to the executable schedule and returns the strongest
source-guard class cap from caller-supplied degree caps and guard alone. A
second generic importer recomputes both endpoint degree bounds and the exact
`2(q^2)^2 delta<=M` source-class guard from the flat values, and returns the
class cap with no remaining premise. The admitted three-edge control is
kernel-proved to fail this stronger guard and parser, matching the native
`source_guard_closure_admitted=false` result. Replay now checks each local
first-minimum against the serialized prefix and derives equality with the
recursive executable schedule by well-founded induction. The separate shared
`packing_label_source_guard_witness.csv` is a degree-one matching with two
labels and 32 edges, exactly `M=2(q^2)^2 delta`; the native verifier reports
source-guard closure and Lean's strict parser accepts the same projection.
The independent native verifier now replays the same proved auxiliary-state
schedule instead of reevaluating every edge pair: row labels use degree and
right-endpoint incidence buckets, then column labels use row-label incidence
profiles. The production packer and independent importer both agree with the
test-only quadratic oracle on all 3,280 ordered `3 x 3` multigraph controls
through three edges and label alphabets 1 through 4, including isolates,
parallel edges, and shared endpoints. Exact-boundary degree-one matching
controls also pass at `a=2,3,4`, respectively
`(M,D)=(512,16),(8192,64),(131072,256)`, with maximum classes
`32,128,512`, packet counts `128,1024,8192`, and charged-step counts
`15810,450038,13496226`. The three new rungs complete together in 0.71 seconds
after compilation. This is finite scaling evidence for the proved score
interface, not a uniform code-to-Lean refinement theorem.
`[pcc:unverified]`
The formal artifact type now also has a canonical `projectionValues` encoder
in exactly the header/edge/row/column order consumed by its parser. Lean proves
the unchecked parser is a left inverse whenever the two declared split lengths
hold, and proves every well-formed bounded artifact round-trips through the
checked parser. The replay and source-guard parsers inherit that round trip
when their formal trace and guard predicates hold. This is arbitrary-finite
formal encoder/parser closure. Conversely, projecting any successful unchecked
parse reconstructs the complete original flat value list, and every checked
parse inherits that identity; accepted trailing data or alternate flat shapes
are therefore impossible. These theorems do not identify the encoder with
Rust's compiler, overflow, or serde semantics. `[pcc:unverified]`
The native verifier also now requires `conditional_labels` to equal cap
presence in both directions; a missing-cap mutation fails at the context gate.
The formal replay no longer reevaluates candidate-independent terms: an exact
affine decomposition proves that the quadratic objective is an offset plus
`2q` times a directed interaction score for rows and columns. Computing that
proved score reduced the best observed cold 32-edge artifact build from 229 to
145 seconds. A second formal split now proves the row score equal to degree
times selected-label bucket degree plus `(q-1)` times the right-endpoint
overlap, and the column score equal to `q` times the row-label profile dot
product. Computing those source-shaped formulas reduces the best observed cold
build to 55 seconds.
`BucketStateReplay.lean` now proves the required auxiliary invariant for both
phases: empty state, exact row/column insertion, and equality of every
state-derived candidate score with its extensional incidence formula. A
fuel-bounded single-pass fold now maintains that invariant and is proved equal
to the original recursive schedule; artifact replay uses it, reducing the best
observed cold build to 52 seconds. The exact-boundary CSV is now embedded as
bounded elaboration-time chunks and reassembled into the same public string
view. Kernel reduction over the chunked character view cuts the parse receipt
from 23 seconds to 8.6 seconds. Parse, row replay, column replay, and
source-guard checks remain separate parallel modules, while a single final join
replaces the former serial trace/parser pair. A forced complete rebuild took
27.82 seconds wall clock, down from 40.94 seconds (32.0%), meeting the
sub-30-second end-to-end target without changing the formal claim boundary.
`[pcc:unverified]`

## Remaining boundary

This closes finite replay from an edge skeleton through conditional label
selection, source packing bounds, packet slots, and active factorization groups.
The native flat-export path is now size-generic; semantic Rust-to-Lean binding
still has only the admitted three-edge control and exact-boundary 32-edge
control.
The endpoint ports and source creation precisions also feed the separate
schema-2 child provenance replay in `CHILD_PROVENANCE_CERTIFICATE.md`.
`PROVENANCE_DESCRIPTION_ENVELOPE.md` gives the checked structural description
ceiling for this object and the complete child family.
Together with `PACKING_GUARD_CLOSURE.md`, it gives a uniform producer-side
packing theorem conditional on the source guards; child symmetry is not
needed. It does not independently formalize that implication, discharge the
factorization/refinement theorem, lift the finite shared traces to every
arbitrary-size producer object, or prove the external contraction theorem. No
practical Curve25519 attack follows.
`[pcc:unverified]`
