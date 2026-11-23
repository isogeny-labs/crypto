# Cross-characteristic global-torsion positive control (2026-11-12)

**Scope:** classical toy control, not a new attack, not a scalable characteristic-changing map. All statements below are **[pcc:unverified]** pending an independently signed verifier. Related root PR: [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877). Open obstruction: [lift/reduce second prime](../../problems/lift-reduce-second-prime-transfer.md).

## Explicit construction

Use the rational curve **Cremona 11a1 / LMFDB 11.a2**

\[
E/\mathbb Q:\quad y^2+y=x^3-x^2-10x-20,\quad \Delta=-11^5,
\]
with rational 5-torsion generator \(P=(5,5)\). The five global points, in scalar order, are
\[
O,\quad(5,5),\quad(16,-61),\quad(16,60),\quad(5,-6).
\]
The finite reductions at primes \(3\) and \(2\) are nonsingular (their characteristics do not divide \(\Delta\)), both have **five rational points**, and both reductions of \(P\) have order five. Therefore reduction of the global subgroup \(T=\langle P\rangle\) yields isomorphisms
\[
T\xrightarrow{\sim}E(\mathbb F_3),\quad T\xrightarrow{\sim}E(\mathbb F_2).
\]
Consequently the following explicitly encoded abstract-group map is well-defined:
\[
\mathcal T=\operatorname{red}_2\circ(\operatorname{red}_3|_T)^{-1}:
E(\mathbb F_3)\rightarrow E(\mathbb F_2).
\]
It preserves *every* discrete logarithm \(Q=[n]P_3\mapsto[n]P_2\), for \(n\in\{0,\ldots,4\}\). This follows from reducing rational torsion in both characteristics, not from a morphism between different-characteristic elliptic curves. No unital field homomorphism \(\mathbb F_3\to\mathbb F_2\) exists.

### Explicit complete lookup

| Source \(\mathbb F_3\) | Target \(\mathbb F_2\) | Rational lift |
|---|---|---|
| \(O\) | \(O\) | \(O\) |
| (2,2) | (1,1) | (5,5) |
| (1,2) | (0,1) | (16,-61) |
| (1,0) | (0,0) | (16,60) |
| (2,0) | (1,0) | (5,-6) |

The Python verifier also checks reductions \(p\in\{2,3,7,23\}\to2\); for \(p=7,23\) only the specified 5-torsion subgroup is transported, not the entire source group. The independent JS finite-field auditor reconstructs the group law separately.

## How to reproduce

No packages, SageMath, paid infrastructure, or external access are required.

```bash
python3 -B research/cross_characteristic_global_torsion_20261112/toy_transfer.py
python3 -O -B research/cross_characteristic_global_torsion_20261112/toy_transfer.py
node research/cross_characteristic_global_torsion_20261112/independent.js
node --no-opt research/cross_characteristic_global_torsion_20261112/independent.js
```

The checks cover, for each source characteristic, all five subgroup points, all 25 group-law pairs, all five planted public DLPs solved in the target, and a deliberately swapped output that must violate the group law. The Python verifier calculates the five global points using exact rational arithmetic; the JS implementation audits the finite-field side independently. Using unconditional checks instead of Python `assert` keeps `-O` testing meaningful. **Local preliminary execution on both implementations passed, but a cryptographic claim is not signed or PCC verified.**

## Cost and obstruction

The characteristic-two destination is extremal for genus one: Hasse gives `#E(F_2) <= floor((sqrt(2)+1)^2) = 5`; this example has exactly five elements. Thus **no elliptic curve over F_2 can contain a prime-order subgroup larger than five**. A putative genus-one target over `F_(2^m)` must instead have `N <= (2^(m/2)+1)^2`, so `m >= 2 log_2(sqrt(N)-1)`. This is a standard necessary size bound, not a complexity lower bound on higher-genus lifting. [pcc:unverified]

To **apply** \(\mathcal T\) to an encoded source point without knowing its logarithm, this construction first identifies the global torsion preimage by a five-entry lookup table. For order \(N\), analogous enumeration is **O(N) time and space**. It already exceeds the \(\tilde O(\sqrt N)\) Pollard-rho baseline on large generic groups. This toy example therefore proves that cross-characteristic *abstract subgroup transfers can exist*; it does **not** provide a sub-rho reduction, uniform small-degree lift of arbitrary cryptographic points, or any algebraic morphism \(\operatorname{Spec}\mathbb F_3\to\operatorname{Spec}\mathbb F_2\). It also is not an algorithm for arbitrary \(E/\mathbb F_p\), merely for this globally preloaded subgroup.

The actual open question is whether one can derive, from **arbitrary** encoded input \(P,Q\in E(\mathbb F_p)[N]\) with \(Q=[n]P\), simultaneous globally algebraic \(N\)-torsion lifts with size/cost \(N^{o(1)}\), **without first computing \(n\)**. For a global abelian variety \(A/K\) of dimension \(g\), good reduction at a prime above \(q\ne N\) and residue field of size \(q^f\) injects global \(N\)-torsion, forcing \(N\mid\#A(\mathbb F_{q^f})\) and
\[
N \le (q^{f/2}+1)^{2g}.
\]
In particular the target field cannot be arbitrarily small at fixed \(g\), but this necessary bound does not prove a prohibitive global lifting degree for \(g\ge2\). The number-field degree, bit complexity, and total target solving cost remain unsolved.

## Literature and prior-art receipt

- [LMFDB 11.a2](https://www.lmfdb.org/EllipticCurve/Q/11/a/2) independently records the exact model, rational order-five generator, discriminant and global rational torsion. This is a **textbook positive control**, not a novel mathematical theorem.
- [Jacobson, Koblitz, Silverman, Stein, Teske, *Analysis of the Xedni Calculus Attack* (2000)](https://doi.org/10.1023/A:1008312401197) treats relation-preserving characteristic-zero lifts rather than a completed cross-prime algorithm.
- [Bourdon–Clark, *Torsion Points and Galois Representations on CM Elliptic Curves*](https://arxiv.org/abs/1612.03229) addresses field-degree restrictions in the CM case; **our 11a1 is non-CM** and this paper is not invoked as a theorem about all genus-\(g\) abelian varieties.
- Repo: `problems/lift-reduce-second-prime-transfer.md`, `problems/prime-field-chained-transfer-no-advantage.md`, `research/chained_prime_transfer_20261111/`, and `cryptanalysis:research/xedni_cross_field`. CT47–50 investigate torsion/gluing/Richelot **within** characteristic 101 and are not duplicated here.
- Shared research KB: no `kb_status/search_knowledge` MCP actions were loaded in this runtime and no repository-local catalog checkout was available. GitHub search in all three repos for `cross-characteristic`, `global torsion`, `11a1`, `xedni`, plus the cited repo records and papers was the fallback. It does not establish absence from the encrypted/autoresearcher atlas.

## Next proof obligation

For genus \(g\ge2\), construct a family of abelian varieties over number fields with a **point-input-uniform** algorithm for simultaneous \(N\)-torsion lifting, then bound its extension degree, coordinate heights, and arithmetic in both characteristics. Treat \(N^{o(1)}\) lifting cost as the falsifiable gate before trying a target index-calculus solver; test on \(N\le2^{20}\) using exact Galois orbit degree. For any claimed advantage require independent full-cost comparisons against matched optimized rho and the current index-calculus baseline. This file changes no benchmark, solver, or claimed speedup.
