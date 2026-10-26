# Round 13 frozen protocol

Client date 20 October 2026, America/Los_Angeles. Parent commit
7fde2b9498ec30ea2e111f1b178ae000e465331d. Earlier evidence is immutable.
All arithmetic, search, factorization, record assembly and hashes are native
Rust. There is no IC timing or efficiency assessment.

| Requested track | Acceptance criterion | Initial status |
|---|---|---|
| Ordinary antidiagonal degree 500 | All source closed points, local lengths and images, exhausting degree 500 | Open |
| Ordinary product-plus-one degree 580 | All source closed points, local lengths and images, exhausting degree 580 | Open |
| Full quotient saturation | Saturated ideals of the four actual systems, verified projective boundary and gluing | Open |
| Relevant Lean proofs | Faithful checked structural and actual-data certificates; written bridges explicit | Open |

## O13: exact pushed-down Hecke norms

The two distinct 11-kernels determine a cyclic-121 isogeny between the
two targets. Forgetting the full-level scalar gives a degree-five etale
cover on the ordinary noncuspidal locus. Prove and identify this bridge;
do not equate a target divisor with its unresolved source lift.

On X0(49), use x(q)=q^-2 H(q)/H(q^49), H(q)=product(1+q^n), and the
retained eta-to-minimal-y formula. Construct the cyclic-121 norms of
x(P)+x(Q) and 1+x(P)*x(Q). The branches are q^121, the ten
zeta11^b*q (b=1,...,10), and all 121 roots of q. Use a tower of two
11-by-11 multiplication determinants in truncated F2 power series for
the last norm. Use actual F1024 coefficients for the middle ten branches.

Freeze output precision at 640 coefficients, input precision 121*640.
The expected pole bounds at infinity are 504 and 528. Recover the rational
functions A(x)+B(x)*y in these Riemann-Roch spaces by their principal
parts; reject a nonzero residual at any checked coefficient. Global norm,
regularity away infinity, the basis and q-expansion injectivity are written
proof obligations, not conclusions inferred from truncation.

Controls: the minimal elliptic equation, norm(1+t)=1+q, multiplicativity
on fixed polynomial fixtures, direct F1024 degree-11 root-of-unity norms,
Euler-product norm identities, and the norm of x over all 132 branches.
No arbitrary field point screen certifies a complete divisor.

Factor the resulting function-field norm to F2(x), with exact product
reconstruction, squarefree decomposition, distinct/equal degree splitting
and Rabin checks of every irreducible factor. Resolve y branches as
residue-field equations. Classify cusp, supersingular and ordinary points
individually, including both branches of common polynomial factors.
Compare with independent round-12 pushed-down ordinary degrees 100 and
116. Only then attempt the five-sheet source lift and projected images;
keep the original requirement open until all source data is certified.

Limits: determinant rank 11 per tower stage; polynomial degree 2048;
4096 deterministic splitting candidates per factor; 100 million polynomial
reduction steps; 20 minutes per native command. Preserve failures and caps.
Precision or scope increases require an amendment before new arithmetic.

## Q13: saturation beyond constant gates

Parse the four original Program DAGs and round-12 constant relations.
Generate quadratic inverse cancellation and reciprocal-chain relations
with explicit h-power membership identities, retaining the original
equations. Search actual h=0 scalar restrictions after all constant-gate
relations, at most 100000 branching nodes per graph/candidate. Enforce
nonbinary constant scaling over K. Check every original scalar equation
and every added relation on each reported model. A surviving point rejects
completeness of this repair; a failed search proves no saturation claim.

Attempt further affine DAG simplification and bounded colon/Macaulay
relations, recording the actual limits reached. No intermediate ideal is
called the full h-saturation. Proper quotient models, boundary gluing and
polarization remain explicit requirements.

## Proofs, evidence and publication

Add faithful Lean norm/conjugation and branch implications, ideal membership
identities and actual-data certificates where feasible. Do not replace
geometry with bookkeeping or custom axioms; keep written bridges explicit.
Pin dependencies and audit transitive axioms. Preserve failed elaborations.
Include source-bound identity certificates for universal identities.

