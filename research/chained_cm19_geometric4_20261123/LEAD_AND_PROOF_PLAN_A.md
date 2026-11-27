# Geometric degree-four cover: prospective proof and qualification plan

PR: https://github.com/isogeny-labs/crypto/pull/5095. Parent: crypto5080
head9d8a60f221e2c0c8f47c95402682eadab50b68ce. This is a new geometric
question about the actual cover, not a rerun of the completed E[3], J[4],
conic or finite-frame populations. All conclusions below are hypotheses
until their dependencies and every case have been checked. [pcc:unverified]

## Exact objects and inherited evidence

E/F7 is y^2=x^3+2x+6; pi^2+3pi+7=0. The actual smooth plane quartic is
C: X^4+4XY^3+2Y^4+5X^2Z^2+6XYZ^2+2Y^2Z^2+Z^4=0.
Pminus=(1:0:6), Pplus=(1:0:1). The inherited actual beta:E^3->Jac(C)
is an F7 isomorphism, not just a module classification: the complete
3-torsion conic divisors certify division of the actual Hom element by3.
See ACTUAL_H_BASIS_PROOF_A.md / PCC-20261123-05a984 at crypto5072,
and its native input PCC-20261123-d9f917. [pcc:unverified]

The actual f0 has degree2 and deck T=diag(1,1,6), f1=f0 composed with
R=[[1,5,2],[0,5,0],[2,1,1]], and f3 is the degree3 quotient by R.
The complete F7 canonical-frame census (PCC-20261123-941363) gives
Aut_F7(C)=S3, with every involution an elliptic quotient. It did not
classify extension-field automorphisms. [pcc:unverified]

L=ell beta^-1, ell=(omega,1,0), omega=pi+2, is a genuine integral Hom,
L L^dagger=[4]. The actual f4(P)=L([P-Pminus]) has degree4 and satisfies
3L=pi f0_*+(2pi+3)f1_*-f3_* (PCC-20261123-3c0acb). Its explicit rational
functions remain open. The inverse3mod11 finite-point expression alone
has global degree576, not4 (PCC-20261123-38ae70; retain its immutable
original prose-lint negative). Cryptanalysis1961 gives a different
univariate-coordinate correctness audit, not a full geometry proof.
[pcc:unverified]

## Proposed dependency A: Frobenius-power descent

On actual J, F^2+3F+7=0 follows by the inherited F7 beta. Write
F^n=a_n+b_n F with a_0=1,b_0=0 and a_(n+1)=-7b_n,
b_(n+1)=a_n-3b_n. For n>=1, a_n=0 mod7 and b_n=4^(n-1) mod7,
so b_n is never zero. A geometric group endomorphism h is defined over
some F_(7^n), hence commutes with F^n. Torsion-free Hom then forces
hF=Fh. Frobenius naturality and its surjectivity force h^(7)=h, hence
F7 descent. Faithfulness of the action of nonhyperelliptic C on canonical
forms then descends all geometric curve automorphisms. Check these
standard hypotheses against primary sources. This says nothing about
translations as variety automorphisms. [pcc:unverified]

## Proposed dependency B: geometric primitivity

A geometric 2-by-2 factorization has a quadratic deck involution. A would
make it one of the three actual F7 involutions. Its quotient is an F7
genus1 curve with a rational point, and uniqueness descends the remaining
degree2 map to E. A dual rational2-isogeny would require an F7 order2
point on E, contrary to #E(F7)=11. No geometric primitivity conclusion
is registered until this argument and A have been checked. [pcc:unverified]

## Proposed dependency C: exclude every A4 case

A separable primitive degree4 cover has geometric monodromy A4 or S4.
Char7 is prime to24, so all inertia is tame. If geometric A4 had
arithmetic S4, odd Frobenius at an unramified rational value has0 or2
fixed sheets. Every nontrivial A4 inertia has two orbits; a rational
branched fibre also has0 or2 rational points. This would make #C(F7)
even. Independently certify odd parity from the actual involution T and
its one rational fixed point, without replaying the 17-point census.
[pcc:unverified]

