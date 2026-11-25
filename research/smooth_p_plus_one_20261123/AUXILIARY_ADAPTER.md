# Auxiliary-curve factor-base adapter: software qualification

2026-11-25 · `auxiliary_adapter_v1` · PR #5142

The next component connects the published auxiliary-isogeny library to the
unchanged native `prepare_cofactor` boundary. It also exposes matched leaf
constraints in chained and dense presentations. This is software integration:
no F4/msolve invocation, relation collection, recovery or performance panel is
part of this qualification. The existing scientific execution requirements
remain in force. No new mathematical mechanism is proposed.

Use a separate Cargo workspace `auxiliary-rr5`, depending on the existing
auxiliary library and the unchanged native RR5 libraries. Preserve all prior
source/evidence pins. Constructor inputs are a public target-curve descriptor,
an already constructed auxiliary chain and its anchor. Generator search and
chain construction therefore remain explicit caller costs. The adapter must:

1. Require matching fields, canonical short-Weierstrass target coefficients
   and a regular auxiliary fiber. Use the existing target lifting operation;
   sort the retained points by x and use the smaller nonzero y as positive.
2. Assign labels +1 through +B in that order, with negative labels denoting
   negation. Reject zero, out-of-range and noncanonical inputs. Map up to four
   labels to the existing `SignedRelation` indices without reordering the base.
3. Retain the full auxiliary fiber polynomial N-chi D and form the separate
   monic polynomial H of the actual lifted base abscissae. Their degrees need
   not agree. Expose both, never replace B by the isogeny degree in accounting.
4. Offer a dense leaf system H(x)=0, the target equation
   y²-x³-a*x-b=0 and y*s-1=0. Its variables are x,y,s.
5. Offer a chained leaf system with the same curve and nonzero-y equations,
   then one cleared equation per map. For z=x_i-r_i and the existing map
   x_(i+1)=z+a_i+b_i/z, use z*(x_(i+1)-z-a_i)-b_i=0; fix the last image to chi.
   The map construction has b_i nonzero, so its equation has no solution at
   z=0. This is the standard rational-map identity, explained in prose rather
   than a new checked identity certificate. Intermediate variables are
   x1 through x_(d-1); every input/assignment must be canonical.
6. Pass the exact sorted point list and public target into the unchanged
   `prepare_cofactor`, preserving its errors and resource limits. This is
   equation construction only, not solver extraction. The RR5 preparation
   boundary uses target-curve geometry; it does not certify the descriptor's
   subgroup order or generator. A later recovery consumer must validate those
   separately under its existing contract.

Unit fixtures are F31 with auxiliary E': y²=x³+x, G=(6,6), depth three and
anchor G. The primary target is the same curve, with order 32 and generator
G used only as public geometry metadata; no prime-order recovery is claimed.
The filtering target is y²=x³+x+5, as in the earlier library controls.
There is no F127/F8191 selection panel in this component.

Qualification checks: deterministic labels under anchor negation/translation;
all canonical F31 affine pairs against the low/dense leaf predicates and
actual signed-base membership; translated-kernel poles, corrupted intermediate
coordinates and coefficient mutation; target nonresidue filtering; malformed
labels/fields/coefficients; and exact equality between adapter-prepared and
directly prepared native cofactor equations on the known public target (7,3).
The identity and target-in-base statuses must propagate unchanged. No test
calls a relation solver or searches for a target scalar. Tests share arithmetic
and remain producer-local software evidence.

Retain every invocation, output and exit status. Run the component's unit
suite and release library build; do not repeat a passing suite without a code
change or unresolved issue. Emit no scientific stream record. A later matched
comparison must charge generator search, maps, fiber enumeration, lifting,
label/index and polynomial construction, serialization, all relation attempts,
validation, deduplication, rank/linear algebra and scalar replay.

Sources: [PKC 2016, Sections 3.2 and 3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
the [existing auxiliary library protocol](AUXILIARY_ISOGENY.md), and the actual
`prime31_four_summand_20261122/native/{extractor,relation_adapter}.rs` APIs.
KB status and exact adapter/denominator queries are retained in this component's
evidence receipt; their unavailable status does not establish novelty.
