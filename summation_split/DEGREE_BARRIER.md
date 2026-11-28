# Semaev signed-fiber elimination-degree barrier

**2026-11-23 · PR #5068 · producer-owned exact arithmetic [pcc:unverified].** This follow-up is scoped **only** to the summation polynomial \(S_{m+1}\). It changes neither the factor-base map nor the ECDLP relation pipeline. It follows **merged** #5023, #5044 and #5058.

## Exact invariant and what it rules out

Let \(E/\mathbb F_p:y^2=x^3+ax+b\), with \(p>3\) odd and nonsingular. Specialize \(x_1,\ldots,x_m\) to abscissae of non-2-torsion points \(P_i\in E(\mathbb F_p)\). The signed fiber is

\[
R=\left\{x\left(\sum_{i=1}^m \epsilon_iP_i\right):
 \epsilon_i\in\{+1,-1\},\ \sum\epsilon_iP_i\ne\mathcal O\right\}.
\]

The simultaneous negation of all signs preserves the abscissa, so at most \(2^{m-1}\) distinct values occur. When there are no other collisions and no identity, there are **exactly** \(2^{m-1}\) values. The specialized Semaev polynomial in its final variable \(X\) has degree at most \(2^{m-1}\); independently compute its fixed-degree Sylvester resultant and assert the *entire coefficient-vector identity*

\[
S_{m+1}(x_1,\dots,x_m,X)=c\prod_{r\in R}(X-r),\qquad c\in\mathbb F_p^*.
\]

On such a full fiber, for **any** polynomial system that preserves exactly these projected \(\mathbb F_p\) roots, any **nonzero univariate polynomial** in the elimination ideal \(I\cap\mathbb F_p[X]\) must vanish at all \(2^{m-1}\) values. Consequently it has degree at least \(2^{m-1}\). The bound applies to any *lex elimination output*, independently of its auxiliary generator degrees. A low-degree \(S_3\)-chain cannot erase this algebraic degree when eliminating down to \(X\).

This **does not** imply a lower bound on graded-reverse-lexicographic F4's maximum Macaulay degree, FGLM complexity, runtime, memory, or first-fall degree. Low-degree circuits can still offer a computational advantage through sparsity or an alternative representation. In particular, a bounded first fall would not resolve the remaining solving-degree question.

Geometrically, for generic independent points \(P_i\) over an algebraic closure, different signed sums (up to global negation) have distinct abscissae: a collision forces a nontrivial relation \(\sum_i n_iP_i=\mathcal O\) with \(n_i\in\{0,\pm 2\}\), cutting out a proper closed subset of \(E^m\). This is the familiar generic exponential degree of Semaev's polynomial, **not** a novelty claim. The finite-field root count cannot grow past \(p\), so the table is a finite witnessed regime rather than an \(m\to\infty\) assertion at fixed \(p\).

## Exact deterministic results

\(p=2^{31}-1\). Each cell has **50** independently seeded full-degree (collision-free) \(\mathbb F_p\)-rational input tuples. The verifier computes signed point sums, re-computes the specialized polynomial via the repository's fixed-degree resultant, checks every root, multiplies the linear factors, and compares **all** coefficients mod \(p\). The leaf choices are deliberately chosen to have rational nonzero ordinates; this is an **obstruction/positive-control family**, not a statistically random family of arbitrary abscissae.

| curve \((a,b)\) | \(m\) | random cases passed | \(\deg_X S_{m+1}\) | distinct rational signed roots | degree-four \(S_3\) generator |
|---|---:|---:|---:|---:|---:|
| (1,1) | 3 | 50/50 | 4 | 4 | 4 |
| (1,1) | 4 | 50/50 | 8 | 8 | 4 |
| (1,1) | 5 | 50/50 | 16 | 16 | 4 |
| (1,1) | 6 | 50/50 | 32 | 32 | 4 |
| (1,4) | 3 | 50/50 | 4 | 4 | 4 |
| (1,4) | 4 | 50/50 | 8 | 8 | 4 |
| (1,4) | 5 | 50/50 | 16 | 16 | 4 |
| (1,4) | 6 | 50/50 | 32 | 32 | 4 |

**400/400 exact coefficient-factorization checks** on the seed \(20261123\) with Python/SymPy fallback (three local unit tests passed); unsigned producer result, not an independent/PCC certificate. The complete per-instance CSV can be reproduced with the module and its SHA-256 is \(`b85cbf643200db00d3db7f4881b727266389a49e2df668e5db9d73421d2c8f50`\) in the local Python run. Different Python implementations of deterministic RNGs should still verify the identity, though they may not reproduce the byte digest.

A fixed small-field witness uses the **smallest admissible odd prime** \(p=5\), \(E:y^2=x^3+x+1\), and \((x_1,x_2,x_3)=(0,3,4)\). Direct exact symbolic resultant:

\[
\boxed{S_4(0,3,4,X)=-X^4-X^3-X^2-X
=-X(X+1)(X^2+1)\pmod5,}
\]

with **four distinct** roots \(X=0,2,3,4\). No polynomial of degree less than four can vanish at all these roots. This is the smallest-field full \(S_4\)-degree witness, not a proof that a quartic-circuit F4 formulation cannot be faster. Previously reported finite-affine-chart counterexamples in #5023/#5044 are unaffected.

For an additional deterministic Prime31 example with \((a,b)=(1,1)\), the first \(m\) abscissae in \((0,6,11,12,14,21,22,25,26,31)\) produce \(2^{m-1}\) distinct signed-sum abscissae for \(m=2,\ldots,10\); the fixed-degree resultant was checked directly at \(m=3,4,5,6\).

## Reproduction and interpretation

```bash
PYTHONPATH=. python -m unittest -q summation_split.test_degree_barrier
PYTHONPATH=. python -m summation_split.degree_barrier --samples 50 --out summation_split
sha256sum summation_split/degree_barrier_gate.csv
```

The generated CSV explicitly leaves **first-fall degree** and **\(d_{F4}\)** blank; it has no solver timings. `degree_barrier_vs_m.svg` is labelled **lex elimination lower bound, not F4 solving degree**. Existing `summation_split/bench.csv` and `solving_degree_vs_m.svg` describe the earlier SymPy controls and generator degrees, not new F4 experiments.

## Next test / falsification criterion

Use a common **unspecialized** random linear-slicing protocol on the same \(S_{m+1}\) inputs, then compare the monolithic, projective chain, symmetric, linear, and complete guarded-sector systems with exactly matched field-membership semantics. Run actual Sage/Singular F4, FGb or msolve. Record maximum Macaulay degree, independently specified first-fall definition, wall, incremental peak memory and projected solution equality after at least 50 random instances per cell. The tested degree barrier is consistent with **either** a faster or a slower graded-F4 circuit.

**Prior sources:** Semaev (2004), [ePrint 2004/031](https://eprint.iacr.org/2004/031); Kosters–Yeo (2015), [arXiv:1503.08001](https://arxiv.org/abs/1503.08001), warns against equating first fall with solving degree; Kousidis–Wiemers (2019), [DOI 10.1515/jmc-2017-0022](https://doi.org/10.1515/jmc-2017-0022), analyzes a different Weil-descent regime, not this odd-prime fiber. Existing repository negative controls: `summation_split/FINDINGS.md` and `summation_split/SECTOR_COMPLETE.md`. The research-KB MCP and wrapper were unavailable in this runtime, so this is a primary-repository/literature fallback review.
