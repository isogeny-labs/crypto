# Amendments

## Amendment 1 (2026-11-20, before any data; written while implementing, no result seen)

PROTOCOL.md v1 names factor bases F_B and the dimension l without fixing them. Fixed here, before the first run:

- Prime R3: F_B = {P in <G> \ {O} : x(P) < B}, B the smallest integer with |F_B| >= ceil(0.76 sqrt(n)) points
  (both signs), aiming at a decomposition density near 0.25; the realised density is reported per instance.
- Binary R3: V = span{1, t, ..., t^(l-1)} (integer encoding x < 2^l), l = floor(m/2).
- D (decomposable targets) = {+-P_i +-P_j : P_i, P_j in F, i <= j} \ {O} (m = 2 Semaev, repeated point allowed).
- R3 binary WHT effective test count: functionals c and c xor c_T (c_T the trace functional, constant on <G>)
  give equal |W|, so N_eff = 2^(m-1) - 1 in the Gaussian-max null.
- R2 spectral null variance uses sigma^2 = n (1 - mu^2), mu the predicate mean (the a = 0 term is excluded).
- R2 binary also emits Tr(x) as an `excluded_constant` row (R1 check by another route); it is not in the test count.
