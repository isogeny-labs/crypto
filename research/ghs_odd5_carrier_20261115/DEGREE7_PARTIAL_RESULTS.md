# Degree-seven partial cover screen on the ten trace-+26 targets

16 November 2026 UTC. All records in this note remain `[pcc:unverified]`. This result does **not** raise the established first-cover interval `[7,20]` because four denominator classes remain unsearched.

## Exact coverage and observed result

Let `K=F3[a]/(a^5+2a+1)`, `Y/F3` and the ten `E_b/K` be exactly as in `TRACE26_CLASS_THEOREM.md`. Translate the image of the rational point at infinity to the elliptic origin. For a separable degree-seven map, `u=A/B` has `deg A=7`, `deg B∈{0,2,4,6}` and the parity rule for branch and nonbranch poles applies. With `f=(x+1)Q1...Q5`, the two frozen producers cover the nine B shapes below for every b-mask `3,27,30,103,139,167,182,194,230,233`:

| B shape | Logical branch/pole assignments per target | Full-equation hits across ten targets |
|---|---:|---:|
| `1` | 243 | 0 |
| `Qi` | 405 | 0 |
| `Qi^3` | 405 | 0 |
| `QiQj` | 270 branch assignments, each with 242 nonzero leading coefficients | 0 |
| `(x-c)^2` | 58,806 | 0 |
| `(x-c)^4` | 58,806 | 0 |
| `(x-c)^6` | 58,806 | 0 |
| `Qi(x-c)^2` | 98,010 | 0 |
| `Qi(x-c)^4` | 98,010 | 0 |

The ten `degree7_sparse_*.json` receipts and `degree7_sparse_manifest.json` are bound to the frozen source hash `e1e0581ffd1072f1729a90b888b4ac77aca051f0927ca72164a53beba20ef935`; their measured wall times total 431.37 seconds. The ten `degree7_linear_square_*.json` receipts and matching manifest are bound to source hash `78b2b546759f868e2e12afed808b6428bf5dee9c420e2469835ecf81183885ca`; their wall times total 38.59 seconds. Every mask finished successfully with zero hits. The square-shape producer solves exact one-variable coefficient equations, including identically-zero fallback cases. A separately frozen direct-c CRT implementation replayed all five square shapes for b-mask 3 and matched every logical-assignment, degree-seven-numerator, coprime, divisible, and hit counter; this replay took 270.33 seconds. It does not independently replay the other nine masks.

## Remaining classes and claim boundary

The parity and degree rules give the complete B classification as `H²` with monic `H` of degree 0,1,2,3 coprime to f; `Qi H²` with H of degree 0,1,2 coprime to f; `QiQj H²` with H of degree 0,1 coprime to f; `QiQjQk`; and `Qi³`. The nine screened shapes leave:

1. `H²` with `deg H=2 or 3` and H not a power of one linear factor;
2. `Qi H²` with `deg H=2` and H not a square of one linear factor;
3. `QiQj(x-c)²` with `c!=-1`;
4. `QiQjQk` for distinct i,j,k.

These shapes include large parameter spaces and free numerator coefficients. They must be solved before claiming that degree seven is excluded for any target. The degree-four through degree-six obstruction and Howe ceiling continue to give `[7,20]`. A positive degree-seven hit would still need independent morphism and subgroup checks, descent to the small-field Jacobian, and full attack-versus-rho measurement. No new vulnerable-family or attack-method claim follows from this partial screen.
