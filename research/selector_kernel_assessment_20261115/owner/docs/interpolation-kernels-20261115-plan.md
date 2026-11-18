# Interpolation-kernel separation: prospective proof map

15 November 2026. Before the first Lean edit/build of this component.
Consumer: crypto PR4338, candidate A, at source e7668a1ebcf75d059ae16311a5d92f47a725fdc9.
This checks a literal interpolation-state enumeration model. It does not
claim a lower bound for all selectors, symbolic unions or rational interpolation.
No numerical, benchmark, native qualification or paid execution is planned.

## Six-field instance gate

1. Structure above: Mathlib Field F, Polynomial F, Finset I, polynomial
   evaluation/natDegree, Set predicates and finite function spaces. A state
   is the actual set of degree-bounded polynomial pairs satisfying selected
   point evaluations. No replacement elliptic curve or formal state-count
   surrogate is introduced.
2. Existing laws used as stated: Lagrange.nodal, natDegree_nodal,
   nodal_ne_zero, eval_nodal_at_node, eval_nodal_not_at_node;
   Finset.card_image_iff/card_image_of_injective and finite function-cardinality
   laws. New generic endpoint: different clean selected supports of size at
   most the u-degree cap have different bounded evaluation kernels.
3. Objects as instances: x,y are actual coordinate functions on primitive
   records, with Function.Injective x (the clean-abscissa assumption).
   For the curve assembly use Mathlib WeierstrassCurve.Affine F and
   q : I -> F x F, with W.Equation (q i).1 (q i).2 explicitly supplied.
   The law is stronger than a curve specialization; it needs no curve equation.
4. Assembly probe: first type-check inherited nodal degree/nonzero/evaluation
   laws on a generic field and finite support. Repeat evaluation assembly at
   coordinates of an actual Mathlib Weierstrass curve. The final probe will
   apply the shared endpoint unchanged to that coordinate data. Field and
   point assumptions remain explicit; no toy arithmetic or concrete fixture.
5. Missing components: bounded-kernel predicate and its membership interface;
   nodal-witness membership iff support inclusion; kernel-support injection;
   colored prefix support injection/cardinality and exact image-cardinality
   endpoint. Optionally a separate generic color-disjunction nonlinearity
   law, only after the finite endpoint is complete.
6. Inheritance table below. No new numerical checker or proof-limit override.

| Operation | Inherited structure/law | New component |
|---|---|---|
| Polynomial ring, eval, degree | Mathlib Polynomial F | none |
| Vanishing polynomial | Lagrange.nodal | none |
| Degree = support size | Lagrange.natDegree_nodal | none |
| Polynomial is nonzero | Lagrange.nodal_ne_zero | none |
| Evaluates zero at selected node | Lagrange.eval_nodal_at_node | none |
| Nonzero at other clean node | Lagrange.eval_nodal_not_at_node + injective x | zero iff membership |
| Bounded pair constraints | Set over Polynomial F x Polynomial F | predicate/interface |
| Kernel separates supports | nodal witness with v=0 | generic shared law |
| One record per color | Finset.univ.image of i -> (i,choice i) | support map laws |
| Number of choices/states | finite function and image cardinalities | generic endpoint |
| Curve coordinates | existing W.Equation predicate on F x F | assembly only |

## Exact proposed endpoint and scope

For t colors, b records per color and an injective abscissa map
x : Fin t x Fin b -> F, let D_c select (i,c i) for c : Fin t -> Fin b.
Let K_D be the set of pairs (u,v) with natDegree u <= dU,
natDegree v <= dV, and u(x_j)+y_j*v(x_j)=0 for all j in D.
If t <= dU, the finite image {K_(D_c) : c} has exactly b^t elements.
Every K_(D_c) has a nonzero polynomial-pair witness.

The consumer will take dU=floor(m/2), dV=floor((m-3)/2),
t=floor(m/2). For a model that explicitly materializes each exact kernel,
prunes only zero feasible kernels and memoizes by exact kernel equality,
the prefix states cannot collapse. Converting b^t to an asymptotic memory
consequence is consumer hand accounting, not a general algorithm lower bound.
A representation that stores a union without enumerating its members is not
covered. Pruning using future-color constraints is not covered by this model.

## Proof chain before code

For selected support D, use the existing polynomial h_D = Lagrange.nodal D x.

1. natDegree h_D = card D (existing law).
2. h_D != 0 (existing law).
3. eval(x_j,h_D)=0 iff j belongs to D, under injective x
   (at-node and outside-node laws).
4. Therefore (h_D,0) belongs to K_E iff E is a subset of D,
   provided card D <= dU; the v-degree bound holds for every dV.
5. Equal K_D,K_E imply E subset D and D subset E, hence D=E.
6. The colored-support map c -> D_c is injective and card D_c=t.
7. The kernel-state map is injective by steps5--6.
8. Apply the existing finite-image and function-cardinality laws to obtain b^t.

Only the kernel membership-interface lemma may open that composite predicate;
the other proofs should use its interface. Nodal internals are never opened.
No custom axiom, sorry/admit/native_decide, maxHeartbeats or recursion override.

## Acceptance, controls and receipts

Compile the pre-edit floor probe, retain its exact source/log and source pin,
then implement the generic shared module. Preserve every failed build before
repairing it. Build the actual source target. Run the unchanged repository-wide
all-source/axiom gate once after final edits; print axioms for every new named
law. If lean4checker is unavailable, report that fact instead of a fresh pass.
Keep the unrelated untracked ProjectiveVeluInstanceProbe.lean out of all commits.

The proof includes empty prefixes/empty choice domains and arbitrary fields;
the growing consumer uses nonempty lists and a clean odd prime-order subgroup
over F_p, p>3. The generic theorem relies only on injective abscissas, so that
curve/subgroup specialization is an endpoint application, not a new proof.

Sources checked: consumer PLAN.md and its pinned earlier assessments; Mathlib
LinearAlgebra/Lagrange.lean Nodal section and finite-image/cardinality laws.
Primary Jeannerod--Neiger--Schost--Villard, Computing minimal interpolation
bases, sections1.1--1.2/Definition1.1:
https://cs.uwaterloo.ca/~eschost/publications/jsc_final.pdf . Its problem is
simultaneous linear interpolation constraints; it supplies no disjunctive
colored-witness search. KB remains unavailable in the consumer runtime.
No novelty claim. A recently found random-AG-code threshold preprint is
combinatorial background; no efficient selector is inferred from its abstract.
