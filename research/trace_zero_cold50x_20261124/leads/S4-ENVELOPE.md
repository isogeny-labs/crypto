# Global S4 linear-envelope lead (proposed, separate from frozen dense indexing)

Sources checked: src/cryptanalysis/binary_semaev.rs:115–188, its actual characteristic-two resultant formula and Semaev 2004 recurrence. KB unavailable (wrapper127). This is an explicit relaxation of an existing summation polynomial; no novelty claim.

Let a=x1+x2+x3, b=x1x2+x1x3+x2x3, c=x1x2x3 and t=target.x on E0. Expanding the implemented resultant gives

S4=(b^4+c^2+1)t^4+c(b^2+1)t^3+(b^2+a^2c^2)t^2+c(a^2+c^2)t+a^4+c^4+c^2.

If every factor-base x has polynomial representative degree<=k, then degrees of a,b,c are <=k,2k,3k. Relax cb^2, ac and c(a^2+c^2) to independent polynomial variables of degrees7k,4k,9k. S4=0 implies t^4 lies in the F2 span of these six families:

- z^(4j), j=0..k;
- t^4 z^(4j)+t^2 z^(2j), j=0..2k;
- z^(4j)+(t^4+1)z^(2j)+t^3 z^j, j=0..3k;
- t^3 z^j, j=0..7k;
- t^2 z^(2j), j=0..4k;
- t z^j, j=0..9k.

An inconsistent span would refute every triple at once, before residual enumeration. Consistency proves nothing. Actual points/signs and factor-base admission remain mandatory. Distinct unknown families are independent only in the relaxation, which is sound for rejecting inconsistency.

Predeclared exact falsification: verify the displayed identity against the original quadratic resultant for all8^4 F8 tuples. Then compute exact relaxed ranks and target membership for all six retained m83/m131 target x-values at d=2,4,6,8,10,12 (k=d-1). If the d12 spans have full field rank, this relaxation cannot filter those targets; retain that outcome and do not integrate it. This diagnostic is not a speed measurement. Native Rust arithmetic only, finite local development check, no paid resources.
