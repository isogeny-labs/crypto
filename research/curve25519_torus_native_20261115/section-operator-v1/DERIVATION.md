# Classical divisor reduction and the exact six-section matrix

Producer derivation awaiting independent/formal review. The geometric argument
is classical; native agreement is not a Lean proof or a novel algorithm.
[pcc:unverified]

Let E/Fp be a nonsingular Montgomery cubic of odd characteristic. Let
Z=sum_i[P_i] be an effective rational divisor of degree6, with repetitions,
and D=5[O]+[R]. Riemann--Roch gives dim L(D)=6. Evaluating local sections of
O(D) on the length-six subscheme Z gives a linear map ev_Z from L(D) to a
six-dimensional jet space. Its kernel is exactly L(D-Z). Since deg(D-Z)=0,

    det(ev_Z)=0
    iff L(D-Z) contains a nonzero function
    iff D-Z is principal
    iff sum_i P_i = R in E(Fp).

The middle equivalence: if f lies in L(D-Z), div(f)+D-Z is effective of
degree zero, hence zero. Conversely a principal divisor has such a function.
The last line uses E(Fp)=Pic^0(E)(Fp) with O as identity. Evaluation at a
repeated geometric point means Hasse jets, not repeated value columns.
[pcc:unverified]

For R=(r,s),s!=0, h_R=(y+s)/(x-r) has a simple pole at R and O; at -R the
singularity is removable. The five monomials1,x,y,x^2,xy have pole orders
0,2,3,4,5 at O and none at R. Thus adjoining h_R gives six independent
sections of L(D), hence a basis. For R=O use1,x,y,x^2,xy,x^3 in L(6O).
[pcc:unverified]

At finite P=(a,b),b!=0, take t=x-a and write y(t)=sum y_n t^n. For n>=1,

    y_n = ( [t^n] f(a+t) - sum_(i=1..n-1) y_i y_(n-i) ) / (2b).

These are Hasse/Taylor coefficients without factorial division. At P=R,
multiply local section representatives by t: t,t*x,t*y,t*x^2,t*x*y,y+s.
At P=-R, h_R has coefficient y_(n+1) at order n. Else divide y+s by its
unit denominator a-r+t. Changing a local trivialization only multiplies the
jet conditions by an invertible triangular matrix; kernel existence is
unchanged. The native implementation excludes the y=0 uniformizer branch
and member O; neither is falsely classified as a failed relation.
[pcc:unverified]

## Why two shortcuts are incorrect

Ordinary value evaluations at repeated P give identical columns/rows and a
zero determinant even if the sum is wrong. Distinct raw indices do not imply
distinct cofactor-cleared signed curve points. Confluent jets impose the
required multiplicity and repair this issue.

In a generic affine chart, denominator clearing yields a row

    ((x-r), x(x-r), y(x-r), x^2(x-r), xy(x-r), y+s).

At P=-R it is the zero row, so its determinant vanishes regardless of the
sum. At P=R the local section interpretation differs from ordinary rational
evaluation. Use local trivializations and removable-pole limits; do not add
an unannounced no-target-alias restriction to the original problem.
[pcc:unverified]

## What would move the exponent

On the generic chart, unknown coefficients c0..c5 give six shared-section
constraints

    (x_i-r)(c0+c1*x_i+c2*y_i+c3*x_i^2+c4*x_i*y_i)
       + c5*(y_i+s)=0.

Require a nonzero coefficient vector (six normalization charts or a suitable
projective encoding) and every original membership/index/sign/alias guard.
Confluent and pole charts are part of the full statement, not optional
postprocessing. This changes the algebraic interface, not the proven solve
cost. A singular matrix for already supplied points is a constant-size check;
discovering which six columns are singular is still the original search.
[pcc:unverified]

Splitting into two triples permits a classical20-coordinate Pluecker pairing
for the determinant. Computing such coordinates for all triples still costs
Theta(B^3). Low tensor dimension, a six-dimensional function space, or a
constant-size null-section certificate does not avoid that enumeration.
The prior B^(3-epsilon) total relation-work target remains open.
[pcc:unverified]

## Sources and formalization boundary

- Cheng, Hard Problems of Algebraic Geometry Codes, cs/0507026 section2 and
  Theorem1: the same L((k-1)O+R) subset-sum construction and basis/evaluation
  argument. General NP-hardness has variable k and special reduction instances;
  it is not a fixed-six average-case exclusion.
- Li--Wan--Zhang,1501.01138 section4: elliptic-code/subset-sum equivalence;
  large-density existence results do not apply at B~N^(1/6).
- Cardinal--Sharir, SoCG2023.22: real-RAM/algebraic-decision-tree degeneracy
  testing, not a prime-field solver or a subcubic bound for this operator.
- Snowden Math679 lecture2: Riemann--Roch, principal-divisor degree and Pic^0.
- Prior root-reference-v1 uses classical rank6 uniform-matroid representative
  families per elliptic half sum; those Vandermonde vectors are different
  from these elliptic sections, and still enumerate triples.

KB unavailable (no loaded MCP, scripts/research-kb127). Primary texts read;
failed UCI/Citeseer fetches replaced by author arXiv versions. No novelty
claim follows from the bounded search. Native and geometric proof obligations
are separated in PROOF_MAP.md before any Lean source is written.
