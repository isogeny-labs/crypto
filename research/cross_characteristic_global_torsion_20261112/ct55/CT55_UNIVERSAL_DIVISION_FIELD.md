# CT55 — Universal existential cross-characteristic torsion bridge by CRT and division fields

**Status:** [pcc:unverified]. Extends CT51–54 in crypto PR #4015; do not conflate existence with an efficiently computable DLP algorithm. Tested cases are public toy instances; no performance claim.

## Exact mathematical statement

Let p >= 5 be an odd prime, and let E/F_p: y² = x³ + a x + b have nonzero discriminant. Let ell be an **odd prime** distinct from p, and let P,Q ∈ E(F_p)[ell] with Q = [x]P. Choose even integers A,B with A = a (mod p) and B = b (mod p), e.g. each of {a,a+p}, {b,b+p} with even parity. Define over Q

    Etilde: y² + p*y = x³ + A*x + B.

Then:

1. Its discriminant is D = -64*A³ - 27*(p²+4B)². Modulo p this is -16(4a³+27b²) != 0, and modulo 2 it is 1. The reduction at p is the original E, while reduction at 2 is the fixed **supersingular** curve E0/F2: y²+y=x³. Both are good reductions.
2. Let L = Q(Etilde[ell]) be the full ell-division field, which is finite Galois with Gal(L/Q) a subgroup of GL_2(F_ell) and hence [L:Q] <= (ell²-1)(ell²-ell). Choose a place v over p and a place w over 2. Because ell is coprime to both residual characteristics and both reductions are good, the reduction maps on all geometric ell-torsion are Galois-equivariant bijections. The source pair P,Q has **unique** preimages Ptilde,Qtilde ∈ Etilde(L)[ell] under red_v (interpreting geometric torsion and the chosen embedding). Uniqueness proves Qtilde = [x]Ptilde. Injective red_w now proves Q2=[x]P2 and ord(P2)=ord(P)=ell.
3. Thus T_(v,w) = red_w ◦ red_v^{-1} defines a group injection E(F_p)[ell] → E0(k_w)[ell] (in fact a restriction of an isomorphism of full geometric ell-torsion). It is NOT an algebraic morphism of elliptic curves across different characteristics, and it depends on the chosen places/orientations.
4. More precisely, let F be arithmetic Frobenius of E0/F2 acting on its ell-torsion. Since #E0(F2)=3, its trace is 0, determinant 2 and its characteristic polynomial is X²+2; Cayley–Hamilton gives F² = [-2]. For **odd ell**, F^k=I cannot hold for odd k, because its trace would be 0 rather than 2 modulo ell. Therefore its exact order is

       f_full = 2 ord_ell(-2).

   At every place w | 2 of the full **Galois** ell-division field L, inertia is trivial and the residue degree is exactly f_full; all of E0[ell] is rational over F_(2^f_full) and not all is rational over any proper subextension. A chosen oriented point may be rational over a smaller extension (cf. CT54).

For p=3, the same existence theorem holds after writing the source in *generalized* Weierstrass form and applying coefficient-wise CRT to force the model (a1,a2,a3,a4,a6)=(0,0,1,0,0) (mod 2). The explicit short-model formula above, and the implementation below, are scoped to p>=5.

**This is standard finite-étale/division-field theory assembled into an explicit uniform curve-lift recipe; no originality claim for the theorem.** It resolves existential feasibility but not the hard algorithmic lift.

## Reproducible finite-stage audit

Two independent stdlib implementations instantiate the CRT lifts and run finite group orders and Frobenius computations, without constructing L. Python uses Legendre-symbol counts, direct elliptic-group additions, and the #E(F_(2^f)) recurrence. JavaScript separately enumerates source points and computes the order of the full Frobenius **matrix** [[0,-2],[1,0]], and checks point-count residues. Both verify the deliberately wrong CRT-parity control (it changes #E(F2) from 3 to 5).

    python3 -B research/cross_characteristic_global_torsion_20261112/ct55/ct55_universal.py
    python3 -O -B research/cross_characteristic_global_torsion_20261112/ct55/ct55_universal.py
    node research/cross_characteristic_global_torsion_20261112/ct55/ct55_independent.js
    node --no-opt research/cross_characteristic_global_torsion_20261112/ct55/ct55_independent.js

Normal/alternative-mode output matches bytewise for each language. All 9 source cases (p,ell)=(5,3),(7,5),(11,7),(13,3),(17,5),(19,7),(23,11),(29,5),(31,7) supply a nonzero exact-order-ell P, have a nonsingular integer CRT lift at both primes, and their binary torsion-extension boundaries agree independently.

