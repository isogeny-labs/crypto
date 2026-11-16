# L3 audit-item closure: PROTOCOL (written 2026-11-01 before any run below)

Scope: close open AUDIT items of N1..N4 (see each AUDIT.md). Mostly README/RESULTS wording and number fixes
(no prediction needed). Two computations get pre-registered predictions here.

## C1. N3 m=3, p=13, seed-0 failure of premise P (gain 3 vs Sigma k = 1, l=1, box4)
Method: rebuild f exactly as armA.py mode m3 (rng=random.Random(13), fpoly(3,13,rng)); enumerate ALL of V(F_13)
in F_13^3 (2197 points, no sampling); compute the kernel of box4 evaluation on all points over F_13; factor f over
F_13 and over F_13^2 (sympy / PARI); check whether kernel elements share a factor with f.
Predictions (pre-registered):
- C1a: exhaustive kernel dim = 3 (sampling was not the cause). Falsified if exhaustive dim = 1.
- C1b (main hypothesis): f is reducible over F_13 (or over F_13^2), and the extra kernel elements are
  products (factor of f) x (other low-degree polynomial) not divisible by f in box4, i.e. the excess is a
  reducible-fibre degeneracy of this one specialisation, not a failure mode that survives at large p.
  Falsified if f is absolutely irreducible over F_13 (checked over F_13 and F_169).
- If C1b is falsified, the cause is recorded as undiagnosed with the kernel basis saved.
Same check on the passing cells p=11, p=17 (seed 0) as controls: predict f irreducible there.

## C2. N1 n>=27 re-analysis
Rerun analysis/analyze.py on current raw. Prediction: n=27 and n=29 numbers (if complete) keep A/C1/C2/D1
with 1 in W_4 on all UNSAT draws; the n=31 stall noted in the NEXT4 addendum (C1/C2/D1 0/8) is reproduced from
raw. No new runs of run_main.

Budget: <= 2 threads, < 10 CPU-min total. Outputs < 5 MB.
