# Independent review: fixed-source-curve Dickson fibre density

**Decision:** no substantive mathematical error or required correction found in the reviewed source.

**Reviewed source:** repo-audit/research/prime31_summation_compression_20261122/iteration7/FIXED_CURVE_FIBRE_DENSITY.md  
**Source SHA256:** c1725df2f00f61a972edbb5cfee803fe97bd2c27d8f761fbb08b73385c70b7a1  
**Standing:** independent written review, [pcc:unverified]. No computation of field values, crypto experiment, native test, or remote mutation occurred.

## Checks of the proof

1. **Trace indicator and endpoints.** For g=X²−4, the norm-one trace image is exactly the nonsquare-discriminant set together with ±2. Its indicator is \((1-\chi(g)+1_{g=0})/2\), and its cardinality is (p+3)/2. Counting \((x-y)(x+y)=4\) independently gives \(\sum_x\chi(x^2-4)=-1\). Expanding the two indicators gives the displayed exact formula
   \[
   A_T=(p+3+S_f-S_{fg}+\chi(f(2))+\chi(f(-2)))/4.
   \]
   Both endpoint corrections are necessary and present.

2. **Character-sum hypotheses.** Nonsingularity makes f squarefree. Odd rational group order excludes every rational zero of f, including ±2. Thus fg is squarefree of degree five. Its smooth projective model has genus two and one rational point at infinity; the elliptic model has genus one and one such point. The imported Hasse–Weil bounds therefore give exactly \(|S_f|\le2\sqrt p\), \(|S_{fg}|\le4\sqrt p\), without an added infinity term. The total-trace lift error is at most \(3\sqrt p/2+1/2\).

3. **Exceptional fibres and partition.** Since q is even, u^q=1 contains the two inversion-fixed elements ±1 and gives q/2+1 trace coordinates; u^q=−1 gives q/2. Their trace sets are disjoint. Removing their q+1 coordinates leaves
   \[
   K=(p+1)/2-q=Mq,\qquad M=(p+1)/(2q)-1.
   \]
   For every regular value, the q traces from one q-coset are distinct. Otherwise inversion would identify two elements of that coset and force its q-th power to be ±1. The monic degree-q Dickson polynomial consequently has precisely these q simple roots. The regular fibres partition the remaining trace set.

4. **Error and fraction constants.** The excluded lift count lies in [0,q+1]; centering this interval gives the additional error (q+1)/2. Hence the numerator \(3\sqrt p/2+q/2+1\) is correct. For the stated cap, \(K\ge p/3\), so the proof's error bound
   \[
   147/(32\sqrt p)+3/p\le8/\sqrt p\le1/8
   \]
   is valid at p≥4096. If the mean lift fraction is at least 3/8, a fraction ρ of fibres above threshold 1/4 satisfies \(3/8\le1/4+3\rho/4\), giving ρ≥1/6. No distribution heuristic is used.

5. **Sampling and charged failures.** Each regular fibre has exactly 2q torus seeds, so uniform torus sampling conditioned on regularity is uniform over fibres. The excluded probability is \(2q/(p+1)\le1/512\); success is at least 511/3072, giving expected seed count at most 3072/511<7. The P¹(Fp) parametrization is a bijection, including infinity mapped to 1; a uniform sampler must include that extra outcome with its correct probability. The q-subgroup projection is uniform, and exactly half its elements have order q. Expected two generator trials, failed fibre trials, inversions, enumeration, and residue/root tests are all explicitly charged. The stated O(q log p) field-operation upper bound plus inversion costs and O(q) storage is consistent with the described algorithm.

6. **Capped family exponent.** From \(p\le C(4Q)^L\) and L≥2,
   \[
   q\ge\tfrac12\min(Q,\sqrt p/16)
   \ge\min(1/(8C^{1/L}),1/32)\,p^{1/L}.
   \]
   Because both q and Q are powers of two and q≤Q, the divisibility 4q|(p+1) is preserved. The cap does not erase the polynomial growth guaranteed by the existing Linnik family, but it also does not provide arbitrary factor-base exponents.

## Scope retained

The result chooses a fibre on the **same specified curve**. It removes the earlier need to choose new curve coefficients for rational-lift density within the capped fibre range.

On an already supplied whole prime-order group, the selected rational points belong to that group. On an odd composite group, density in a prescribed proper subgroup is not established; cofactor clearing need not preserve the fibre. Neither a growing prime-order source family nor a faster relation-solving algorithm follows.

For exact-m affine-chain completeness, the earlier prime-order condition m≤n remains sufficient. On a composite group, retain the permitted-leaf order >m−1 restriction, its charged small-order filtering/membership guards, or explicit identity-intermediate branches. The density theorem itself supplies no replacement for this completeness condition.

The chosen c need not equal a previously frozen value. No claim for every fibre, c=0, a fixed balanced tree, relation independence, or target recovery is justified by this averaging argument.

## Source check and prior-art boundary

I independently read the bound in Weil's Theorem 13, Corollary 3, PDF p.61, in the explicitly identified 2022 transcription of the 1948 work:
https://denisevellachemla.eu/Weil-courbes-varietes.pdf .

I also checked Galbraith's author-maintained Chapter 10, Theorems 10.7.1 and 10.7.5, Exercise 10.7.9, and the squarefree degree-five/ramified-model conventions:
https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf .

These support the genus-one/genus-two point-count bounds and the infinity convention. The review imports the classical theorem; it does not independently reprove Weil's theorem or certify the transcription against the historical printing. The attempted PMC PDF of Weil's separate 1948 exponential-sums note was inaccessible and is not relied on.

The argument is a concrete application of standard character-sum bounds and exact torus counting. No literature-wide novelty conclusion follows from this bounded review. KB tools and wrapper remained unavailable; the review used the parent-provided lane sources and the above texts.

The reviewed source and all pinned iteration-six artifacts were left unchanged.
