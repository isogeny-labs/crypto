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

## Amendment 2 (2026-11-20, R4 controls only; no R4 output had been written)

The first R4 run hung: for j = 0 / j = 1728 the coefficient scan over B (resp. A) in 1..p-1 costs O(p) per
coefficient, and when none of the 6 (resp. 4) twist classes has prime order it scans all p of them. The scan is now
capped at coefficients 1..64, which covers every twist class (a coefficient set of size 64 hits all classes of
F_p*/F_p*^6 with overwhelming probability), and a prime with no prime-order twist in that range is skipped. The
selection rule (first 20 primes p >= 2^16 with a prime-order twist, first coefficient) is otherwise unchanged. The
R1, R2 and R3 code is unchanged; their data had already been written when this was found, and they do not touch R4.
- Second defect found in the same run: y^2 = x^3 + Ax (j = 1728) always has the rational 2-torsion point (0,0), so it
  never has prime order and the control loop could not terminate. The j = 1728 control now accepts #E = h n with
  h in {2, 4} and n prime, G = h P0. The j = 0 control keeps h = 1.
- Third defect, same run: the D = -7 curve y^2 = x^3 - 35x + 98 has the rational 2-torsion point (-7, 0) (the kernel
  of the norm-2 endomorphism alpha), so #E is always even and the "prime order" rule never fired. The D = -7 family
  now takes #E = h n, h in {2, 4, 8}, n prime; the eigenvalue of alpha on the odd prime-order <G> is unaffected.

## Amendment 3 (2026-11-20, POST-DATA; R2 null model; v1 results kept unchanged in results/r2.json)

Seen in the v1 data: every bit predicate on every prime curve had M/sqrt(n ln n) ~ 1.2-1.3, a systematic excess.
Cause (an instrument defect, not a property of the curves): x(-P) = x(P), so x-predicates are even functions of k
(f(kG) = f((n-k)G)), and y-predicates are odd ones (chi(-y) = -chi(y) for p = 3 mod 4; lsb(p-y) = 1 - lsb(y)). Their DFT
is purely real (imaginary), so |S(a)| is a *real* Gaussian, not a complex one, and the v1 tail e^{-t^2/sigma^2}
is too light. The v1 A/A arm used asymmetric random functions and could not see this.

v2 changes only the per-frequency tail for predicates detected as even or odd (exact check over k = 1..n-1):
erfc(t / sqrt(2 sigma^2)); and adds A/A arms of K = 64 random even and 64 random odd functions per instance. Instances,
predicates, statistic, threshold and test count are unchanged. Both v1 (`r2.json`) and v2 (`r2_v2.json`) are reported;
v2 decides, because v1's null was invalid for symmetric predicates, as the new A/A arm shows.
