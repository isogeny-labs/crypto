# Abstract mixed-line kernels qualified; full point field has degree658

[pcc:unverified] Freeze222e527102c pushed before once-only invocation; input hashes pass, exit0. All1316 abstract normalized-eigenbasis subspaces pass rank3, Gramzero, Frobenius-invariance and sigma-nonstability (rank4 after adjoining sigma(firstrow)). Distinct normalized slopes and disjoint final eigenspaces give distinct subspaces. The existing cyclotomic/Tate/pairing interpretation remains conditional and independently unreviewed.

Exact Frobenius orders modulo659:
|eigenvalue|multiplicity|multiplicative order|
|---|---|---|
|219|2|329|
|357|2|94|
|388|1|658|
|424|1|329|

The659th roots of unity overF421 also require degree658. Each mixed-line kernel has eigenspaces219,357 and one of388/424, so its full point field is F_(421^658). This is a pointwise rationality statement. The kernel scheme, quotient and isogeny remain defined overF421 under the rational-kernel premises.

Two parameter families, each658 kernels:
- gamma388: generator orders329,94,658;434948 nonzero Frobenius point orbits.
- gamma424: generator orders329,94,329;435607 nonzero Frobenius point orbits.

Both have286191178 nonzero points and isogeny degree659³=286191179. Fixed-point/divisor inversion counts are exact, sum to659³ includingzero, and every exact-degree count is divisible by its degree. The raw record gives representative basis matrices and all orbit-degree strata. These are abstract module matrices, not actual Mumford points or a geometric kernel ideal. The normalized symplectic eigenbasis itself has not been constructed.

## Construction boundary

A generators-based method need not place every generator in the compositum immediately: individual eigenvectors may use their degree329/94/etc fields. Pairing, combined operations and all-point rationality can require the degree658 compositum. Do not replace all these field operations by base-field operations or infer that the rational scheme requires degree658 to define.

Lubicz-Robert, Computing separable isogenies in quasi-optimal time, Theorem1.1, https://www.normalesup.org/~robert/pro/publications/articles/rational.pdf , takes rational kernel equations in theta coordinates and avoids imposing pointwise rationality over the base. Our abstract matrices do not supply those equations. Its stated complexity is an algorithmic upper bound, not a lower bound on all constructions.

Milio, Computing isogenies between Jacobians of curves of genus2 and3, introduction, https://arxiv.org/pdf/1709.06063 , takes the kernel as geometric divisor data or extension-field orbit representatives and explicitly separates obtaining that input. The present matrices likewise do not supply its geometric input. These primary sources were read; KB source:d1f066b14c5b3335c99349166064ca33 is only an unreviewed pointer.

Decision: retain659 as a mathematically admitted exceptional family but do not launch full kernel-point enumeration on the basis of the1324 count. First obtain a bounded constructive bridge from the abstract mixed eigenspaces to geometric kernel data or rational equations, with theta/curve conversion, torsion generation, pairing, quotient construction, destination recovery and target transport charged. No runtime budget or speedup follows from the orbit count. No non-hyperellipticity or faster destination is established. Original genus2+10x/genus2/4 and independentvalidation obligations remain open.
