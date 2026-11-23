# Native development prototype: norm and shared membership encodings

This standalone C++17 tool constructs and checks **known-answer selected-chart witnesses**. It exports two quadratic systems for each of arities 3, 5 and 7. It does not search for an unknown decomposition, run a Gröbner solver, measure performance, estimate degree of regularity, or solve a discrete logarithm.

The fixed fixture is the nonsingular curve `y^2 = x^3 + 2x + 2` over `F17`. Exact point enumeration checks `#E(F17)=19`, prime and different from the field characteristic. The membership base is `x^16=1`, so these tiny development cases have a dense base, deliberately **not** an asymptotic or security benchmark.

## Reproduce the checks and exports

```sh
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror \
  research/prime_one_shot_20261029/prototype/norm_tower.cpp \
  -o /tmp/norm-tower-development
/tmp/norm-tower-development --selftest-export /tmp/norm-tower-exports
```

The source and all retained exports are bound by `files.sha256`. `check.log` contains the final native development output; `build-receipt.txt` records exact build/run commands, compiler identity, source/binary hashes and exit status. This is a local correctness check, permitted separately from the repository's EKS scientific-measurement workflow. No elapsed timings are collected or compared.

## What is implemented

- Native polynomial arithmetic, division, derivative and extended Euclidean witnesses over `F17`.
- An independent affine elliptic group law used to sum the input points.
- Monic `A` and `C` interpolation for `A(x)+C(x)y`, with the residual zeros and `-R` prescribed. The interpolation fixes a known-answer fixture; its bounded sign/point selection is not proposed as a decomposition algorithm.
- The exact norm identity `A^2-f C^2=(X-x_R)U0` and the linear `-R` condition.
- Shared Graeffe membership, with odd-degree transform `T O(T)^2-E(T)^2`.
- Shared remainder membership, with retained quotient coefficients: `R0=X`, `R_(j-1)^2=Q_j U0+R_j`, and `R_t=c`. Retaining each `Q_j` avoids expansion of the remainder-reduction circuit. Both presentations have quadratic coefficient equations and linearly many coefficient variables per layer.
- Explicit selected-chart constraints: polynomial Bezout witnesses for `gcd(C,U0)=1` and `gcd(U0,U0')=1`, plus an inverse witness for `U0(x_R)!=0`.
- Native evaluation of every exported polynomial at the retained assignment, recovery of each sign through `-A/C`, and rejection of a changed norm coefficient.
- Invalid split-terminal, target, repeated-root and zero-denominator cases.

| Arity | Graeffe variables/equations | Remainder variables/equations | Largest input degree |
|---:|---:|---:|---:|
| 3 | 25 / 27 | 33 / 35 | 2 |
| 5 | 42 / 44 | 58 / 60 | 2 |
| 7 | 59 / 61 | 83 / 85 | 2 |

These counts describe an input encoding, not solver work. The known assignments are not independently found solutions. The parent study's separate parser re-evaluates the textual exports without using this tool's expression-tree evaluator.

## Two mandatory distinctions

**Graeffe is not the same scheme as divisibility.** Development regressions compute in `F17[epsilon]/(epsilon^2)`. For the squarefree base polynomial `U=X^2-1+epsilon X`, the even-degree Graeffe transform is exactly `(T-1)^2` modulo `epsilon^2`: the `epsilon X` perturbation disappears. But `(X^2-1) mod U=-epsilon X` is nonzero. The odd-arity counterpart uses roots `1+epsilon,-1+epsilon,4`, with `U0=(X-1)(X+1)(X-4)+epsilon*(-2X^2+8X)`. Two Graeffe steps give `(T-1)^3` with zero epsilon coefficient; the first-order remainder of `X^4-1` modulo the perturbed `U0` is nonzero. Thus terminal Graeffe admits an infinitesimal tangent not admitted by divisibility, even at squarefree `U0` and odd arity. These are transform/constraint checks, not a performance conclusion about the full norm system. The remainder system is retained as a distinct comparator.

**Rational abscissas are not rational points or coefficient solutions.** A separate native `F17^2` implementation uses `z^2=3` and checks:

```text
T=(1,8z), 2T=(12,16z), -3T=(15,12z), sum=O;
P=(5,1), Q=(6,3), P+Q=R=(10,6).
```

The five residual points `T,2T,-3T,P,Q` sum to rational `R` and have distinct rational abscissas `{1,12,15,5,6}`, all satisfying `x^16=1`, disjoint from `x_R=10`. Extension-field interpolation constructs nonrational `A,C` and verifies their full norm identity on the coprime chart. At least one coefficient fails `z^17=z`, and an explicit `F17` coefficient-domain check rejects the witness. This is a geometric solution that must not be accepted as a rational decomposition.

The `.ms` exports use characteristic 17 but **do not include coefficient field equations**. A Gröbner system over `F17` describes geometric solutions over an algebraic closure, not automatically only `F17`-valued coefficient assignments. The known assignments supplied here are rational, but a future solver must enforce or recover coefficient rationality and charge its cost. Adding `v^17-v` would alter the input degree and is not silently done here.

## Export format

`exports/m{3,5,7}_t4_{graeffe,remainder}.ms` contains:

1. Comma-separated variable names.
2. The characteristic `17`.
3. Parenthesized polynomial expressions using `+` and `*`, separated by commas. Squares appear as products; constants are residues 0 through 16.

Matching `.assignment.tsv` files have a `variable<TAB>value` header and one rational assignment per variable. Matching `.witness.json` files retain the curve, target, source summands, `A,C,U0`, system sizes and assignment. They are development fixtures, not an `ecbench` receipt, an independent solver certificate, a machine-checked general identity proof, or a `VALIDATION.json`.

## Reading, failures and limits

Before implementation, the relevant primary text was read from Petit-Kosters-Messeng sections 3.1-3.3 in `/workspace/audit/PKM2016.txt`, Nagao's frozen Riemann-Roch/norm construction sections 3-4 in `/workspace/audit/Nagao2013.txt`, and the local predecessor `IDEA-20261029-3d0e71`. The proposal and both independent review corrections determine the checked semantics. These ingredients and the five-point construction are prior art.

`conductor check` was attempted and returned command-not-found, matching the parent environment diagnosis. Initial strict compilation stopped on misleading-indentation warnings and a temporary-reference range-loop warning; formatting/declaration changes fixed them without weakening flags. The quadratic-extension addition introduced one further misleading-indentation warning, also fixed. No scientific hypothesis test failed or was rerun as a timing result.

The prototype checks bounded known answers and malformed cases. It does not prove complete model-set equivalence for every target or exceptional chart, establish radicality of the full norm system, enforce rationality in the exported geometric equations, implement all-root rational extraction, establish useful relation rank, justify growing-arity scaling, or deliver a sub-exponential algorithm. New scientific comparisons must use the frozen protocol and prescribed execution route in the parent report.
