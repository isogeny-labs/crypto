# Proof map and inheritance review

## Timing and verification scope

The source/theorem searches and prospective scope commits preceded the Lean
implementation. The required lean-instance-gate six-field writeup and assembly
probe were completed after the builds, during final review. This timing does
not satisfy the skill's pre-edit ordering and is recorded as a process gap,
not a preregistration. No source or failed diagnostic is replaced to hide it.
The mathematical source/axiom gates are separate completed checks.

Lean4.35.0; Mathlib cfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Shared
coverage source2e4b08b, secant source867527c. Exact full pins/hashes are under
checks/coverage-laws/ and checks/line-laws/. Canonical gates134/141theorems,
0failures and0trusted native exceptions, respectively. Full audit inventories
name every theorem and its transitive standard axioms. These are local proof
checks; all PCC statements remain unverified without signed verdicts.
[pcc:unverified]

## Six fields: finite-group coverage

1. Structure above: Mathlib AddCommGroup G, AddMonoidHom, finite functions,
   additive subgroups and finite Nat-valued sums. The operations used are
   addition, zero, finite sum/product, kernel and cardinality.
2. Existing laws used as stated: AddSubgroup.card_ker_mul_card_of_surjective;
   Nat.card_fun/card_prod/card_fin; finite Cauchy--Schwarz
   Finset.sum_mul_sq_le_sq_mul_sq. No full partite coverage declaration was
   found in the inspected exact sources; the missing generic assembly lives
   in crypto-lean, not a curve-specific consumer copy. KB unavailable.
3. Instance: tupleSum is a homomorphism from the actual record-assignment
   function group to G; the joint map is its existing prod construction.
   The support is the subtype of assignments admitting an actual zero-sum
   tuple. No new group syntax, toy curve or assumed moment model.
4. Assembly probe: checks/inheritance/inheritance001.lean.txt checks the existing generic
   kernel law, finite Cauchy--Schwarz, and the actual coverage endpoint by
   single applications. Output is checks/inheritance/inheritance001.log.
5. Missing components: generic tuple-sum/surjectivity witnesses, moment
   bookkeeping and the coverage endpoint. All belong in shared
   CryptoLean.Group.PartiteCoverage. Curve cleanliness, label conditioning and
   asymptotic/program costs are separate, unformalized consumer bridges.
6. Inheritance table follows below. No proof-limit override/native trust.

| Statement/operation | Kind and owner | Inherited line / exact hypotheses |
|---|---|---|
| Tuple sum | Generic instance obligation, shared PartiteCoverage | AddMonoidHom over the pointwise AddCommGroup; map_zero/map_add from finite-sum laws |
| Single and pair maps | Generic instance obligations, shared | Pi.single coordinates; surjectivity for m>0 or distinct tuples |
| Kernel counts | Endpoints, shared | Existing AddSubgroup.card_ker_mul_card_of_surjective applied as stated |
| Single/pair event counts | Generic assembly, shared | Kernel subtype cardinality and existing Fintype.card_subtype/card_filter |
| Both moments | Generic laws, shared | Existing finite-sum commutation/distribution, diagonal and off-diagonal counts; Fintype G, m>0 |
| Support inequality | Endpoint, shared | Finset.sum_mul_sq_le_sq_mul_sq on support indicator and relationCount |
| coverage_count | Generic endpoint, shared | Both exact moments, support inequality and positive Nat cancellation; Fintype G, m,b>0 |
| Uniform probability interpretation | Hand consumer bridge | Finite assignment cardinality N=r^(mb); division by N>0 |
| Elliptic clean-base loss / labels / full cost | Hand consumer bridges | REPORT.md's subgroup/randomness/chart/selector hypotheses; no full Lean endpoint claimed |

Proof chain: form existing homomorphisms; replace their kernel counts by the
surjective-map cardinality law; restructure event sums; replace diagonal/pair
counts; apply finite Cauchy--Schwarz; replace exact moments; cancel positive
L*N. Each replacement uses its named law. This assembles the missing generic
statement and does not re-prove the kernel-cardinality or Cauchy--Schwarz law.
[pcc:unverified]

Composite opening: tupleSum and hit/relationCount are introduced in this shared
module. Their owning generic sum/spike and event-count lemmas open them to
connect to existing finite sums/subtype counts. No study-local curve instance
unfolds them or copies a generic bound. The full canonical audit covers their
definition equations too.

## Six fields: fixed-slope curve fibres

1. Structure above: existing WeierstrassCurve.Affine F, its Equation and
   addPolynomial, Cubic.toPoly, polynomial roots, finite sets and subtypes.
2. Existing laws used as stated: Affine.addPolynomial_eq;
   Cubic.natDegree_of_a_ne_zero'; Polynomial.card_le_degree_of_subset_roots;
   Fintype.card_le_of_injective; Finset.card_biUnion_le.
3. Instance: a nonvertical line substituted into the actual Weierstrass
   equation. The known point is a root; the other x-values are an erased
   root set. Projection from line points to x is injective.
4. Assembly probe: checks/inheritance/inheritance001.lean.txt applies generic polynomial
   root cardinality and the actual finite slope-family endpoint. It prints
   central coverage and all seven secant declarations' axioms.
5. Missing components: the addPolynomial degree/evaluation bridge and finite
   point-fibre assembly. These generic statements live in shared
   CryptoLean.Curve.LineIncidence. No new curve model, cubic engine or group.
6. Inheritance table follows. No characteristic/primality/nonsingularity
   assumption is needed for this geometric upper bound over a finite field.

| Statement/operation | Kind and owner | Inherited line / exact hypotheses |
|---|---|---|
| Curve and line polynomial | Existing Mathlib objects | Affine.Equation/addPolynomial/linePolynomial, unchanged |
| addPolynomial_natDegree | Generic endpoint, shared | addPolynomial_eq, natDegree_neg and existing Cubic degree law; Field F |
| addPolynomial_eval / line_root_iff | Generic bridge, shared | Polynomial evaluation and existing Equation; opens substitution once in the owning evaluation lemma |
| line_x_card | Generic endpoint, shared | Existing card_le_degree_of_subset_roots applied to actual cubic; Fintype F |
| other_line_x_card | Generic endpoint, shared | Known point belongs to root set; card_erase_of_mem and cardinality <=3 |
| other_line_point_card | Generic endpoint, shared | Existing card_le_of_injective applied to x-projection, whose injectivity follows from line equation |
| slope_family_point_card | Generic endpoint, shared | Existing card_biUnion_le and finite-sum bound; known curve point and finite slope family |
| Uniform subgroup admission / conditioning / W-base yield | Hand consumer bridge | SELECTOR-JOINS.md; a priori point-independent slope sets and independent unconditioned point records; not a full formal probability/cost result |

Proof chain: replace addPolynomial by its existing cubic expression; read off
degree3 with the existing cubic theorem; apply root-cardinality; erase the
known root; apply cardinality monotonicity along injective projection; apply
finite-union cardinality. The degree/root/injection/union bounds are inherited,
not re-derived for a concrete curve.

Composite opening is localized to addPolynomial_eval, to connect the existing
polynomial substitution to Equation at arbitrary t. equation_add_iff only
covers the specific addition output x-coordinate and does not state this
arbitrary-evaluation bridge. The subsequent root/count endpoints use that
named bridge and the existing degree/root laws, rather than unfolding the
curve equation again.
