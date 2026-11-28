# Child port mass: pre-edit instance gate

Status: proposed source-binding study, before compiler or native execution.
The existing root35/history81/conditional34 records stay unchanged.

## Six fields

1. Structure: Mathlib `Multiset (G × G)`, with finite group-label type G,
   `[Fintype G] [DecidableEq G]`. Multiplicities preserve parallel edges.
   Endpoint ports are the existing multiset operations
   `edges.map Prod.fst + edges.map Prod.snd`; no graph/type wrapper is introduced.
2. Existing theorem: `Multiset.sum_count_eq_card` from
   `Mathlib.Algebra.BigOperators.Group.Finset.Basic`. With the full label
   alphabet, it counts all endpoint ports exactly. `Multiset.card_add`,
   `Multiset.card_map` and `Multiset.count_add` give2*card(edges).
   No new counting argument or novel theorem is needed. A reusable named
   composition belongs in crypto-lean before the study endpoint.
3. Instance: source packing contains every original edge exactly once;
   the actual child constructor retains each selected left/right endpoint
   as one edge to its packet vertex. Native group labels are base-r digits.
   Each edge contributes one tag on each side, even for parallel ports.
   The Rust-to-multiset extraction is checked on bounded actual objects;
   the generic theorem alone is not a Rust refinement proof.
4. Assembly: first type-check the existing counting theorem on the endpoint
   multiset, card/map/add composition and common-weight multiplication.
   Include a nonempty literal Fin2 parallel-tag example. Missing component
   is only the shared named composition, not a replacement counting engine.
5. Components and order: assembly; shared `grouped_port_mass` and
   `grouped_port_weight`; study endpoint/axiom checks; frozen source-bound
   Rust constructor controls. No native counter algorithm change.
6. Inheritance: tag collection via existing `Multiset.map`/addition;
   sum-to-card via `Multiset.sum_count_eq_card`; endpoint size via
   `card_map`/`card_add`; pointwise counts via `count_add`; common weight
   via `Finset.sum_mul`. Graph validity and packet coverage come from the
   actual Rust routines and remain separately scoped.

## Proof chain and fidelity

Let ports=map fst(edges)+map snd(edges). For all labels g in the full finite
alphabet, sum count(g,ports)=card(ports), by the existing theorem as stated.
One replacement gives card(ports)=card(edges)+card(edges)=2card(edges).
For any common natural weight p, distributivity gives
sum_g[(leftcount_g+rightcount_g)*p]=2card(edges)*p.

The native working ring is common to all group children of one fixed
evaluation/mode tuple. This permits that structural weighted quantity;
it does not price tensor entries, coefficient extraction, mode enumeration,
interpolation, rational widths or total bit operations. A stopped traversal
visits a subset of nonnegative group masses, so it is at most the full sum.

The primary paper D.4.6 already constructs one child per label and retains
each group's ports. This study checks a useful accounting consequence of
that construction, not a novel ECDLP mechanism. Source strict shrinkage
8Mchild<=Mparent is a separate upper bound; conservation alone does not
imply shrinkage or a lower child-size bound.

## Exact qualification environment

Official Lean4.35.0; available retained Mathlib cache/source56a005855ea17e0b0958f73ec801ae140b0a1679.
crypto-lean's existing default Mathlib pin is cfc55fc58714a59dc06e6e79edc6d1d95140d3c2,
also Lean4.35.0. The counting theorem's complete source file is identical
between pins, but a conservative1352-module import scan finds changed
transitive sources. Therefore a focused56 build must be labeled56 and is
not a default-pin/full-library qualification. Do not change either existing
pin or claim the cached imports are cfc. Default integration remains open.

KB MCP unavailable, wrapper127. Bounded repository/GitHub searches followed by
exact Mathlib/native/primary-source reads supply no novelty or absence claim.
The generic laws are ordinary existing multiset counting instantiated here.

## Acceptance and resource limits

Accept the scoped component only after assembly, shared composition,
study endpoint and all reported axiom/gate checks pass on their recorded
pin, plus the source-bound native experiment in NATIVE-PLAN. Preserve every
failed attempt and pin mismatch explicitly. No holes/custom axioms,
native_decide or raised elaboration limits. One compiler worker,600s phase
ceiling, fixed whole-cgroup stop15,032,385,536bytes. Local only; no S3.
