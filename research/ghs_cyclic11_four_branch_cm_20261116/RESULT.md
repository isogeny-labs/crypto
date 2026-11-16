# Four-branch cyclic-11 carriers: a necessary ordinary-source criterion

16 November 2026. This is a finite specialization of published signature and p-rank bounds, with an additional conditional CM argument. Its publication priority is unresolved. Claim standing: unverified.

## Exact population and result

Let (p=3) or (5), let (C/\mathbf F_p) be a smooth cyclic degree-11 cover of (\mathbf P^1) with four distinct geometric branch points, and let its nonzero inertia exponents be ((a,b,c,d)\in(\mathbf Z/11)^4), with (a+b+c+d=0). Riemann–Hurwitz gives (g(C)=10). The normal form with rational branch points is

\[
 C_{a,b,c,\lambda}: y^{11}=x^a(x-1)^b(x-\lambda)^c,
 \quad d=-a-b-c\pmod {11},\quad \lambda\ne0,1.
\]

Put (s_i=-1+\sum_{e\in\{a,b,c,d\}}\langle-ie/11\rangle). The order-five character orbits are (R=\{1,3,4,5,9\}) and (N=\{2,6,7,8,10\}), so the published Bouw bound is

\[
 f_p(JC)\le B=5\min_{i\in R}s_i+5\min_{i\in N}s_i.
\]

The native Rust enumeration and kernel-checked Lean finite certificate give the same result for all **910 ordered** admissible triples ((a,b,c)): 280 rows have (B=0), 360 have (B=5), and 270 have (B=10). Moreover (B=10) **if and only if** the inertia multiset is two opposite pairs (\{u,-u,v,-v\}). The 270 ordered rows represent 15 sorted exponent patterns. These counts classify inertia only; they do not count distinct curves or isomorphism classes. [pcc:unverified]

![Counts of ordered inertia patterns by p-rank upper bound](bound-histogram.svg)

## Conditional exclusion for a simple ordinary restriction

Assume an ordinary elliptic curve (E/\mathbf F_{p^5}) has a Weil restriction (A=\operatorname{Res}_{\mathbf F_{p^5}/\mathbf F_p}E) that is **simple over (\mathbf F_p)**, and assume a nonzero (\mathbf F_p)-homomorphism (A\to JC). Simplicity makes its image an ordinary dimension-five isogeny factor. If (B\le5), that image uses all of the Jacobian's possible ordinary p-rank. After base change to (\mathbf F_{p^5}), its isogeny class is (E^5), and it is the unique ordinary isogeny factor of (JC). [pcc:unverified]

The deck automorphism is defined over (\mathbf F_{p^5}), since (p^5\equiv1\pmod {11}). It preserves the ordinary factor. Because (C/\langle\sigma\rangle\cong\mathbf P^1), the deck action on (JC) obeys (1+\sigma+\cdots+\sigma^{10}=0). Its faithful action on the ordinary factor embeds (\mathbf Q(\zeta_{11})) into (\operatorname{End}^0(E^5)=M_5(K)), where (K=\operatorname{End}^0(E)) is an imaginary quadratic field. The degree bound over (K) forces (K) to be the unique quadratic subfield (\mathbf Q(\sqrt{-11})). Consequently the Frobenius trace (t) of (E) satisfies (t^2+11m^2=4p^5) for an integer (m\ge1). This is a necessary condition, not a construction of a transfer map. [pcc:unverified]

The exhaustive Hasse-range arithmetic gives only (t=\pm31,m=1) for (p=3), and (t=\pm57,m=29) for (p=5). All four traces are fifth-power traces of elliptic isogeny classes already defined over (\mathbf F_p): base traces (\pm1) at (p=3) and (\mp3) at (p=5), using (t_5=s^5-5ps^3+5p^2s). Thus, **under the simple-restriction and nonzero-map hypotheses**, a non-descended ordinary isogeny class can occur only in the (B=10) opposite-pair stratum. This does not assert that such a map exists in that stratum. [pcc:unverified]

## Carrier lead and prior work

An opposite-pair rational normal form can be written

\[
 y^{11}=x^a\left(\frac{x-1}{x-\lambda}\right)^b,
 \quad (a,b,-b,-a),\quad \lambda\in\mathbf F_p\setminus\{0,1\}.
\]

It admits a geometric involution candidate (x\mapsto\lambda/x,\ y\mapsto\delta/y), with (\delta^{11}=\lambda^{a-b}); direct substitution gives (f(\lambda/x)=\lambda^{a-b}/f(x)). Because (\gcd(11,p-1)=1), such a (\delta\) lies in (\mathbf F_p). A concrete candidate at (p=3,\lambda=2,a=b=1) is (C:y^{11}=x(x-1)/(x+1)), with invariant coordinates (u=x+2/x,\ v=y+1/y). The native coefficient check gives the proposed quotient equation (uD_{11}(v,1)=u^2+u+1), where (D_0=2,D_1=v,D_n=vD_{n-1}-D_{n-2}). Its [frozen check and output](QUOTIENT_PROTOCOL.md) are retained as a candidate identity; an `identity.certificate/v1` record and independent quotient replay remain to be done. A smooth quotient model, its Jacobian decomposition, and an elliptic-source subgroup map remain to be checked before treating this as a GHS carrier. Dihedral Jacobian decompositions are established prior art, so this involution is a lead for an exact instance, not a claimed new attack method. [pcc:unverified]

The literature screen used [Ozman–Pries–Weir, Lemma 2.1 and Theorem 2.3](https://doi.org/10.1017/nmj.2022.12) for the signature and bound; [Iijima–Momose–Chao](https://eprint.iacr.org/2015/805.pdf) for direct-cover context; and [dihedral Jacobian decompositions](https://arxiv.org/abs/1609.01562) for the carrier lead. The precise four-branch CM filter was not located in that targeted search. This is insufficient to establish a new theorem or family in the literature.

## Reproduction and evidence limits

The frozen protocol is [PROTOCOL.md](PROTOCOL.md); compilation-only pre-run correction is [AMENDMENT.md](AMENDMENT.md). The native implementation is [screen.rs](screen.rs), pinned source hash [SOURCE.sha256](SOURCE.sha256), compiler record [TOOLCHAIN.txt](TOOLCHAIN.txt), output [RUN.stdout](RUN.stdout), and empty error stream [RUN.stderr](RUN.stderr). The finite Lean certificate is [FourBranchFinite.lean](FourBranchFinite.lean), its design boundary [PROOF_MAP.md](PROOF_MAP.md), and its focused proof gate [status](lean-gate/research_ghs_cyclic11_four_branch_cm_20261116_FourBranchFinite_lean/status): 17 theorems, zero trusted exceptions under Lean 4.35.0. The geometry, CM embedding, and the reduction from a subgroup map to the stated homomorphism are paper arguments and are **not** formalized by that Lean file. [pcc:unverified]

There is no exact elliptic source curve, verified nonzero subgroup map, complete transfer/decomposition solve, or same-subgroup GHS versus rho measurement for this four-branch lead. The next falsifier is to build and independently verify a genus-five dihedral quotient for an exact (p,\lambda,a,b), count its elliptic factors over (\mathbf F_{p^5}), and test a source subgroup map. Only a surviving exact instance should enter an attack-cost benchmark. [pcc:unverified]
