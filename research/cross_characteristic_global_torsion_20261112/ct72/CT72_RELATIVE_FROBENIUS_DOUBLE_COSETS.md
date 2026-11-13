# CT72 — Relative Frobenius alignment: exact split/nonsplit double cosets

**Scientific standing: [pcc:unverified].** This is a bounded arithmetic control and an elementary group-theoretic classification, **not** a general lower bound on ECDLP or an implemented characteristic-changing transfer.

## The precise ambiguity left by CT68 and CT71

Let \(q=\ell\) be an **odd prime**, \(G=\operatorname{GL}_2(\mathbb F_q)\), and fix a split, regular-semisimple source Frobenius \(S=\operatorname{diag}(a,b)\) with \(a\ne b\), and a nonsplit regular-semisimple target Frobenius \(T\) with irreducible quadratic characteristic polynomial \(X^2-\tau X+\delta\), \(\delta\ne0\).

Suppose that only the separate conjugacy classes of \(S\) and \(T\) have been identified, **not** their matrices in a common, globally identified torsion basis. The relative identifications modulo the source and target centralizers are classified by the double-coset space
\[
 C_G(S)\backslash G/C_G(T),\qquad
 \boxed{|C_G(S)\backslash G/C_G(T)|=q}.
\]

**Proof.** The source centralizer is the split torus \(D=(\mathbb F_q^\times)^2\) of order \((q-1)^2\). The target centralizer is the nonsplit torus \(\mathbb F_{q^2}^\times\) of order \(q^2-1\). For every \(g\in G\), \(D\cap gC_G(T)g^{-1}\) comprises only scalar matrices, since a nonscalar diagonal matrix has two distinct rational eigenspaces, which could not be preserved by an irreducible commuting matrix. Thus every \(D\)-orbit on \(G/C_G(T)\) has size \(|D|/(q-1)=q-1\), whereas \(|G/C_G(T)|=q(q-1)\). There are exactly \(q\) orbits.

An explicit invariant is available. Write a conjugate of \(T\) in the \(S\)-eigenbasis as
\[
 A=\begin{pmatrix}u&v\\w&\tau-u\end{pmatrix},
 \quad vw=u(\tau-u)-\delta.
\]
Irreducibility forces \(v,w\ne0\) for every \(u\in\mathbb F_q\). Diagonal conjugation rescales \((v,w)\mapsto(zv,z^{-1}w)\) without changing \(u\). Each \(u\) gives exactly one orbit of \(q-1\) matrices, proving completeness constructively.

For \(S=\operatorname{diag}(1,-1)\), the mixed trace \(\operatorname{tr}(SA)=2u-\tau\) distinguishes all \(q\) alignments. **This mixed trace is not determined by the two individual Frobenius characteristic polynomials**. In particular, writing both matrices in an assumed common basis would silently choose information that the separate local traces do not supply.

## Exact CT67 example and independently executed finite checks

For the source mod-3 split matrix \(S=\operatorname{diag}(1,-1)\) and target mod-3 matrix \(T=\bigl(\begin{smallmatrix}0&1\\1&1\end{smallmatrix}\bigr)\) from CT67, \(\tau=1,\delta=2\). Its six conjugates divide into **three** diagonal-centralizer orbits of two matrices each, indexed by \(u=0,1,2\). The three distinct mixed traces are \(2,1,0\pmod3\). All six have the **same** target characteristic polynomial and share the same source characteristic polynomial.

The remaining cases \(q=5,7,11\) are abstract finite-group controls, **not** claims that their particular \(T\) matrices arise as binary Frobenius of the CT69 curve.

| q | |GL2| | Split centralizer | Nonsplit centralizer | Conjugates of T | Double cosets |
|---:|---:|---:|---:|---:|---:|
| 3 | 48 | 4 | 8 | 6 | 3 |
| 5 | 480 | 16 | 24 | 20 | 5 |
| 7 | 2016 | 36 | 48 | 42 | 7 |
| 11 | 13200 | 100 | 120 | 110 | 11 |

