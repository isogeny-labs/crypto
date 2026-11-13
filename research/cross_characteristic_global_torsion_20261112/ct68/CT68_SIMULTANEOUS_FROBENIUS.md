# CT68 — Complete Frobenius-centralizer taxonomy and simultaneous-prime alignment

**Standing: [pcc:unverified].** This is classical finite-dimensional algebra with bounded exact checks. It is not a novel ECDLP lower bound, a new cross-characteristic homomorphism, or an encoded-point transfer.

## Precise theorem and distinction

For an odd prime ell, let V=F_ell² and F be an invertible two-by-two Frobenius matrix. The changes of *torsion basis* preserving the **literal matrix** F form its centralizer C(F) in GL2(F_ell). Their orders and actions are exactly:

| Type of F | Size of C(F) | Orbit of a nonzero eigenvector (if one exists) | Orbit of vector with both split components / off Jordan eigenline |
|---|---:|---:|---:|
| Scalar | ell(ell−1)²(ell+1) | ell²−1 | — |
| Split distinct eigenvalues | (ell−1)² | ell−1 | (ell−1)² |
| Nontrivial Jordan block | ell(ell−1) | ell−1 | ell(ell−1) |
| Irreducible quadratic | ell²−1 | no F_ell eigenvector | ell²−1 for **every** nonzero vector |

Proof: In the scalar case all invertible matrices commute. The centralizer of a split matrix with distinct eigenvalues is a two-dimensional diagonal torus. For a Jordan matrix F=lambda*I+N with N≠0,N²=0, the centralizer units are a*I+b*N with a≠0. For irreducible F, F_ell[F] is the quadratic field F_(ell²) and its units act regularly on nonzero V. These computations prove the matrix sizes and orbit formulas independently of any elliptic curve. [pcc:unverified]

**Rational-point condition:** A point of ell-torsion defined over the target *base* field must also satisfy Fv=v. When the fixed subspace is a one-dimensional 1-eigenline it contains ell−1 oriented nonzero points; for F=I it contains ell²−1. An irreducible F has no such nonzero base-field point; the CT67 11a1/3 example instead rationalizes its nonzero torsion over the binary extension where F^8=I. This prevents misusing irreducible-F orbit counts as base-field torsion. [pcc:unverified]

## Simultaneous prime information

If F_s and F_t are represented in the **same already identified global torsion basis**, their remaining common-basis symmetries are C(F_s)∩C(F_t), not C(F_t) alone. If F_s is split with two distinct eigenvalues and F_t is irreducible over F_ell, then

\[
\boxed{C(F_s)\cap C(F_t)=\mathbb F_\ell^\times I.}
\]

Proof: Diagonalize F_s. Every matrix commuting with it is diag(a,b). Irreducibility of F_t forces both of its off-diagonal entries nonzero in this basis (otherwise F_t would have an eigenvector over F_ell). Commutation with F_t forces a=b. Nonzero scalars certainly commute with both. The joint residual oriented-vector orbit has ell−1 elements; fixing the literal oriented global vector itself leaves just the identity. [pcc:unverified]

In the CT67 elliptic curve E/Q: y²+y=x³−x²−10x−20, source #E(F_29)=30 gives trace 0 and mod-3 Frobenius eigenvalues 1,2. The target binary trace −2 gives an irreducible mod-3 Frobenius represented by [[0,1],[1,1]]. Its centralizer alone has order 8, whereas the **simultaneous model** with F_s=diag(1,2) has joint centralizer order 2. This is a valid exact matrix-pair illustration; it does **not** compute the actual relative Galois conjugation/prime embeddings needed to express both arithmetic Frobenius elements in a single chosen global basis. Selecting an oriented global P in that basis eliminates the residual sign but is part of the unresolved lifting input, not information freely supplied by the prime-field curve. [pcc:unverified]

Thus CT67's eight possibilities apply to the fixed *target-only* Frobenius model. Jointly aligned Frobenius data can narrow this abstract label ambiguity, but cannot independently construct a globally compatible preimage of encoded Q=[x]P or its binary reduction. A reduction in label count is not a speedup or a DLP cost bound. [pcc:unverified]

## Independently checked executable controls

The Python verifier exhaustively enumerates every GL2 matrix and tests commutation and vector orbits. Independently written JavaScript constructs each centralizer by its algebraic parameterization, and separately checks completeness against GL2. Both additionally check joint centralizers. At ell=3,5,7,11 all four normal/alternate Python and Node executions agreed byte-for-byte; producer-local stdout SHA-256: 473ff606c90c2cd0dc38db1c3c5f47fc95c40a1b2e104c2eae1b9bbaf1a37187. Exact-head CI and independent scientific signing are separate, not established by this local agreement. [pcc:unverified]

Run from repo root with D=research/cross_characteristic_global_torsion_20261112/ct68 and these commands, comparing all four outputs: python3 -B "$D/ct68_frobenius_pair.py"; python3 -O -B "$D/ct68_frobenius_pair.py"; node "$D/ct68_frobenius_pair_independent.js"; node --no-opt "$D/ct68_frobenius_pair_independent.js". The bounded integration workflow is .github/workflows/ct68-frobenius-pair.yml. [pcc:unverified]

## Provenance, boundaries and next rung

Prior work: CT51, CT52 (global mod-3 image), CT54 (source Frobenius filter), CT66 (same-characteristic dual), CT67 (single-Frobenius orientation gate), root crypto PR #3877, cross-characteristic PR #4015; checked related cryptanalysis #1510/#1520/#1521 and crypto-autoresearcher cross-field PR queries. The centralizer theorem is standard linear algebra, not claimed novel. Primary context: Victor Shoup, 1997, https://www.shoup.net/papers/dlbounds1.pdf (generic-group bounds, **not** an unconditional obstruction here); Anthony Snowden, https://websites.umich.edu/asnowden/teaching/2013/679/L08.html (torsion reduction). [pcc:unverified]

KB unavailable: the shared research-KB MCP tools and authenticated checkout wrapper were absent in this connector runtime. GitHub research files and targeted cross-repo searches were the limited fallback, not evidence that no other work exists. No native root Cantor/cryptanalysis code touched; no EKS or other paid compute. [pcc:unverified]

The next decisive test remains a **common globally labeled point-field model for arbitrary encoded source P,Q**, with compatible source and binary prime embeddings, recovery of global algebraic torsion roots **without x**, and priced degree, heights, root selection, same-characteristic dual, target DLP and cold baselines. Complete matched negation/Frobenius rho and admitted index-calculus costs cannot be claimed without that lift. [pcc:unverified]
