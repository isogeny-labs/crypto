# Joint Richelot model charts: first arithmetic unit

All eight fixed CT49 nondegenerate source/target pairs admit a common rational
nonbranch chart in this unit. Complete808-candidate traces are retained; exact
source factors, transformed roots, Frobenius, determinant, Wronskian and monic
target coefficients pass, together with inverse model substitution. All48
factor reorderings and32 legitimate sign choices pass; all4 degenerate and24
other semantic controls are detected. Claim42a9c2 is unsigned. [pcc:unverified]

| Pair | Eligible a | Selected a | b | c | Delta prime | bc | Constructor scale |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Cpi1_j1 | 38 | 3 | 22 | 4 | 99 | 88 | 13 |
| Cpi1_j2 | 38 | 3 | 22 | 12 | 2 | 62 | 39 |
| Cpi2_j1 | 36 | 3 | 49 | 39 | 86 | 93 | 8 |
| Cpi2_j2 | 36 | 2 | 15 | 44 | 86 | 54 | 47 |
| C10_j1 | 31 | 1 | 7 | 47 | 6 | 26 | 26 |
| C10_j2 | 31 | 1 | 7 | 8 | 95 | 56 | 45 |
| C20_j1 | 33 | 7 | 38 | 31 | 20 | 67 | 34 |
| C20_j2 | 33 | 1 | 28 | 42 | 25 | 65 | 36 |

The recorded monic coefficient vectors and all six cubic-field branch roots
are in [the literal output CSV](../evidence/first-arithmetic-unit-03/recorded-monic-models-and-roots-13.csv).
These are actual CT49 correctness fixtures. The constructor scale is its least
nonzero square root; it need not equal the recorded bc sign. Both are retained
for the later evaluator's orientation check. [pcc:unverified]

For w=1/(x-a), F=w^6 f(a+1/w)/b^2 and G=w^6 H(a+1/w)/c^2 are monic.
Writing alpha_i=q_i(a), S=product(alpha_i)=b^2/lambda, the binary coefficient
matrix has determinant-1, so Delta prime=-Delta/S. The Wronskian contributes
-w^2/(alpha_j alpha_k); the new raw Richelot target is exactly (bc)^2 G.
This is the written coefficient argument ee5c8b, with finite checks42a9c2;
it is not a new Lean construction or executed divisor correspondence.
[pcc:unverified]

Public source4df62c68/Rustf5dfad0a precedes compilation and the first arithmetic
test. Rust1.99.1, offline locked guarded vendor sources, one compiler worker,
120s caps and a separately copied555 qualifier were used. One new test passes;
12 unchanged backend tests were compiled and explicitly filtered. All13 phases,
strict Clippy/fmt and source/dependency/qualifier guards exit0. Complete75635B
stdout and the601280B/195-member declared archive remain; every archived byte
and mode was compared. Disposable build cache was excluded prospectively and
inventoried. No runtime ratio is inferred. [pcc:unverified]

Initial storage preflight6df51e and namespace compiler errord3ae2a are retained
in full declared packets. Neither ran arithmetic. The versioned C source fixes
only the wrapper alias and is frozen after this first arithmetic success.
Next is the existing generic operational Richelot/closed-fibre evaluator on
these monic models, with explicit dual[2] and scalar-relation checks.
[pcc:unverified]

Primary sources: full CT49/CT50 notes and ConsumerE079 nonsplit.rs/map.rs in
primary-inputs; Galbraith corrected [ch8](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch8.pdf)
and [ch10](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf).
The arithmetic backend is reused and the transform oracle has the same author;
the result is not a signed independent-verifier verdict. [pcc:unverified]
