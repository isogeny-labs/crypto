# CT57 — A universal rational-isogeny gate for engineered elliptic lifts

**Status:** [pcc:unverified]. Existing mathematical theorem, scoped deduction, independent finite-group control. No uniform encoded-input transfer, end-to-end cost claim, or new cryptanalytic attack.

## The conditional obstruction (no full Galois-image assumption)

Let \(\ell\) be prime and let \(E/\mathbb Q\) be **any** elliptic curve, possibly engineered in response to an input DLP. If the \(\ell\)-torsion representation admits a Galois-stable one-dimensional \(\mathbb F_\ell\)-subspace \(C\subset E[\ell]\), the corresponding finite subgroup scheme defines a \(\mathbb Q\)-rational cyclic isogeny \(E\to E/C\) of degree \(\ell\). Conversely, the kernel of such an isogeny is Galois-stable. Thus Mazur's rational-isogeny theorem implies

\[
\ell\in\{2,3,5,7,11,13,17,19,37,43,67,163\}.
\]

**For prime \(\ell>163\), no engineered elliptic curve over \(\mathbb Q\) has a globally Galois-stable cyclic \(\ell\)-subgroup.** This holds even when \(E/\mathbb Q\) is selected based on the source curve. It does **not** assert that the mod-\(\ell\) image is all of \(\mathrm{GL}_2\), and it does not give a general lower bound on the field degree or running time of an arbitrarily chosen torsion point.

A \(\mathbb Q\)-rational *generator* is stronger than a Galois-stable *line*. The line may be stable while its nonzero generators are permuted by a Galois character. A rational line does not by itself supply a rational point.

### Why a source point rational modulo p does not evade the theorem

For good reduction at a prime \(p\ne\ell\), existence of nonzero \(P\in E(\mathbb F_p)[\ell]\) says only that \(\mathrm{Frob}_p\) has an eigenvector with eigenvalue one on \(E[\ell]\). This is a property of **one** Frobenius element, not of the global Galois action. The elementary negative control below uses \(\mathrm{diag}(1,2)\) and a coordinate-swap matrix: the former fixes the line \(\langle e_1\rangle\) and the pair has no common invariant line. This is an abstract group-theoretic illustration, **not** a claim that these two matrices arise as Frobenius elements for a particular global curve.

### Application to split Jacobians

Suppose a higher-dimensional abelian variety \(A/\mathbb Q\) admits a \(\mathbb Q\)-defined isogeny \(E\times B\to A\) of degree **prime to \(\ell\)**. If the desired **globally stable** cyclic \(\ell\)-subgroup of \(A[\ell]\) is supported on the \(\mathbb Q\)-defined elliptic factor, its preimage yields a Galois-stable line in \(E[\ell]\); Mazur blocks this specific construction for \(\ell>163\).

This conclusion does not apply when the subgroup is not globally stable, when the elliptic factor is not \(\mathbb Q\)-defined, when the isogeny degree is divisible by \(\ell\), or when the subgroup genuinely occupies a higher-dimensional simple factor. In particular CT55's full-division-field construction is **not contradicted**; its needed torsion lifts are generally defined over a nontrivial number field and their oriented lines are not required to be globally Galois-stable.

## Independent finite arithmetic control

For a hypothetical *full* \(\mathrm{GL}_2(\mathbb F_\ell)\) image, elementary stabilizer counting gives

\[
|\mathrm{GL}_2(\mathbb F_\ell)|=(\ell^2-1)(\ell^2-\ell),
\quad |\operatorname{Stab}(e_1)|=\ell(\ell-1),
\quad |\operatorname{Stab}(\langle e_1\rangle)|=\ell(\ell-1)^2.
\]

Therefore the oriented-point orbit has \(\ell^2-1\) elements whereas the projective-line orbit has \(\ell+1\). These are **full-image** controls, not lower bounds for arbitrary engineered images.

| ell | GL2 order | point stabilizer | line stabilizer | point orbit | line orbit |
|---:|---:|---:|---:|---:|---:|
| 3 | 48 | 6 | 12 | 8 | 4 |
| 5 | 480 | 20 | 80 | 24 | 6 |
| 7 | 2016 | 42 | 252 | 48 | 8 |
| 11 | 13200 | 110 | 1100 | 120 | 12 |

The Python implementation exhaustively enumerates invertible 2x2 matrices and their two stabilizers, while the independently written JavaScript implementation generates vector and projective-line orbits using a separate matrix-generator BFS. Both reject the incorrect inference that a single locally fixed Frobenius line is globally invariant.

### Reproduce from repository root

```bash
python3 -B research/cross_characteristic_global_torsion_20261112/ct57/ct57_borel_audit.py
python3 -O -B research/cross_characteristic_global_torsion_20261112/ct57/ct57_borel_audit.py
node research/cross_characteristic_global_torsion_20261112/ct57/ct57_independent.js
node --no-opt research/cross_characteristic_global_torsion_20261112/ct57/ct57_independent.js
```

All four bounded local executions passed. Python normal/optimized outputs match byte-for-byte; JavaScript normal/no-opt outputs match byte-for-byte; the cross-language rows match. Local execution is **not** a signed independent PCC verification and is no substitute for repository exact-head CI.

## Next highest-value construction/proof gate

Retain \(\ell>163\), genus \(g=2,3,4\), and allow the global number field \(K\) to vary with the input. Try to construct **encoded-input** lifts of two arbitrary source points \(P,Q\) without their logarithm, with precisely recorded \([K:\mathbb Q]\), coefficient heights, compatible prime ideals over the two characteristics, cost to select correct roots, and cost to reduce to the target. The globally stable elliptic-line shortcut is eliminated by CT57; the nonstable-line or genuinely higher-dimensional routes remain open.

## Sources and custody

- **Primary theorem restatement and proof context:** Michaud-Jacobs, *Mazur's isogeny theorem* (2022/23), https://arxiv.org/abs/2209.03153 ; Banwait et al., *On elliptic curves with p-isogenies over quadratic fields*, Theorem 1.1, https://doi.org/10.4153/S0008414X22000244 ; original Mazur 1978 theorem as quoted there. Sage official documentation also describes rational isogeny degrees: https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/isogeny_small_degree.html.
- **Prior local research:** CT52–56, `problems/lift-reduce-second-prime-transfer.md`, root PR #3877, cross-characteristic PRs #4012 and #4015, AGENTS.md, and 2026-11-13 session log. No native Cantor files edited.
- **Shared research-KB receipt:** KB MCP unavailable in this runtime and repository-wrapper CLI not executable without an authenticated checkout; GitHub searches across crypto, cryptanalysis, and crypto-autoresearcher used as fallback. No claim that earlier work does not exist.
- **Validation:** Python source SHA-256 `0d58551a5ab7894522b6a9f6fac975b0339746f05aa02951feed5d59df6fcb51`, independent JS source SHA-256 `4ee0b02599834a9fe2c18967a64381143e8eb39e4abfdaa63ee0ca5dff66dba6`. No EKS job or spend authorized for this bounded exact arithmetic test. Conductor and scientific PCC signing unavailable in this runtime.

[pcc:unverified]
