# CT54 — Frobenius source-prime filter for cross-characteristic torsion

**Status: [pcc:unverified].** This is a checked instance of known Galois/finite-group theory, not a new cryptanalytic transfer. Extends cross-characteristic PR #4015 and its CT52–53 certificates, without changing native PR #3877.

## The two distinct selection events

Let E/K be an elliptic curve over a number field, ell prime, with mod-ell Galois image GL_2(F_ell). Consider primes of good reduction away from ell and the finite ramification set.

**Existence of a rational ell-torsion point.** The reduction has a nonzero point in E(k_v)[ell] precisely when Frobenius has eigenvalue one. Count one-dimensional subspaces L: there are ell+1 lines, and for each line exactly ell(ell-1) invertible matrices act as the identity on L (basis form [[1,a],[0,b]] with b nonzero). Any two distinct fixed lines imply the identity matrix, counted ell+1 times. The union therefore has

    ell(ell+1)(ell-1) - ell = ell(ell^2-2)

matrices. Since |GL_2(F_ell)|=ell(ell-1)^2(ell+1), Chebotarev gives the **base-prime density**

    delta_any=(ell^2-2)/((ell-1)^2(ell+1)) ~ 1/ell.

This is a conjugacy-invariant event and is a classical group count, including in Jones–Rouse.

**A specified oriented global point.** The stabilizer of an oriented nonzero vector has ell(ell-1) elements, hence a uniformly selected Galois element fixes it with probability 1/(ell^2-1). This is NOT automatically a Chebotarev density of primes of K: a fixed labeled geometric point P can be sent to a conjugate by changing the chosen place above a base prime. Its stabilizer is not a conjugacy-stable subset. The fraction is valid as a group-theoretic sample, or after specifying a compatible lifted-place/orientation sampling model. A source prime possessing *some* rational ell-torsion point does not identify a single common algebraic torsion point or its target conjugate.

**Higher-dimensional conditional extension.** If A/K is g-dimensional and the mod-ell Galois image contains Sp_(2g)(F_ell), the action on nonzero A[ell] is transitive (CT53). Burnside's lemma gives the mean number of nonzero fixed vectors under a random group element equal to one. If any nonzero fixed vector exists, all ell-1 nonzero scalar multiples are fixed. Thus the conjugacy-invariant base-prime event A(k_v)[ell] nonzero has density at most 1/(ell-1). A particular labeled nonzero vector has stabilizer fraction 1/(ell^(2g)-1), with the same labeling caveat. This requires the stated Galois image; it is not a theorem about arbitrary globally split or engineered families.

## Exhaustive independent finite controls

Python enumerates all matrices and nonzero fixed vectors directly; JavaScript independently checks eigenvalue one through trace and determinant and separately enumerates affine curve points. Both enforce the group cardinality, the identity and one-line fixed-space counts, the oriented stabilizer count, and a mutant control that would conflate the two events.

| ell | |GL2| | existential matrices | base-prime density (full image) | labeled-vector matrix fraction |
|---:|---:|---:|---:|---:|
| 2 | 6 | 4 | 2/3 | 1/3 |
| 3 | 48 | 21 | 7/16 | 1/8 |
| 5 | 480 | 115 | 23/96 | 1/24 |
| 7 | 2,016 | 329 | 47/288 | 1/48 |
| 11 | 13,200 | 1,309 | 119/1200 | 1/120 |

The rational elliptic curve 11a1, y²+y=x³−x²−10x−20, provides contrasting cases. CT52 derives full mod-3 Galois image (subject to independent scientific review), while its rational point (5,5) of order 5 supplies a **low-image** control. Python's exact Legendre-symbol point counts over all 665 eligible primes p <= 5000, excluding 2,3,5,11, found:

- 294/665 have 3 dividing #E(Fp), compared with asymptotic 7/16 under the CT52 full-image hypothesis. The finite sample is descriptive, not proof of convergence.
- All 665/665 have 5 dividing #E(Fp), as guaranteed by the global rational 5-torsion, in contrast to hypothetical full-image ell=5 density 23/96.
- Independent JS brute-force (x,y) counts on the 51 eligible primes p <= 257 agree with Python: 21/51 for ell=3 and 51/51 for ell=5.

## Reproduce (stdlib, local)

    python3 -B research/cross_characteristic_global_torsion_20261112/ct54/ct54_primes.py
    python3 -O -B research/cross_characteristic_global_torsion_20261112/ct54/ct54_primes.py
    node research/cross_characteristic_global_torsion_20261112/ct54/ct54_independent.js
    node --no-opt research/cross_characteristic_global_torsion_20261112/ct54/ct54_independent.js

Local normal/alternative-mode stdout matched byte for byte by language. Python stdout SHA-256 95bbe8f1094c3765828fdb322208a0d083f93617d9979678b72d2c443e96e43b; JS stdout SHA-256 f49f74e28e0896faa927315d0f39d46341a5c0fc1ec84885bb8043f0e2bc4a7d. Neither is an independent signed PCC verdict.

## Sources reviewed and novelty boundary

- Prior local: CT51–53, problems/lift-reduce-second-prime-transfer.md, cross-characteristic LOCAL_TO_GLOBAL.md, root PR #3877 and existing cryptanalysis cross-factor work. GitHub code searches of crypto, cryptanalysis and crypto-autoresearcher for the exact fixed-vector formula found no match. Research-KB MCP and its repository wrapper were not available in this runtime, and the lead atlas was not fully read; no absence/novelty claim.
- **Primary prior art:** R. Jones and J. Rouse, *Galois Theory of Iterated Endomorphisms*, §5 (page 18), https://www.math.colostate.edu/~achter/math/jonesrouse09.pdf . It already contains the GL2 fixed-eigenvalue-one count. D. Zywina, *On the surjectivity of mod ell representations associated to elliptic curves*, https://arxiv.org/abs/1508.07661 , supplies full-image context. Chebotarev is invoked only for conjugacy-stable conditions.

## Decision and next test

Do not use merely existential source-prime torsion compatibility as a global torsion-lift construction. The strongest unblocked route is still an **adaptive, encoded-input** low-image or split global family: supply explicit global P and Q with Q=[x]P without computing x; identify the same conjugate at both good-reduction primes; charge representation degree, heights and construction cost, then measure the complete destination solver and compare to matched rho. The 11a1 rational-5 example shows why exceptional image is relevant, but does not scale by itself.

No paid compute, end-to-end cryptanalytic speedup, protected merge, independent PCC signature, or general obstruction to varying-A constructions is claimed.
