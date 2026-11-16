# Prepared C119: deterministic coprime 2+3 lifting on the majority branch

Status: derived prototype, AST/global-name preflight only; not scientifically executed this session. Hensel factor lifting is known prior art; no novelty or speed claim.

For k3, F(Z,0)=Z²(Z+1)³. Write A0=Z² and B0=(Z+1)³. They are coprime. For every n, the next correction solves B0*A_n+A0*B_n=E_n with deg_Z A_n<2 and deg_Z B_n<3. The five-by-five coefficient map is invertible over F2. If E_n=e0+e1Z+e2Z²+e3Z³+e4Z⁴, then

A_n=e0+(e0+e1)Z,
B_n=(e1+e2)+(e1+e3)Z+(e0+e1+e4)Z².

These formulas explicitly give its inverse, proving existence/uniqueness one coefficient at a time; no new factor-choice bits. Truncate through n=3(d−1), enough to contain every coefficient of a degree-three polynomial factor after normalization. Require individual coefficient degree bounds and the FULL product identity F=A*B, not only a congruence to truncated precision. This is necessary and sufficient for a bounded normalized coprime factorization with these initial factors: any such factors agree with the unique lifted truncation, and exact equality proves the returned factors. Degree-two and degree-three factors can retain internal multiplicity; no squarefree assumption is used.

Unnormalize coefficient j of a monic degree-m factor by u^((m-j)(d−1)), reversing bits. The resulting factors must separately satisfy the original prefix-support criterion. This extra condition is essential.

Concrete counterexample (d=2): H(T)=(T²+T+1)(T+X)³. It satisfies both k3 weighted leading bounds and translated k2 bounds. Its normalized factors are A=Z²+uZ+u² and B=(Z+1)³. They are bounded, coprime, and multiply exactly, yet T²+T+1 has no root among {0,1,X,X+1}; H is not fully prefix-supported. Ascending coefficients are [X³,X³+X²,X³+X²+X,X²+X+1,X+1,1]. Translation T→T+X gives T³(T²+T+X²+X+1). Thus neither leading bounds nor exact bounded factorization replace the remaining support tests.

On the current actual cover, k1/k3/k5 contribute16/81,64/81,1/81of the nominal affine assignment count. These are relaxed parameter fractions, not search time fractions. The new mechanism targets k3rather than merely increasing k1guards. Prediction: factoring before support may lower total quotient degrees enough to offset deterministic lift cost. Falsify it by measuring full circuit/setup/search/memory versus retained quintic predicates; do not infer gain from factor degrees.

Next independent oracle: exhaust d2/d3root multisets, including repeated/zero roots, arbitrary bounded factor pairs, and nonsplit factors. Compare output to brute bounded coefficient-factor enumeration on tiny degrees, not the same recurrence; explicitly reject the counterexample as a complete-support solution. Only then profile an actual k3 coefficient-family Boolean circuit. For odd extensions, replace constant-factor inverse by the corresponding Sylvester linear solve over the base field and prove digit-stratum coverage. Prime intervals need a separate integer carry/quotient proof; binary polynomial-prefix lifting does not establish that case.
