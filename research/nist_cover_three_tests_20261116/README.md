# P-192 mixed 3-isogeny route: native construction checks

2026-11-16. Follow-up to [PR4348](https://github.com/isogeny-labs/crypto/pull/4348). **Scope: native development tests and static archived-evidence replay; the complete proposed attack has not run.** All new assertions are producer-unverified under the repository PCC policy. [pcc:unverified]

## Findings

The P-192 d7 cover now has executable native pullback and norm, six explicit genus3 order3 generators spanning729 distinct classes, and actual divisor realizations of its eight reported Frobenius-stable maximal isotropic kernels. The archived product factors' primitive cube roots are opposite: an elliptic-basis Frobenius change is required before reusing the abstract kernel coordinates. Isotropy is transported through the two degree2 quotient maps; it was not independently computed with a genus3 Weil-pairing implementation. [pcc:unverified]

![Eight kernel rationality checks](figures/kernels.svg)

The next missing algorithm is an explicit quotient from one of those actual kernels, followed by its dual map and a restricted-base projected relation solver. Stable kernels alone do not supply those algorithms. Neither a DLP speedup nor a general obstruction is established. [pcc:unverified]

## Request-to-evidence status

| Requested component | Actual status and evidence |
|---|---|
| Existing NIST/ANSI torsion findings, including omitted controls | Native exact finite-module replay on all306 archived rows from six prime-field streams; all dimension1/2/3 isotropic counts and full Frobenius orders match. This does not independently recompute curve torsion for every row. |
| P-192 d7 actual torsion | Native GMP replay: irreducible degree8 extension; smooth normalized genus2 model; four independent order3 Prym divisors and81 classes; actual Frobenius; four tangent identities and six Weil pairings; elliptic basis/order/Frobenius/pairing. |
| Source elliptic-to-cover transfer | Implemented and fixture-tested pullback and arbitrary rational-coefficient reduced-divisor norm on this monic genus3 model. Thirty-three scalar/composition cases,169 addition pairs, eight further point norms,64 degree2 and64 degree3 norm-additivity fixtures; source/image order-r screens. |
| Actual mixed3 kernels on the genus3 Jacobian | Six generators span729 distinct classes. Eight subgroups of27 elements are explicitly realized and Frobenius-stable. Pairing transport uses aligned roots and the standard polarized degree2 pullback identities. |
| Boundary inputs | Origin, second branch class, and both ramified fibers at elliptic x=1 over a quadratic extension. The latter tests execute pullback fiber/additivity; the norm backend is restricted to rational-coefficient divisors over the base prime field. |
| Corruption controls | Nine rejected cases: count, Frobenius, form, missing modules, duplicate module, torsion point, cube-root/pairing witness, curve coefficient and non-isotropic kernel. |
| Explicit quotient, nonhyperelliptic target and descent | Not implemented for these eight kernels. |
| Executed dual composition TF=[6] | Not implemented; depends on the quotient/dual. Source-cover norm composition [2] is separately fixture-tested. |
| Candidate projected relation solver | No explicit restricted-base solver has been derived or implemented. |
| Complete cost / rho / scaling panel | Not attempted. Both quotient/solver implementation and cloud admission remain open. No scientific job is running. |

All executed rows above have unsigned producer standing; see the linked raw outputs and PCC records. [pcc:unverified]

## Exact curve and models

The source is the standard NIST P-192 / ANSI prime192v1 model

$$E: y^2=x^3-3x+b,$$

with

```text
p=6277101735386680763835789423207666416083908700390324961279
b=2455155546008943817740293915197451784769108058161191238065
r=6277101735386680763835789423176059013767194773182842284081
Gx=0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012
Gy=0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811
```

P-192 is deliberately outside the small standard performance panel: this is the user's named structural construction input, with its actual192-bit prime, rather than a timing measurement or substituted small curve. The preceding [models export](../nist_cover_separator_20261115/data/models.json) includes all21 curves and their published/eligible-omitted labels. ANSI binary curves are outside this odd-characteristic construction.

For d=7, write

$$h(t)=(t^2-1)\big((t^2+7)^3-3(t^2+7)(t^2-1)^2+b(t^2-1)^3\big),\qquad H:v^2=h(t).$$

The two degree2 maps are

$$\pi(t,v)=\left(\frac{t^2+7}{t^2-1},\frac{v}{(t^2-1)^2}\right),\qquad
\rho(t,v)=(u,w)=(t^2,tv).$$

The second target is the genus2 curve

$$D:w^2=u(u-1)\big((u+7)^3-3(u+7)(u-1)^2+b(u-1)^3\big).$$

The archived Prym coordinates use its monic normalization $X_D=(b-2)u$, $Y_D=(b-2)^2w$. The first attempted native replay missed this normalization and rejected the Mumford equation. Its failed output is retained; changing the adapter, rather than changing the frozen coordinates, produced the accepted replay. [pcc:unverified]

Since $h(1)=0$ and $h'(1)=1024$, set

$$X=\frac1{1024(t-1)},\qquad Y=X^4v.$$

Then $Y^2=X^8h(1+1/(1024X))$ is monic of degree7. The branch t=1 becomes the unique infinity; t=-1 becomes $(X,Y)=(-1/2048,0)$. This supplies the one-infinity divisor model used by the native implementation.

With c=1024, the fiber above a finite elliptic point $(x,y)$ has

$$U(X)=X^2-\frac{2(x-1)}{8c}X-\frac{x-1}{8c^2},\qquad
V(X)=\frac{y(1+2cX)^2}{c^4}\pmod U.$$

Its pullback class is the fiber class minus the second branch class. The formula includes x=1 as the repeated fiber U=X². For P-192 these fibers are not rational point pairs over the base field; both were checked over its quadratic extension. [pcc:unverified]

The norm factors a rational-coefficient Mumford U into irreducibles of degrees at most3, evaluates the map over each residue field, takes its Frobenius trace on E, and respects repeated factors. It does not find an elliptic scalar. Its tested domain is this declared prime-field monic model; it is not a general extension-field norm API.

## The eight kernels

Basis order is two aligned elliptic pullbacks followed by four Prym pullbacks. The exact six Mumford generators are in [actual-cover-torsion.json](evidence/final-kernel-check/actual-cover-torsion.json); coefficient arrays are in ascending order and use the extension modulus in [the input witness](inputs/p192-d7-prym.json). The [eight RREF coordinate bases](evidence/final-kernel-check/kernels.jsonl) reconstruct all kernel generators with the native code.

Every kernel has full point-field degree8, trivial intersection with the elliptic factor and intersection of size3 with the Prym factor. Kernels1,2,4,8 have only the identity rational; kernels3,5,6,7 have three rational elements. This separates subgroup descent from point rationality. [pcc:unverified]

The two nonzero rational Prym 3-torsion classes were lifted into J(H), checked fixed by Frobenius and checked to have zero elliptic norm. Their translations therefore do not create distinct projected elliptic factor-base elements. This is a checked constraint on that mechanism, not a rejection of all torsion-assisted coordinate methods. [pcc:unverified]

![Construction and remaining quotient](figures/route.svg)

## Reproduce and audit

Requirements: C++17 compiler, GMP C++/C libraries, OpenSSL libcrypto and POSIX shell. No Python arithmetic, curve search, solver or replay is used. The Python figure script only lays out recorded observations.

```sh
research/nist_cover_three_tests_20261116/check.sh /tmp/nist-three-new-output
```

The output directory must not already exist. The harness compiles separate executables, records hashes before calls, sets executable files read-only and retains distinct post-call source hashes. Generated arithmetic headers are exact prefixes of the unchanged point and cover sources, with their CLI main functions removed. This reuses one implementation; it does not constitute an independent verifier.

Canonical development evidence is [final-kernel-check](evidence/final-kernel-check/): [module summary](evidence/final-kernel-check/module-summary.txt), [actual source torsion](evidence/final-kernel-check/point-replay.txt), [cover maps](evidence/final-kernel-check/cover-replay.txt), [kernel realization](evidence/final-kernel-check/actual-cover-torsion-summary.txt), [controls](evidence/final-kernel-check/controls.txt), hashes and raw module rows. Earlier pre-call manifests and failed normalization/global-initialization attempts remain under evidence/. No elapsed time or host-performance ratio is reported.

Evaluated polynomial certificates: `IDC1h2fca4883893cfdf8` (cleared cover equation), `IDC1h27026700af8d9bbf` (normalized quadratic fiber). Each records32 deterministic evaluations modulo2^61-1 and rejects an added-one corruption. These are producer-issued evaluations, with no Lean proof or independent signature. [pcc:unverified]

PCC statements: `PCC-20261116-b0b9e9` (finite modules), `PCC-20261116-4a86ed` (source points/pairings), `PCC-20261116-e627e5` (cover fixtures and original eight controls); `PCC-20261116-e4ac3b` records actual kernel realization and nine controls. All have unsigned producer standing. [pcc:unverified]

## Full test still required

For a polarized3-isogeny phi with dual phi-dagger, the proposed maps are $F=\phi\pi^*$ and $T=\pi_*\phi^\dagger$. Standard identities imply TF=[6]; this remains conditional on the unconstructed quotient. The proposed relation problem is

$$T\left(\sum n_iD_i\right)=aP+bQ,$$

allowing a free residual in ker(T). A complete test needs an explicit solver for this equation, independent elliptic verification, duplicate-image merging and rank measurement, then all cold costs against direct elliptic equations and matched rho. Merely applying Diem's ambient genus3 solver does not provide the hypothesized elliptic-subgroup advantage.

The [named obstruction](../../problems/nist-mixed-three-quotient-and-relation-solver.md), [blocked backlog card](../../backlog/XP-20261116-nist-mixed-three-quotient/README.md) and [protocol](PROTOCOL.md) preserve the original requirements. The current main journal reports monthly lab spend USD1982.224 againstUSD1300; this executor has no live AWS readback. No paid experiment/build was submitted, and no alternative backend was used to bypass that admission limit. Restored budget alone would not supply the missing quotient and solver.

## Sources checked and novelty boundary

KB unavailable: no loaded KB tools; repository wrapper exits127. Used the prior module/source exports, local transfer skill/API, existing [odd-isogeny problem](../../problems/odd-isogeny-quartic-transfer-execution.md), [projected-relation problem](../../problems/prime-cover-projected-relation-cost-missing.md), and these primary sources:

- Song Tian, [arXiv:2007.03172](https://arxiv.org/html/2007.03172), sections1,4,6: known odd-degree genus3 quotient/map construction, rational descent considerations and the F257 example.
- Enea Milio, [arXiv:1709.06063](https://arxiv.org/html/1709.06063), introduction and section7: quotient/theta methods and implementation scope.
- Joux--Vitse, [Cover and Decomposition](https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/CoverDecomp.pdf), sections1--3: prior cover/decomposition mechanism over composite-degree extension fields; no automatic prime-field implementation follows.
- Faugere et al., [Symmetrized Summation Polynomials](https://doi.org/10.1007/978-3-642-55220-5_3): prior torsion-assisted coordinate compression. Publisher/HAL full text was inaccessible in this executor; author bibliography and indexed author text were inspected, so no detailed priority or formula claim rests on that inaccessible text.
- OpenSSL `ecparam -name prime192v1 -param_enc explicit -text -noout`: source-generator/public-order cross-check.

This work implements/replays prerequisites for the proposed combination. It makes no priority claim for isogenies, norm projection or torsion-assisted index calculus. Earlier ranking graphs and the IC leaderboard stay unchanged because no new ranking analysis or admitted performance result was produced.