Keep native/root, Lean, replay and site/workflow checks, failed logs,
source/raw hashes, deterministic archive extraction and repack, editable
SVG and inspected Typst PDF. Update the construction problem and canonical
frontier, then commit and update the existing PR. Check its exact head;
do not bypass failed, cancelled or pending CI or unrelated review gates.

## Amendment A: ordinary full-level fibers after exact norm factorization

O14 finds two simple ordinary degree-50 points for the antidiagonal and
four simple degree-29 points for product-plus-one. Before new fiber
arithmetic, freeze O15: work in their exact polynomial residue fields.
Find common 11-neighbor j-invariants for the two prescribed targets using
the retained Phi11. Reconstruct all compatible middle level-49 triples,
then verify both 11-isogenies by transporting the actual two 7-kernels.
For product-plus-one test both signs of translation by T; do not choose
a sign from the norm alone.

Factor division polynomials of degree 24 and 60 over these residue fields.
Use binary absolute traces for equal-degree splitting, with fixed seed
20261020 and at most 4096 candidates per factor. A rational odd-prime
kernel has its x-polynomial reconstructed and checked for invariance under
doubling, target coefficient, and the two exact commuting 7/11 squares.
All candidate middles and rejected sign/transport attempts are retained.
At most 4096 middle triples per ordinary target, fields of degree at most
500, and the existing polynomial degree/reduction/command limits apply.

The five choices of full-level scalar are the five nonzero x-coordinates
in the first 11-kernel, with the second generator fixed by its pairing.
Report the etale fiber algebra K[t]/kernel11(t), its factorization and
Frobenius orbit degrees. Explicit coordinate towers and this written
moduli/pairing bridge must be distinguished from a global Lean model.
Projected images need a separate actual evaluation; finite fiber algebra
or degree exhaustion alone does not evaluate them.

## Amendment B: bounded O16 completion and degree-four colon instances

The first O16 command verified one degree-250 source point and then hit
100 million binary polynomial reduction steps on the second fiber. Keep
fibers-01.log. Before repeating, raise the O16-only reduction limit to one
billion; retain the 20-minute command cap and all degree bounds. Add
write-once per-fiber checkpoints. Other commands retain the original
100-million limit.

Q13 finds actual counterpoints after constants and again after every
generated inverse cancellation. It generates 23/22 inverse relations and
3826/3774 shared-factor quadrics per circuit; the three-leaf associative
linear family is empty, so no test of that family was performed.
Before further saturation arithmetic, Q14 groups products of two Mul
outputs by their four nonconstant input leaves. Generate their quadratic
differences with explicit h^2 membership identities, at most 250000
candidate output pairs per graph. Search for witnesses after all Q13
relations, with the same 100000-node query cap. This is a specified
degree-four Macaulay/colon family, not a computation of the whole space
or a full saturation claim. Preserve exhausted/capped queries separately
from an empty candidate family.

## Amendment C: parsed replay and square-of-linear-form saturation

Before further arithmetic, freeze O18: parse the exported functions,
closed supports and six fiber algebras. Recheck their equations, binary
Rabin conditions, multiplicities, degree totals, division-polynomial
membership, doubling invariance and all four commuting squares per
base point. Reconstruct the q-series from the exported A/B coefficients.
This verifier shares field arithmetic with the producer; it is a separate
parsed execution, not an external implementation audit. O18 retains the
one-billion reduction bound required by O16 and the 20-minute command cap.

Q15 expands DAG additions and constant multiples into K-linear forms,
treating other gates as independent atoms. A square gate of such a form
gives a saturated linear relation if each atom already has a square gate.
Use at most 64 atoms per form, existing K=F2^360 coefficients, and an
explicit conservative h-membership depth. At h=0 expand every such
relation into its 360 scalar linear equations. Search for counterpoints
after all Q13 and all four Q14 repairs, then after all Q15 relations;
retain the 100000-node per-query cap. Empty families, inconsistency of
the restricted linear equations, search exhaustion and caps are distinct.
No such outcome computes the full h-saturation or proves a proper model.
