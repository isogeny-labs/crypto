# Frozen four-branch cyclic-11 eligibility screen

Date: 2026-11-16. Question: for characteristic 3 or 5 and base field
F_p, which four-geometric-branch cyclic degree-11 covers can even carry a
non-descended **ordinary** elliptic source over F_(p^5)?

## Literature and premises checked before computation

- Ozman--Pries--Weir (2022), Lemma 2.1 and Theorem 2.3, give the
  signature and Bouw p-rank upper bound for cyclic covers:
  https://doi.org/10.1017/nmj.2022.12 .
- Iijima--Momose--Chao (2015) classify the `(2,...,2)` odd-characteristic
  direct-cover setting through extension degree seven under their
  hypotheses; this different cyclic-11 carrier is not claimed new from
  a search failure: https://eprint.iacr.org/2015/805.pdf .
- The unique quadratic subfield of Q(zeta_11) is Q(sqrt(-11)):
  https://www.math.purdue.edu/~jcumberb/SANTA/nicoQRnotes.pdf .
- Waterhouse's elliptic trace/endormorphism classification, primary
  thesis entry: https://eudml.org/doc/81852 .
- The local F3 four-branch point-count screen is
  `../ghs_cyclic11_genus10_20261116/RESULT.md`. Its trace -31/-9
  example is a positive control for distinguishing ordinary and
  supersingular factors.

The shared research KB was queried on 2026-11-16 (revision
`9141e386254340a9bdba3036f2bb706a`, last refresh 2026-11-14);
targeted web search found no exact four-branch CM exclusion statement.
That search does not establish publication priority.

## Frozen finite population and checks

Enumerate ordered `a,b,c` in `1..10`, put `d=-(a+b+c) mod 11`, and
retain `d!=0`. The geometric normal form is
`y^11=x^a(x-1)^b(x-lambda)^c` with four distinct geometric branch
points, for any permitted lambda over an algebraic closure. Compute
`s_i=-1+sum_{e in {a,b,c,d}} ((-i*e mod 11)/11)`, the two order-five
orbits R and N, and `B=5*min_R s_i+5*min_N s_i`.

Primary falsifier: test whether `B=10` holds exactly for the exponent
multisets that split into two opposite pairs `{u,-u,v,-v}`. Retain
counterexamples if false. Report counts for every bound value and the
number of ordered exponent rows.

Separately enumerate integer ordinary elliptic traces `t` satisfying
`t^2+11m^2=4p^5`, `|t|<=2sqrt(p^5)`, and `p` not dividing `t`, for
`p=3,5`. Compare each with degree-five traces descended from F_p,
using `t5=s^5-5ps^3+5p^2s`. This finite arithmetic is a necessary
CM-trace filter only; no elliptic source curve, subgroup map, target
decomposition or attack timing is inferred from it.

The code is a standalone native Rust program, compiled with `rustc -O`.
The mathematical implication from a p-rank bound at most five to the
CM field is a paper argument in the report, not an output of the code.
Failure/censoring, command, compiler version, stdout and source hash
will be retained. No parameters will be changed after a run; any
correction is a versioned amendment.
