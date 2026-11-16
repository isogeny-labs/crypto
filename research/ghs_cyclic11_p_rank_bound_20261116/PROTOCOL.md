# Three-branch cyclic-11 p-rank bound

16 November 2026. Before calculation, fix the scope as all smooth geometric
cyclic degree-11 covers of P1 with exactly three geometric branch points
in characteristic p with `ord_11(p)=5`. Over an algebraic closure, a
coordinate change and Kummer theory give
`y^11=x^a(x-1)^b`, with `1<=a,b<=10` and `c=-a-b mod 11` nonzero.
These are 90 parameter rows, with repetitions under automorphisms.

Apply the published signature and p-rank bound of Ozman--Pries--Weir
(Nagoya Math. J. 248 (2022), Lemma 2.1, Theorem 2.3, citing Bouw):
`s_i=-1+sum_j < -i*a_j/11 >` and
`f_p(C)<=sum_O |O| min_{i in O} s_i` over multiplication-by-p orbits.
The two orbits when `ord_11(p)=5` are
`R={1,3,4,5,9}` and `N={2,6,7,8,10}`. Check all 90 rows exactly,
including each signature's five one-dimensional eigenspaces and the
minimum on both orbits. If both minima are zero in every row, the
published theorem gives the geometric p-rank bound zero. A Lean file
checks the finite modular arithmetic; the geometric theorem and Kummer
normal form remain external assumptions.

The original p-rank theorem and cyclic-cover method are prior art. This
specialization may be useful as a GHS eligibility filter, but the exact
scope's publication priority is unresolved. It does not imply a new
vulnerable family or exclude other cover types or supersingular sources.

Primary source: https://doi.org/10.1017/nmj.2022.12