| ell | Smallest f with **some** ell-torsion over F_(2^f) | Exact f_full, all ell-torsion |
|---:|---:|---:|
| 3 | 1 | 2 |
| 5 | 8 | 8 |
| 7 | 12 | 12 |
| 11 | 5 | 10 |
| 13 | 24 | 24 |
| 17 | 16 | 16 |
| 19 | 9 | 18 |
| 23 | 44 | 44 |
| 31 | 20 | 20 |
| 37 | 72 | 72 |
| 41 | 40 | 40 |
| 43 | 7 | 14 |

These are **finite-stage and exact algebra controls**, not independent global division-field construction or a full end-to-end measured solve. The previously committed CT51 tests provide a separate explicit 3-torsion lift in degree eight, but the CT55 models are different and were not transferred by the programs in this directory.

## Explicit positive and falsifying toy controls

The **genuine global** three-torsion control is the first CRT curve at (p,ell)=(5,3):

    Etilde/Q : y² + 5y = x³ + 6,
    T=(0,1), [2]T=(0,-6), [3]T=O.

Both primes 5 and 2 are good. The three exact rational points O,T,2T reduce to three distinct group elements modulo 5 and 2. The transfer is a homomorphism on all nine ordered addition pairs, and independently implemented finite-field target solvers recover all three planted public logarithms. This is an explicit **3-element** prime-to-binary DLP test, not scalable. Python recomputes rational additions using Fraction arithmetic; JavaScript independently checks all source and destination group laws and logs.

The **naive coordinate-lift falsifier** is the second CRT curve at (p,ell)=(7,5):

    Etilde/Q : y² + 7y = x³ + 8x + 8,
    R=(0,1).

The rational coordinate point R is on Etilde and its reduction R7=(0,1) on E(F7) has **order 5**. Yet the same coordinate point reduces to R2=(0,1) on E0(F2), which has **order 3**; a rational calculation also confirms [5]R != O on Etilde(Q). Thus the map that merely lifts a source point's coordinates into an integral model and reduces again **does not preserve the original subgroup order or its logarithms**. The unique *ell-torsion* global lift in the full division field is a different algebraic point. Independent Python/JS negative controls reject this incorrect shortcut.

Reproduce:

    python3 -B research/cross_characteristic_global_torsion_20261112/ct55/ct55_toy_controls.py
    python3 -O -B research/cross_characteristic_global_torsion_20261112/ct55/ct55_toy_controls.py
    node research/cross_characteristic_global_torsion_20261112/ct55/ct55_toy_independent.js
    node --no-opt research/cross_characteristic_global_torsion_20261112/ct55/ct55_toy_independent.js

All modes passed locally and the first two result lines match between independent implementations. Scientific status remains [pcc:unverified]; the toy success does not solve the encoded-input global lift.

## Asymptotic obstruction and missing implementation

ord_ell(-2) | ell-1, so f_full <= 2(ell-1). This bound does not provide a uniform polylogarithmic guarantee. Examples with f_full=44 (ell=23) and 72 (ell=37) already show linear-sized binary field degrees at those orders. Full division-field degree has a general O(ell^4) upper bound and attains that order for full mod-ell Galois images; this is **not** a lower bound for all engineered low-image families. This construction supplies no lift of arbitrary *encoded* points in time poly(log ell), and no complete source-to-target solver or matched-rho advantage.

**Gates for the next run:** (i) construct explicit representation of L, its compatible places v,w, and unique lifts of arbitrary encoded P,Q without computing x, including bit complexity and heights; (ii) independently check that the named orientations correspond to those used by the destination finite-field point engine; (iii) measure full cold cost against matched optimized rho/index calculus. An alternative low-image construction must still pass (i)–(iii). No unapproved compute, no leaderboard row, no claim of a scalable attack.

## Sources and governance receipt

- Prior committed CT51–54 in research/cross_characteristic_global_torsion_20261112/, crypto PRs #3877, #4012, #4015; problems/lift-reduce-second-prime-transfer.md, AGENTS.md, latest dated session log. Separate native genus-two/Cantor work in #3877 was not touched.
- Lecture proof of reduction of prime-to-residue-characteristic torsion as finite étale and unramified: https://websites.umich.edu/asnowden/teaching/2013/679/L08.html (reduction section, Proposition and Corollary). Classical Frobenius trace/Cayley–Hamilton. CT52–54 already cover Galois orbits and Frobenius orientation in the fixed 11a1 family.
- Cross-repo GitHub textual search for universal division-field and CRT bridge across crypto, cryptanalysis and crypto-autoresearcher; shared research-kb MCP not loaded in the connector runtime, repository wrapper not executed; **KB unavailable**, no negative prior-art/novelty inference.
- Validation: local Python normal/optimized SHA-256 d88e996f281029a015a91deed05b3efea7194ca08cb80d549bf8292c94f6ae21; local JavaScript normal/--no-opt SHA-256 89d93417d198d12f5ec18fae90833d8b42befd4d30c3039db5e8bba1c2002855; all 12 (f_some,f_full) tuples agree cross-language. **Exact-head CI and independent PCC have not yet been confirmed.** Local checks are not cryptographic proof certificates.