Thus an A4 alternative would be regular over F7. Its V4 quotient gives
a cyclic3 cover D->E. Riemann-Hurwitz gives exactly two geometric branch
values for the degree4 cover, each of defect2. Classify the number of
3-cycle branch values: zero gives an etale genus1 degree3 isogeny,
excluded by the irreducible F3 Frobenius polynomial X^2+1; one is
excluded by Kummer valuations and the degree of a principal divisor;
two gives ramification 2R+2S and a rational theta class R+S. Since
J(F7) has odd order, its rational theta class must be Pminus+Pplus,
the bitangent Y=0. Nonhyperellipticity makes its effective divisor
unique. But f4 should be etale at both Pminus and Pplus: T swaps them,
R fixes them, df0 and df1 are nonzero, df3=0, and differentiation of
the actual Hom identity gives 3df4=3df1-df3. Check all implications,
including constants, descent, unique effectiveness and rational points
on D, before stating the S4 theorem. [pcc:unverified]

## New bounded correctness/proof inputs

Prepare a native proof unit on the natural24 permutations of four letters.
Enumerate every subgroup by closure under adjoining each element, not
an unsupported two-generator shortcut. Check primitive subgroup cases,
all odd fixed-sheet counts, all A4 inertia orbits and the three-pairing
quotient V4. Independently check actual quartic smoothness by split
critical polynomials, the T-fixed parity polynomial, and two curve jets.
This is a fixed algebraic/case certificate, not a sampled experiment,
performance benchmark or an ECDLP advantage measurement. [pcc:unverified]

Formalize the quantified integer-coefficient recurrence using the pinned
Lean4.35.0 standard Omega tactic on actual Int/Nat structures. The desired
leaf is forall n, b_(n+1) != 0, with a mod7 invariant. Do not substitute
custom toy groups for the actual endomorphism/Jacobian hypotheses. All
bridges from that arithmetic leaf to actual Frobenius, canonical action,
curves and monodromy remain readable proofs and explicit open formal
obligations. The earlier unchanged Mathlib2GiB ENOMEM is not retried.
[pcc:unverified]

Publish every source/protocol/runtime/input lock and read every byte back
before first strictformat/compiler/Clippy/native/Lean calls. Per phase:
120s,2GiB,one worker; Rust1.99.1 -Dwarnings opt3 overflow/debug assertions,
Lean4.35.0 -j1 warningAsError with prior successful Core startup settings.
Preserve the first failed or successful complete small originals and
whole original ELF/olean custody. New source versions for repairs; do not
weaken limits, gates, statements or criterion. [pcc:unverified]

## Sources checked (KB)

Tool discovery after restart found no kb_status/search_knowledge/
search_theorems or Conductor tool. The CLI fallback enters the explicitly
locked crypto-autoresearcher VLT1 layout; never read, decrypt or execute
it. Exact inherited repository sources and primary Milne/Stacks sources
are the available fallback, not novelty or absence clearance. The lead
is a proposed proof here, with no priority claim. The first Milne web
search misidentified 2005aX.pdf: opening showed Introduction to Shimura
Varieties, so that hit is rejected rather than cited as an AV source.

## Persistent boundaries and continuation

No ECDLP cost improvement follows from S4 or geometric primitivity. A
successful theorem would constrain factorization strategies and explicit
function extraction. Next inspect actual rational fibre divisor classes,
then genuine function-field/Riemann-Roch extraction. Benchmark only after
a concrete computational mechanism and approved experiment checkpoint;
match the strongest existing rho reference. Same-prime proper-subfield
descent remains impossible. Preserve all contrary or inconclusive cases.
[pcc:unverified]

PCC is unsigned, append-only, and the crypto main fork after row1804 stays
blocked. Do not merge/re-anchor frozen ledger history. PaidSTOPe42949,
EKS-only workload owners and locked vault remain unchanged. Parent5080
CI has reviewed failures; next unchanged review>=2026-11-24T02:30:22.794477Z.
Cryptanalysis1961 head6d99cfc became ready22:55:53Z; no first CI query
before23:45:53Z. No auto-merge ahead of all applicable requirements.
