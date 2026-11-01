# R59: normal fourth-order jets and complete stable finite-support exclusions

## Derived and checked
The actual normal source algebra B has rank32 over F2[z]. The commuting
A/conjugate annihilator product is f I with f=z^5+z^2+1. Its fourth power
shows f^4 B lies in A^4 B. Thus the 640-coordinate presentation over
F2[z]/(f^4), f^4=z^20+z^8+1, computes B/A^4 without truncation loss.
The exact cokernel has dimension320, certified by E M4=0, E G=I320,
and M4 H+G E=I640. The quotient A operator has successive ranks
320,240,160,80,0, with a complete invertible Jordan model. Each of the
four source-polynomial-labelled CRT blocks has dimension80.

All100 R58 numerators are actual normal source functions: full field
reconstruction and 3,200 denominator-unit checks pass. The old160 reduced
columns agree exactly after passing to B/A. Finite evaluation has rank100.
The source geometry interpretation uses the retained reduced degree20
points and local-DVR premises from R55/R56; it does not assert saturation
of the separate incidence/projective ideal.

## Complete finite cohort
Let P be the union of the four degree20 points and x=f/A^2 with
f in the complete100-dimensional L(2D). An ordinary target map with
x,t=y/x in L(2P) and the prescribed F2-stable fibers requires
ord_T(f)>=4 and ord_outside(O union T)(f)>=2. All18 possible patterns
are present: twelve ordered one-component pairs of degree20 and six
complementary two-component partitions of degree40. Each constraint
matrix C has rank100. Each retained certificate includes H C=I100,
so no nonzero f exists, even after extension to F32. No scalar/AS branch
remains for these18 patterns. The global-regular positive control has
rank99 and its one-dimensional kernel contains the known A^2.

The separately invoked checker recomputes all matrix products by row-dot-
column parity, whereas the producer uses column XOR. It checks all18
complete kernel certificates, full cokernel recovery, source operators,
CRT/Jordan models, old columns and the100-column injection. A changed
projection fails. This is certificate replay, not an independent scientific
review.

## Failures, proofs and sources
Five workflows: four accepted, one rejected. data59-01 omitted linalg.lib;
its unavailable unitmat errors and late PASS are retained and rejected.
The repaired data59-02, Rust producer, checker and Lean job all pass.
All31 executed Git-source pins match their historical revisions.
Five authored generic Lean claims compile and audit with zero failing or
trusted declarations. See PROOF_MAP.md for native premises.

Method sources: R55 source-annihilator and CRT model; R52 normal table,
nu certificate and coordinate bridge; R58 full100-space/source functions;
Singular official source documentation for linalg helpers, as recorded in
SESSION.md. No novelty or performance claim is made. The calculations
complete the prescribed normal-jet and finite-cohort obligations.

## Next construction
The four actual infinity points have degree2 and pole caps56,8,14,2.
The complete infinity-fiber cohort is different and remains available.
NEXT.md specifies its mixed pole bound, both trace parities, all404
balanced signed ramification patterns and exact whole-source checks.
