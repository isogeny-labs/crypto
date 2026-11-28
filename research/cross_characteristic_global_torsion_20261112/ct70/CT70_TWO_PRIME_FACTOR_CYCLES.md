# CT70 — Two-prime factorization/Frobenius compatibility certificate

**Scientific standing: [pcc:unverified].** Exact, bounded certificate for the CT69 degree-eight global 3-torsion point field. This checks a necessary compatibility condition for a characteristic-zero bridge **independently of the encoded-point lookup**. It does not construct a general lift or an ECDLP improvement.

## General exact criterion (good primes away from the torsion order)

Let E/K be an elliptic curve, ell an odd prime, and T a geometric nonzero ell-torsion point whose point field K(T) has degree ell²−1 over K (e.g. the mod-ell Galois image acts transitively on nonzero torsion vectors). Suppose a primitive polynomial H_T for K(T) has an integral, separable reduction at a good prime v away from ell and its index. Then the degrees of the irreducible factors of H_T modulo v are exactly the lengths of the Frobenius-v cycles on E[ell]\{0}. This follows from the finite-etale prime-to-residue-characteristic torsion representation and the usual separable polynomial factorization/Frobenius correspondence. It is a **necessary verification gate**, not a canonical choice of root or orientation.

For an invertible 2x2 Frobenius matrix over F_ell, the cycle profiles are determined by its rational canonical type. Write ord(a) for the multiplicative order in F_ell^*:

- Scalar aI: all ell²−1 nonzero vectors lie in cycles of length ord(a).
- Split distinct eigenvalues a,b: the two eigenlines contribute ell−1 vectors each, of cycle lengths ord(a), ord(b); the (ell−1)² mixed vectors have cycle length lcm(ord(a),ord(b)).
- Nontrivial Jordan block aI+N: its eigenline contributes ell−1 vectors in cycles of ord(a), and the other ell(ell−1) vectors have cycle length ell·ord(a).
- Irreducible semisimple: identifying the plane with F_(ell²), Frobenius multiplication by alpha has all ell²−1 nonzero vectors in cycles of ord(alpha).

These formulas follow by direct matrix powers or the multiplicative action on a field; each count must be divided by its displayed cycle length to obtain factor multiplicity. They apply to the full nonzero-vector field, not automatically to a smaller chosen subfield or input-dependent target curve. This finite permutation calculation is classical and is not claimed novel.

## CT69 field K=Q[Y]/H(Y): explicit two-prime check

Use the **same** integral polynomial H as CT69 (not a fitted replacement):

    H(Y) = 27Y^8+108Y^7−4813Y^6−14817Y^5
         +162543Y^4+349907Y^3+267096Y^2
         +89655Y−101306819.

The two modular reductions are squarefree. The factorizations are:

- Over **F29**, H factors with degrees **[1,1,2,2,2]**. The linear roots are Y=5 and Y=23, corresponding to oriented source P and −P. The full monic factors are (Y−5), (Y+6), (Y²+Y+1), (Y²+3Y+7), and (Y²−Y+5); multiply by leading scalar 27 mod 29.
- Over **F2**, H reduces to the irreducible degree-eight polynomial **Y^8+Y^6+Y^5+Y^4+Y^3+Y+1**, or 0x17b.

At source prime 29, #E(F29)=30 implies trace t29=0. On E[3], charpoly is Z²+29 = Z²−1 mod 3, with split eigenvalues 1,−1. Its action on the eight nonzero vectors has cycles **[1,1,2,2,2]**.

At binary prime 2, #E(F2)=5 implies t2=−2 and charpoly Z²+2Z+2 mod 3, irreducible. The matrix [[0,1],[1,1]] has one **eight-cycle** on nonzero vectors. The irreducible mod-2 H therefore supplies both a degree-eight point field and exactly the predicted residue degree.

The two totally different residue profiles are therefore compatible with a *single global point field* K, at two selected primes. They confirm a necessary condition of CT69. Neither profile selects the target embedding for an arbitrary source Q.

## Independent reproductions and negative limits

The Python implementation computes modular polynomial GCDs with Y^(p^n)−Y, squarefreeness from H and H', then separately enumerates all nonzero F3² matrix orbits. The independently written JavaScript implementation trial-divides by all monic linear and quadratic polynomials over F29 and all monic polynomials of degrees 1–4 over F2, then separately enumerates the Frobenius vector orbits. It does **not** reuse Python's polynomial-GCD or finite-field factoring routine.

From repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct70
    python3 -B "$D/ct70_two_prime_factorization.py" > ct70-py.json
    python3 -O -B "$D/ct70_two_prime_factorization.py" > ct70-py-opt.json
    node "$D/ct70_two_prime_factorization_independent.js" > ct70-js.json
    node --no-opt "$D/ct70_two_prime_factorization_independent.js" > ct70-js-noopt.json
    cmp ct70-py.json ct70-py-opt.json
    cmp ct70-py.json ct70-js.json
    cmp ct70-py.json ct70-js-noopt.json

All four **producer-local** bounded modes were byte-identical, SHA-256 **06014aa0236518ce074ec52bb9c92c703bdacd14de2d46092aa7b0857ce0b3ad**. This is not a PCC independent verifier signature. No changes were made to CT69's algebraic point-field model; CT69's previous exact-head bounded CI completed successfully, while its PCC sign job was skipped.

**Scope/negative result:** this is a consistency *filter*, not a transfer algorithm. For a selected global T, root selection for arbitrary encoded Q=[x]P is unsolved. Reading x out of an ell-entry source lookup merely performs the source DLP; it cannot be counted as a cheap cross-characteristic lift. No complete cold-cost comparison with matched optimized rho or admitted index calculus is available because that scalable encoded-input stage has not been constructed. No GPU, paid cloud, protected merge or large-size security claim.

## Sources, provenance, and research gate

Repository: CT69 exact degree-eight point field and CT67–68 Frobenius orientation/centralizer notes, root crypto [PR #3877](https://github.com/isogeny-labs/crypto/pull/3877), stacked cross-characteristic [PR #4015](https://github.com/isogeny-labs/crypto/pull/4015), cryptanalysis [#1520](https://github.com/isogeny-labs/cryptanalysis/pull/1520)/[#1521](https://github.com/isogeny-labs/cryptanalysis/pull/1521). Prior literature: Jacobson–Koblitz–Silverman–Stein–Teske, [*Analysis of the Xedni Calculus Attack* (2000)](https://doi.org/10.1023/A:1008312401197), for the limitations of point lifting; Snowden's [elliptic curves over DVRs](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html), for prime-to-p torsion specialization. The shared research-KB MCP tools, its checkout wrapper and Conductor were not available in this GitHub-connector runtime. Cross-repo PR search and primary papers are a **nonexhaustive fallback**, not evidence of novelty.

**Next strongest gate:** obtain one input-uniform point-field root-selection/compatible specializations algorithm on public P,Q for growing ell without enumerating the scalar, measure factorization/height/orientation/input/output/all solving costs, and reject any candidate whose Frobenius residue-factor profiles disagree at either prime. [pcc:unverified]
