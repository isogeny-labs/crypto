# Instance gate and proof map (before Lean edits)

## Six fields

1. **Structure above:** Mathlib integers, commutative-ring operations and
   divisibility; shared `CryptoLean.quadraticNorm` from
   `CryptoLean/Curve/Conductor.lean`. The curve basis and its adjoint matrix are
   prior geometric inputs, not new definitions of elliptic curves/Jacobians.
2. **Theorem used as stated:** the positive-multiple bound `Int.le_of_dvd`
   supplies the minimum once positivity and norm divisibility are established.
   Missing generic component: norm divisibility from a reciprocal quadratic
   root and a linear coefficient congruence. Build that component upstream.
3. **Object as instance:** trace=-1, norm=1 in the existing quadraticNorm,
   with n dividing lambda^2+lambda+1 and b+lambda*a. A nonzero pair is required.
4. **Assembly probe:** variables q,n with q>0 and n|q give n<=q by the
   existing Int theorem. Type-check that probe before writing the new law.
   Then instantiate q=quadraticNorm(-1,1,a,b) using the two generic laws.
5. **Missing components:** generic norm factorization/divisibility and positive
   Eisenstein norm. Geometric adjoint transport remains an explicit input;
   no compiled abelian-variety/polarization theorem is claimed from integer laws.
6. **Inheritance:** table below. Lean/Mathlib installation is underway; until
   its successful receipt, every formalization status was pending, not checked.

## Proof chain before source

Write Q=quadraticNorm(t,u,a,b), L=b+lambda*a and
R=1-t*lambda+u*lambda^2. Ring restructuring gives

    Q = a^2*R + L*(u*b+(t-u*lambda)*a).

Replace R and L by their divisibility witnesses to obtain n|Q.
For t=-1,u=1, 2Q=(a-b)^2+a^2+b^2, so a nonzero pair has Q>0.
Apply Int.le_of_dvd without re-proving the positive-multiple bound.
Only then use the prior adjoint matrix to read the polarization coefficient
2Q; its geometric interpretation is a written conditional argument.

## Statement and inheritance table

| Statement/operation | Kind and owner | Existing structure/law | Status |
|---|---|---|---|
| quadratic norm | Inherited definition, shared Conductor | CryptoLean.quadraticNorm | Source read, revision e0e86e13 |
| reciprocal-root norm divisibility | Generic law, shared new module | integer ring, dvd_add/dvd_mul | To build upstream |
| Eisenstein positivity | Generic law, shared new module | squares nonnegative/positive | To build upstream |
| norm lower bound | Endpoint over integers | Int.le_of_dvd applied as stated | Probe pending |
| rotation/norm identity | Generic ring identity | ring operations | Written derivation; checker planned |
| finite eigenline transport | Instance of prior subgroup maps | alpha relation and n-torsion | Prior geometry + written argument |
| polarized degree 2Q | Conditional geometric consequence | prior adjoint matrix | No compiled geometric bridge |
| global embedding at Q=n | Written consequence | coprime adjoint multipliers | Not yet formalized |
| norm-n coefficient construction | Published Eisenstein Euclidean method | primary GLV/Cornacchia literature | Native implementation/qualification pending |

No new maxHeartbeats, native_decide, custom axiom or study-local algebraic
structure is planned. `quadraticNorm` may be opened once in its owning generic
factorization lemma because the existing Conductor API has no factorization
lemma with a reciprocal root; study endpoints must import that law.

## Checked endpoint update

The assembly probe type-checks. Shared source f74b0e8 passes the canonical gate
with115audited/0failing/0trusted exceptions. The study gate audits six theorems:
four intended endpoints and two generated constant equations; all pass.
PCC-20261115-aa17eb and9ff011 remain unsigned. [pcc:unverified]

| Report statement | Exact declaration/evidence | Scope |
|---|---|---|
| Fixed n/lambda congruence | P256MinimalTransport.cubic_root_contract | Existing constants; kernel arithmetic, no new map call |
| Q>=n | P256MinimalTransport.coefficient_norm_floor | Nonzero pair and linear congruence |
| d>=2n | P256MinimalTransport.polarization_coefficient_floor | d=2Q is explicitly assumed; no geometric bridge |
| Coefficients <2^128 | P256MinimalTransport.minimal_vector_coefficient_bound | Norm=n and opposite-sign sector; no existence claim |
| Generic norm law | CryptoLean.quadraticNorm_dvd_of_reciprocal_root | Inherited norm, arbitrary integer reciprocal quadratic |
| Norm-n vector exists | Written Eisenstein ideal/index argument | Not formalized by this round |
| Map/eigenline/native runtime | Planned qualification panel | Not executed |

`PrintAxioms.lean` prints every new/shared endpoint's foundations. Standard
propext/Classical.choice/Quot.sound only; no sorry/custom/native axioms or
limit changes. Independent lean4checker is not available. The three identity
records point to the exact shared source and were serialized with the unchanged
checker after its integer kernel proof; they are not independent signed geometry.
