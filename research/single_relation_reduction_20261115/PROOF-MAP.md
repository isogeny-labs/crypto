# Proof map and inheritance review

## Timing and verification scope

For A1/A2, the source/theorem searches and prospective scope commits preceded
the Lean implementation. The required lean-instance-gate six-field writeup
and assembly probe were completed after the builds, during final review. This timing does
not satisfy the skill's pre-edit ordering and is recorded as a process gap,
not a preregistration. No source or failed diagnostic is replaced to hide it.
The mathematical source/axiom gates are separate completed checks.

Lean4.35.0; Mathlib cfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Shared
coverage source2e4b08b, secant source867527c, subset Fourier source5b7a9d2.
Exact full pins/hashes are under checks/coverage-laws/, checks/line-laws/ and
checks/interval-laws/. Canonical gates134/141/149theorems,0failures and
0trusted native exceptions, respectively. Full audit inventories
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

## Prospective A3: finite subset return count

This plan is written before the first A3 Lean edit, distinct from the
retrospective A1/A2 review above.

1. Structure above: Mathlib finite AddCommGroup G; AddChar G complex-valued
   characters; Finset subsets/products; complex finite sums, conjugation and
   norms. The curve remains the existing Weierstrass model only in the hand
   analytic application.
2. Laws used as stated: AddChar.sum_apply_eq_ite and card_eq (finite Pontryagin
   duality); map_add_eq_mul/map_neg_eq_inv/norm_apply; finite sum distribution,
   norm_sum_le and multiplication of nonnegative real inequalities.
3. Instance: Fourier coefficient of an actual finite subset S is the finite
   sum of a character over S. Return count is the number of actual triples
   x,y,z in S with x+y=z, equivalently the existing addConvolution summed on S.
   No new group representation or elliptic curve is introduced.
4. Assembly probe: before the generic implementation, type-check applications
   of existing character orthogonality/cardinality, norm and finite-sum laws.
   Retain source/log under checks/interval-laws/. Then instantiate the generic
   target at semantic subsets. This probe alone is not the target theorem.
5. Missing generic assembly: finite-subset Parseval, Fourier representation of
   the actual triple count, and a norm bound on its nontrivial-character sum.
   These belong in a shared crypto-lean module, not a study-specific law.
   The Kohel--Shparlinski x-coordinate analytic input is not in these inspected
   Mathlib sources and remains an explicit unformalized curve bridge.
6. Inheritance/statement table and proof chain follow. No limit override or
   native evaluation. Do not present a conditional Fourier theorem as a full
   formalized elliptic mixing result.

| Proposed component | Kind / owner | Inheritance or hypotheses |
|---|---|---|
| Group, characters, subset | Existing Mathlib objects | AddCommGroup/Fintype, AddChar, Finset |
| Character sum | Generic definition, shared | Existing finite sum of actual character values |
| Orthogonality and character cardinality | Inherited laws | AddChar.sum_apply_eq_ite/card_eq; no re-proof |
| Subset Parseval | Missing generic assembly, shared | Expand two finite sums, use conjugation/map_sub and the existing orthogonality law |
| Triple Fourier identity | Missing generic assembly, shared | Expand three finite sums, replace character sum by existing orthogonality; count actual x+y=z triples |
| Flatness-conditioned return bound | Generic endpoint, shared | Explicit M>=0 and norm(character sum)<=M for all nontrivial characters |
| x-interval flatness and subset size | External analytic bridge, hand | Kohel--Shparlinski Theorem1/Corollary1 and interval L1 Lemma3, only f=x and p>3 |
| Tree interpretation | Model-specific hand consequence | Independent uniform subset points; no claim for correlated lists or adaptive selection |

Proof chain: expand the finite character coefficients; use existing
orthogonality to replace the character sum by the zero-test/cardinality;
restructure to the actual pair/triple counts. In the triple formula isolate
the trivial coefficient |S|^3. Bound the remaining norm by the sum of cubed
coefficient norms; replace one factor by M; replace the squared-norm sum by
the derived finite-subset Parseval identity. Cancel only the positive group
cardinality. The curve character input and sampling interpretation are not
silently imported into this algebraic theorem.

Composite opening: the new generic subset coefficient is a finite sum by
definition; its owning Parseval/triple assembly lemmas open it to connect
with existing orthogonality. No curve equation, character internals or
external analytic bound is unfolded or axiomatized. The consumer will apply
the resulting endpoint rather than copying the finite Fourier proof.

### A3 actual result and remaining bridge

At5b7a9d2a9a2638a1251a805d20d31349c35fa904 the generic actual-subset
energy, Parseval and triple identities and both deviation endpoints pass.
Canonical all-source/axiom gate:149theorems,0failures,0trusted native.
The module contributes8audited declarations including2definition equations;
all six named laws' axiom prints list only propext, Classical.choice and
Quot.sound. Claims730397/ab6d22 remain unverified. [pcc:unverified]

The pre-edit floor-law probe passes. The final endpoint probe applies the
same law to generic subsets and Mathlib's actual Weierstrass affine-point
group, with finiteness/flatness explicit. Both probe sources/logs and all four
build versions are in checks/interval-laws/. Build001's sum-binder/cast issue
and003's cancellation-lemma-shape failure are retained;002and004pass their
respective targets. No endpoint was weakened or limit raised. The runtime
has no lean4checker executable, so no separate --fresh checker pass is claimed.
[pcc:unverified]

The full curve application is still partial formalization: the inspected
Mathlib modules do not contain the Kohel--Shparlinski elliptic character
estimate/interval L1 theorem, and this source does not formalize those
analytic proofs or the actual solver's output law. INTERVAL-JOINS.md uses
these as cited external hand bridges. The finite-group theorem is not a
formal proof of all elliptic interval selectors or their costs.