Python enumerates every invertible matrix, forms the full conjugacy class, independently enumerates both centralizers, directly partitions all orbits and verifies the mixed trace and scalar intersection. JavaScript **does not enumerate conjugates**; it independently parametrizes the fixed-trace/fixed-determinant matrices, verifies diagonal-orbit transitivity, and enumerates commuting matrices as a centralizer check. Normal and alternate optimization modes agree byte-for-byte. The producer-local four-mode stdout SHA-256 is **78885691f4964a2d28cdaca83fdb0cefcace6818680f3fc30896f92a875e3fb7**. This is not independent PCC certification.

Reproduce at repository root:

```bash
D=research/cross_characteristic_global_torsion_20261112/ct72
python3 -B "$D/ct72_double_coset.py" > ct72-py.jsonl
python3 -O -B "$D/ct72_double_coset.py" > ct72-pyopt.jsonl
node "$D/ct72_double_coset_independent.js" > ct72-js.jsonl
node --no-opt "$D/ct72_double_coset_independent.js" > ct72-jsnoopt.jsonl
cmp ct72-py.jsonl ct72-pyopt.jsonl
cmp ct72-py.jsonl ct72-js.jsonl
cmp ct72-py.jsonl ct72-jsnoopt.jsonl
echo '78885691f4964a2d28cdaca83fdb0cefcace6818680f3fc30896f92a875e3fb7  ct72-py.jsonl' | sha256sum -c
```

## Interpretation and limitations

CT68's statement that **two specified matrices in a shared basis** have joint centralizer consisting of scalars is correct. CT72 proves the converse inference is invalid: the **separate** local conjugacy classes leave \(q\) relative classes undetermined. The count is an alignment-space cardinality, **not** an information-theoretic runtime bound, a count of distinct ECDLP attacks, a count of geometric isogenies, or an obstacle once explicit global points plus both residue embeddings are supplied. General source matrices with repeated eigenvalues, Jordan blocks, or nonsurjective mod-ell image require their own analysis.

A true encoded-input lift must identify the relative embedding / appropriate \(u\) in a **single actual global torsion basis**, or avoid the requirement by some different construction. No low-cost method to select it for arbitrary \(P,Q=[x]P\) is established. The next falsifiable test is to compute \(u\) from explicit CT69 global residue embeddings without asking for \(x\), then determine whether the construction scales without enumerating the subgroup. Charge field arithmetic, root choice, point-field degree, global height and target solving cost.

## Prior work and literature receipt

Repository sources checked: [CT68](../ct68/CT68_SIMULTANEOUS_FROBENIUS.md) (joint centralizer after a common basis is supplied), [CT69](../ct69/CT69_EXPLICIT_POINT_FIELD_BRIDGE.md), [CT70](../ct70/CT70_TWO_PRIME_FACTOR_CYCLES.md), [CT71](../ct71/CT71_POINT_FIELD_STABILIZER.md), [root PR #3877](https://github.com/isogeny-labs/crypto/pull/3877) and [this PR #4015](https://github.com/isogeny-labs/crypto/pull/4015); related [cryptanalysis PR #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523). Cross-repo GitHub code/PR searches covered crypto, cryptanalysis and crypto-autoresearcher. Primary mathematical background: [Snowden, elliptic curves over DVRs](https://websites.umich.edu/asnowden/teaching/2013/679/L08.html) for prime-to-characteristic torsion reduction, and finite-field matrix conjugacy/centralizer theory. The displayed double-coset proof is elementary and is **not claimed novel**. Shared research-KB MCP tools and checked-out catalog are unavailable in this runtime; fallback review is necessarily incomplete.

No paid CPU/GPU instance, EKS job, automatic protected merge, whole-project cargo test, complete cold-cost measurement, or ECDLP speedup is asserted. [pcc:unverified]
