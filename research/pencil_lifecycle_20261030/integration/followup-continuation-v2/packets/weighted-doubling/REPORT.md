# Weighted degree-one doubling: tangent validity

Seven Lean declarations compile, with separate controls and axiom readback passing. The actual weighted degree-one doubling field formula preserves normalized Mumford divisibility for a point on f=X^5+a3X^3+a2X^2+a1X+a0, with nonzero z,w,y and 2. No nonzero-derivative assumption is needed.

Write xx=x/z² and yy=y/(z⁵w). The output has z'=z,w'=2y and normalizes to q=-2xx,r=xx²,s=f'(xx)/(2yy),t=yy-s*xx. Thus its monic u is (X-xx)². The explicit Taylor identity proves f-v² is divisible by u: the quotient is X³+2xxX²+(3xx²+a3)X+4xx³+2a3xx+a2-s², and the two residual terms vanish by the point equation and 2yy*s=f'(xx).

The field output definition follows gate dbl lines478–510, preserving multiplication/subtraction association. The inverse-first prelude is retained: the self-add guard reaches doubling for nonzero y in characteristic not2; y=0 reaches inverse identity. The guard theorem covers a self-add invocation. The payload validity theorem applies whenever its stated input hypotheses hold, but no general differently-weighted equality-to-guard converse is claimed.

Curve nonsingularity is not necessary for this polynomial-divisibility lemma. It remains necessary for a Jacobian geometric/group interpretation. The exact formula assumes monic degree5 and zero X4 coefficient. The normal_curve preprocessing, including its division by5, is not proved here, and characteristic5 normalization must not be inferred from the formula theorem. No UInt64, canonical-residue, counter, compiler, class-sum or whole-walk claim is made.

Controls include nonunit weights z=2,w=3 at normalized(1,1) on f=X5; output slope5/2 and intercept-3/2 are checked and the full theorem instantiated. This first curve is a polynomial-algebra fixture, not a claimed nonsingular Jacobian. A second full theorem control at normalized(0,1) on f=X5+1 has derivative zero. Separate controls check self-add doubling, y=0 inverse precedence and the zero output weight that bypassing that inverse guard would produce.

The initial proof attempt failed for a noncomputable polynomial definition and a denominator cancellation requiring explicit hypotheses; its source/errors remain retained. Corrected tangent proof, weighted extension and final guard extension compile with exit0, as do Controls and Readback. Readback contains only propext, Classical.choice and Quot.sound. Harmless linter warnings remain retained.

Literature/reference: Ozbay Gurler–Hisil, Complete group law for genus2 Jacobians on Jacobian coordinates, section2.2 equation(3), https://link.springer.com/article/10.1007/s13389-024-00351-7, gives these affine tangent coefficients and excludes zero ordinate in this nonidentity case. Sections2–4 and prior retained source audits supply representation context. This packet proves the local formula identity independently, without a novelty or whole group-law claim.

All Lean commands use existing pinned dependencies, one thread, 4096MiB and120-second timeout. No scientific trials or repository edits occurred. Independent review is pending.
