# CT96 — Exact Galois-orbit tower and dense cold-setup obstruction

**2026-11-14. Scientific standing: [pcc:unverified].** A classical conditional orbit-stabilizer theorem, a new fixed-instance irreducibility certificate, and source/target dimension checks. No universal lower bound for arbitrary cross-characteristic computations, no complete ECDLP solver, and no speedup.

## Scope and sources checked first

- Repository: CT80 x-division field, CT84–85 Padé/endomorphism experiments, CT86–93 degree-eight subgroup and degree-24/48 oriented tower, CT94 Frobenius and CT95 degree gate; root PR #3877, stacked PR #4074, CT95 PR #4102; `AGENTS.md`, November 13–14 session logs.
- Cross-repo fallback searches: `isogeny-labs/cryptanalysis` chained-transfer reference and `isogeny-labs/crypto-autoresearcher` report corpus; no overlapping dedicated full-image tower implementation identified. Research-KB MCP not loaded in this runtime and repository KB wrapper not available in the connector context. Search result absence is not a novelty claim.
- External primary sources: Serre open-image theorem as stated in [Mayle–Wang, *On the effective version of Serre's open image theorem*](https://londmathsoc.onlinelibrary.wiley.com/doi/10.1112/blms.13002), and [Larson–Vaintrob on exceptional prime images](https://arxiv.org/abs/1204.0046); [Sage division-polynomial documentation](https://doc-gitlab.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_generic.html), [Sage division-field implementation](https://github.com/sagemath/sage/blob/develop/src/sage/schemes/elliptic_curves/ell_field.py), and classical orbit-stabilizer theory. This result is not claimed novel.

## Theorem: exact degrees for an oriented \(\ell\)-torsion point

Let \(E/\mathbb Q\) be an elliptic curve, \(\ell\ge3\) an odd prime, and assume the **actual mod-\(\ell\) Galois image** is all of \(\mathrm{GL}_2(\mathbb F_\ell)\). Choose \(T\in E[\ell]\setminus\{O\}\), and put

\[
S=\mathbb Q(\langle T\rangle),\qquad L=\mathbb Q(x(T)),\qquad K=\mathbb Q(x(T),y(T)).
\]

Here \(S\) means the fixed field of the stabilizer of the *line* \(\mathbb F_\ell T\), not an arbitrary quotient field of a cyclic subgroup; \(L\) is the fixed field of the stabilizer of \(\{T,-T\}\), and \(K\) is the fixed field of the stabilizer of \(T\).

For the full matrix group, the three stabilizer orders are

\[
|\mathrm{Stab}(\mathbb F_\ell T)|=\ell(\ell-1)^2,\quad
|\mathrm{Stab}(\{T,-T\})|=2\ell(\ell-1),\quad
|\mathrm{Stab}(T)|=\ell(\ell-1).
\]

The first group contains the second, which contains the third. As \(|\mathrm{GL}_2(\mathbb F_\ell)|=\ell(\ell-1)^2(\ell+1)\), orbit-stabilizer gives the exact tower

\[
\boxed{[S:\mathbb Q]=\ell+1,\quad
[L:S]=(\ell-1)/2,\quad [K:L]=2,\quad [K:\mathbb Q]=\ell^2-1.}
\]

Proof: projective lines number \(\ell+1\); the stabilizer of a given line acts transitively on its \(\ell-1\) nonzero generators; passing to x-coordinates identifies just \(T\) and \(-T\), and passing to oriented points distinguishes the two. This is a basic Galois-orbit argument. In particular, the smaller *line/subgroup field* **cannot itself specify the oriented target point** in this canonical full-image tower: identifying a generator costs an additional extension of relative degree \(\ell-1\) (factoring via the x and y stages). This is an algebraic degree statement, not a computational hardness theorem.

For \(\ell=7\) the tower is \(8\to24\to48\), matching the specific CT86–93 construction. **Do not silently infer full \(\mathrm{GL}_2(\mathbb F_7)\) image from these degrees alone.** The fixed-instance irreducibility certificate below independently proves \([L:\mathbb Q]=24\), without this image hypothesis; the earlier actual degree-eight and oriented degree-48 claims remain separately supported by CT86–93.

## New exact certificate: the CT80 seven-division x-field has degree 24

Reconstruct the integral odd division polynomial for the curve \(y^2+y=x^3-x^2-10x-20\) via the CT84 recurrence:

\[
D(X)=4X^3-4X^2-40X-79,\quad
P_3(X)=3X^4-4X^3-60X^2-237X-21,
\]
\[
F_4(X)=2X^6-4X^5-100X^4-790X^3-210X^2-1496X-5821,
\]
\[
F(X)=(D^2F_4-P_3^3)P_3^3-D^2F_4^3.
\]

\(F\) has degree 24, leading coefficient 7 (nonzero modulo 3). The independently implemented finite-field Rabin irreducibility criterion gives, in \(\mathbb F_3[X]\),

\[
X^{3^{24}}\equiv X\pmod{F},\quad
\gcd(F,X^{3^{12}}-X)=\gcd(F,X^{3^8}-X)=1.
\]

The exponents 12 and 8 correspond to the distinct prime factors 2 and 3 of 24. These congruences **prove \(F\bmod3\) irreducible**, hence \(F\) irreducible over \(\mathbb Q\). Thus the particular x-torsion field \(L=\mathbb Q(\alpha)\) indeed has exact degree 24. This supplies an explicit finite-arithmetic proof rather than relying only on a prior irreducibility assertion. Both implementations independently reconstruct the polynomial modulo 3.

## Exact checks of source tower degrees and binary target extension

Two separately written programs enumerate all GL2 matrices at \(\ell=3,5,7,11,13\), with different enumeration methods, and independently check orbit and stabilizer formulas:

| \(\ell\) | \(|\mathrm{GL}_2|\) | line field degree | x field degree | oriented field degree |
|---:|---:|---:|---:|---:|
| 3 | 48 | 4 | 4 | 8 |
| 5 | 480 | 6 | 12 | 24 |
| 7 | 2,016 | 8 | 24 | 48 |
| 11 | 13,200 | 12 | 60 | 120 |
| 13 | 26,208 | 14 | 84 | 168 |

For CT94's fixed binary reduction, \(\pi\) obeys \(\pi^2+2\pi+[2]=0\). Python computes matrix powers and the first kernel of \(\pi^d-1\) on \(E[\ell]\); independently, JavaScript works in \(\mathbb F_\ell[t]/(t^2+2t+2)\) and the Newton trace recurrence for \(\#E(\mathbb F_{2^d})\).

| \(\ell\) | first degree having nonzero \(\ell\)-torsion | degree containing all geometric \(\ell\)-torsion |
|---:|---:|---:|
| 3 | 8 | 8 |
| 5 | 1 | 4 |
| 7 | **24** | **24** |
| 11 | 40 | 40 |
| 13 | 6 | 12 |
| 17 | 16 | 16 |
| 19 | 72 | 72 |
| 23 | 88 | 88 |
| 31 | 40 | 40 |

The positive control \(\ell=5\) demonstrates that **large extension degree is not universal** even for this fixed binary fiber; the \(\ell=7\) degree-24 requirement is exact. The full field degree at \(\ell=13\) is 12 while nonzero torsion already exists at degree 6; do not conflate these.

## Consequence for cold dense arithmetic and its scope

The usual odd \(\ell\)-division polynomial over characteristic zero always has formal degree \((\ell^2-1)/2\). Under full Galois image it is irreducible; no representation as a *smaller rational number field containing the x-coordinate of that same geometric torsion point* is possible. CT80's x-field has 24 dimensions, subgroup field 8, and oriented field 48. A dense explicit quotient-basis construction of the x-field writes at least \((\ell^2-1)/2\) coefficient slots, and an oriented power-basis field writes at least \(\ell^2-1\) slots. At \(\ell\asymp2^n\), this is \(\Omega(2^{2n})\) slots **for this explicit dense representation route**. If \(\log p\) is polynomial in \(n\), this setup alone eventually dominates an ordinary \(O(\sqrt\ell\,\mathrm{poly}(\log p))\) generic rho algorithm under the stated word-write model.

This is **not** a lower bound on implicit, sparse or circuit representations of the division polynomial, on all number-field algorithms, on arbitrary transfers, or on the actual memory of lazy towers. Division polynomials have succinct recurrences. Even lower relative-degree subgroup towers do not eliminate the multiplicative total degree when an oriented point must be represented *densely*. No cold end-to-end timing has been run or asserted. The source field in CT80 remains tiny enough to serve as a correctness fixture.

## Reproduction and verification

From the repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct96
python3 -B "$D/ct96_tower_certificate.py" >/tmp/ct96-py.json
python3 -O -B "$D/ct96_tower_certificate.py" >/tmp/ct96-pyopt.json
node "$D/ct96_tower_independent.js" >/tmp/ct96-js.json
node --no-opt "$D/ct96_tower_independent.js" >/tmp/ct96-jsopt.json
cmp /tmp/ct96-py.json /tmp/ct96-pyopt.json
cmp /tmp/ct96-py.json /tmp/ct96-js.json
cmp /tmp/ct96-py.json /tmp/ct96-jsopt.json
printf '%s  %s\n' 7dfe6c347eefb21353b71be3d2f325e18fb7c46909c8fb6f01a817190eb09c52 /tmp/ct96-py.json | sha256sum -c
```

Producer-local Python / Python -O / Node / Node --no-opt passed on the same JSON and SHA-256 **7dfe6c347eefb21353b71be3d2f325e18fb7c46909c8fb6f01a817190eb09c52**. Dedicated GitHub CI and independent PCC signatures are distinct gates. No heavy computation or paid EKS launch.

## Next strongest unblocked lead

Implement a *polynomial-factorization-based*, scalar-independent global point recognizer over the fixed field \(K\), not a family of preselected \([m]\)-maps. On the present seven-torsion fixture this should isolate a unique root using its **source local residue at one fixed place**, return exact algebraic coordinates in \(K\), then evaluate the **same global element at the fixed binary prime** with y-orientation. Record when source Hensel data or global polynomial factorization is insufficient to resolve the joint embeddings and do not silently choose a target root by its log.

For full cold cost, charge polynomial/number-field setup, factorization, root identification, residue embedding, degree-24 binary arithmetic and destination DLP recovery. A viable scalable route must either bypass the dense \(\ell^2\)-degree torsion-field representation or exploit a rigorously verified different structure. Do not declare an ECDLP speedup from this certificate. [pcc:unverified]
