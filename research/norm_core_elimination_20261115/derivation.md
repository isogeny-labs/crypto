# Exact core substitutions and their cost boundary

This is a written derivation, not yet a completed Lean endpoint.
PCC-20261115-be28ec:unverified. [pcc:unverified]

Write A=X^h+sum_(i<h) a_i X^i and C=sum_(i<h-1)c_i X^i, with m=2h-1.
At R=(r,s) on y^2=f(x), the linear target equation uniquely determines

    a0=s C(r)-r^h-sum_(1<=i<h) a_i r^i.

It follows that (A^2-f C^2)(r)=s^2 C(r)^2-f(r)C(r)^2=0. Division by the
monic linear polynomial X-r therefore has zero remainder and a monic
quotient U of degree m. If N=A^2-f C^2=sum N_i X^i, its coefficients are

    u_m=1; u_j=N_(j+1)+r u_(j+1), j=m-1,...,0.

These are polynomial functions, without denominator or field search. Removing
a0 and the m U coefficients is unit-coefficient graph elimination. The norm
and target equations vanish identically after substitution; every solution of
the substituted remaining equations has a unique lift to the full system.
[pcc:unverified]

For a monic degree-m U, division of X^e with e<m has zero quotient and
remainder X^e. Consequently every initial chain layer with 2^j<m has fixed
public quotient/remainder coordinates. At the first layer with 2^j>=m,
the input is still X^(2^j) and the quotient degree is at most2^j-m; its
higher declared coefficients are zero. Uniqueness of monic division justifies
removing them too. Later retained unknown layers are unchanged. At terminal
layers below m the resulting constant nonzero equations correctly exclude all
solutions; do not invent a remainder variable to hide them. [pcc:unverified]

For t4, witness-pruned counts24,42,60 become15,18,22 at arities3,5,7. Norm
coefficient expressions are quadratic; the remaining products Q_j U and the
target inverse have degree at most3. This does not prove that construction of
a Groebner basis becomes easier. Asymptotically v remains O(mt): the early
public prefix removes only O(m log(m)) coordinates, and eliminating U/a0
removes O(m). No subexponential total bound follows from these counts.
[pcc:unverified]

Sources: parent shared norm study and the one-shot initial-elimination question;
standard polynomial remainder theorem and uniqueness of monic Euclidean division.
The shared library already checks the nonzero-Kummer/squarefree norm charts.
The new graph/chain endpoint still needs a faithful formalization assembly,
not a restatement of those conclusions as assumptions. [pcc:unverified]
