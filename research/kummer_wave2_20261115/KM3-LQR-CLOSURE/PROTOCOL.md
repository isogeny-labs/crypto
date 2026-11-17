# PROTOCOL: KM3-LQR-CLOSURE (wave 2, lab A), 2026-11-15 [pcc:unverified]

Frozen before any data. Every number produced is [pcc:unverified].

## Question
Does the D10 functional (lambda3, lambda(g) = sum_x x1 x2 x3 <u>(g f1^-1 mod f2)) behave as a closure
pseudo-solution at m = 3 Kummer chains, i.e. does it vanish on V_{F,c} for every c < d_F? Is
d_F^(f2) = d_F, where d_F^(f2) = least c with 1 in V_{F,c} + (f2)? Does the m = 2 control
(lambda_f(g) = sum x1 x2 g/f, "Conjecture Lambda") hold on native Kummer cells?

## Route
Independent Python HKY closure (`hky_closure.py`) on the Fourier/monomial basis of
R = F_p[z1,z2,z3,u]/(z_i^K - 1) (D2), with the exact HKY rule: V_{F,c} = least subspace W of R_{<=c}
containing the generators of degree <= c and closed under multiplication by every tower variable
y_{i,l} = z_i^(2^l) and by u of its elements of degree < c (degree = popcount sum + u-degree). The Rust
engine cannot be built here (cargo --offline fails: crate aws-credential-types is not vendored), so
the route is Python, validated against committed Rust values (see Controls).

## Instances
- m = 3: primes 40961 and 12289; t = 2, 3; draws idx 0, 1, 2; the same draws as lab N1
  (`ns_degree.make_instance`, seed sha256 "kummer_m3|p|t|idx"). F1 = S3(x1,x2,u), F2 = S3(u,x3,x_R).
  Optional t = 4, one draw per prime, only if time allows.
- m = 2 control: same primes; t = 2..7 (more if time), 3 draws, F = S3(x1,x2,x_R) pointwise
  (`make_instance(..., m=2)`), no u.

## Measurements
1. d_F: least c with 1 in V_{F,c} (c from 2).
2. d_F^(f2): least c with pi(1) in pi(V_{F,c}), pi: R -> R/(f2) = pointwise F_p[u]/f2(x3,u) (dim 2K^3).
3. For each c < d_F: does lambda vanish on a basis of V_{F,c}? Report c* = largest c >= 2 with
   lambda(V_{F,c}) = 0. Also lambda(1) != 0.
4. m = 2: same with lambda_f; d_F(f); largest c with lambda_f(V_c) = 0.

## Predictions (the lead's, preregistered)
- P1: d_F^(f2) = d_F on every m = 3 cell, and d_F = 5 (t = 2), 6 (t = 3) as committed
  (`research/pkm_falldegree_unblock_20261107/RESULTS_STAGE_C.md` Tables 1, 2).
- P2: lambda(V_{F,c}) = 0 for every 2 <= c < d_F, and lambda(1) != 0, so c* = d_F - 1 on every cell.
- P3: m = 2: d_F = 4, lambda_f kills V_c for every c < d_F, lambda_f(1) != 0, c* = d_F - 1.

## Kill / survive
- P1 dies on any cell with d_F^(f2) < d_F (report which c).
- P2 dies on any cell and any c < d_F with lambda(V_c) != 0 (report the count of rows with
  lambda != 0 and the dimension of the annihilator of pi(V_c) in (R/(f2))^*).
- P3 dies on any m = 2 cell with c < d_F and lambda_f(V_c) != 0.
- A claim "survives" only as: no counterexample on the listed cells. Nothing beyond these cells.

## Controls
- Engine validation: d_F must equal committed Rust values: m = 2 -> 4 (t = 2, 3), m = 3 -> 5 (t = 2),
  6 (t = 3). Failure invalidates the Python engine and the whole run.
- Engine positive control (planted common root, `ns_degree.planted_instance`): closure must not
  contain 1 up to c = d_F + 1 and pi(1) must not lie in pi(V_c).
- Negative control for the functional (the test has power): lambda of a different draw (same p, t, idx+1),
  evaluated on V_{F, d_F - 1} of this draw, must be non-zero on most cells.
- Functional sanity: lambda(g f2-multiples) = 0 on random elements; lambda on rows re-evaluated through
  the pointwise (grid-value) formula equals the DFT-column formula on random rows.
- c = d_F: lambda(1) != 0 while 1 in V_c (lambda cannot be a witness once 1 is in V_c).
- Determinism (A/A): one cell is run twice; outputs identical.
- Not done (deviation to be reported): quotient by f1 instead of f2; null-support chain.
