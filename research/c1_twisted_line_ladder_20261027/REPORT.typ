#set page(margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")
= C1 toy ladder: standard base vs twisted all-lift line, n = 4

2026-10-27. Protocol `PROTOCOL.md` (frozen before the run). Raw rows and
summary in `reports/run-20261027T1502Z/`. All numbers are exact counts; no
wall-clock claim.

== Question
The upstream C2PNB cost model assumes the per-decomposition solver cost
$c_"trial"$ is unchanged when the Gaudry-Diem base $x in F_Q$ is replaced by a
twisted line $u F_Q$ ($T(u) = T(sqrt(b)/u) = 0$), which makes every abscissa
lift and raises the relations per trial by about $(2 rho)^4 approx 16$. This is
the toy ladder: $E$ over $F_(Q^4)$, $Q = 2^k$, $k in {5,7,9,11}$ for yield and
$k in {5,...,15}$ for cost.

== Method
- Yield: exact ordered counts of $R = P_1+P_2+P_3+P_4$ by a pair table, same
  targets for both bases, empirical-variance z tests, paired bootstrap.
- Cost: $S_5$ by resultants, symmetrised; in characteristic 2 it is a polynomial
  in $e_1^2, e_2^2, e_3^2, e_4$. With $x_i = u y_i$: four equations over $F_Q$
  in $(E_1..E_4)$, weights $(2,2,2,1)$, 105 monomials, weighted Bezout 512.
  Solver: weighted Macaulay matrix at $D = 26$, forward elimination with
  sparsest pivots; counted: $F_Q$ multiplications and inversions.
- Curves: family A ($a, b in F_16$, the task's), A0 ($tau = 0$ control), NEG
  ($"Tr"(a) = 1$), family B (generic $b$, $u$ by search in $ker T$).

== Results
#figure(image("fig_c1_ladder.png", width: 100%),
  caption: [(a) yield ratio, measured with 95% CI and exact-count prediction. (b) cost ratio, geometric mean with 95% bootstrap, tolerance [0.8, 1.25].])

*Yield.* H-Y2 passes: prediction inside the interval in 34/36, pooled
observed 17.3254 against predicted 16.9227 (1.0238). H-Y4 passes exactly: with
$"Tr"(a)=1$ the twisted line has 0 lifting abscissae and 0 relations at
$k = 5,7,9$. H-Y5 (A/A) passes 18/21, but on A2 the two valid $u$ give
identical counts. H-Y1 fails (48/54 rows within $|z| <= 3$; five rows beyond 5, all on
A0: $k=9$, $z=-5.17$, and four rows whose counts are all zero against a
prediction near 15.5, kept in the gate with $z = -infinity$) and H-Y3 fails: A0 ($b = 1$) has extremely clustered
counts, so the control is uninformative. 16678 relations enumerated at $k=5$
all satisfy the descended equations (0 violations).

*Cost.* Matrix dimensions are identical on all 594 solves (4840 x 4200, 40.7 MB),
rank and Hilbert function agree on 396/396 pairs, $"HF" = 512$ everywhere,
first reached at $D = 25$. Counted mults ratio twisted/standard, both closed
($n = 384$): geometric mean *0.9606*, 95% [0.9279, 0.9913], inside the
preregistered tolerance [0.8, 1.25]: *unchanged* for this solver. Family A
0.9097 [0.8627, 0.9539], family B 1.0045 [0.9788, 1.0327]; A/A noise reference
1.0596 [1.0043, 1.1266]. At $k >= 11$ the spread vanishes (A about 0.93, B about
1.00). Per-solve ratios range 0.476 to 2.026 (87% inside the tolerance).
11 of 594 systems (1 standard, 10 twisted) do not close at $D = 26$: their
top-weighted forms have a common zero at infinity (target-dependent, also on the
standard base); a full F4 would be needed.

== Limits
Simplified solver, not F4; no rational-solution extraction; no timing; $a, b in F_16$
forces a $tau$ structure (only two $tau = 1$ classes), hence the extension
family B; family B at $k >= 9$ has no absolute yield prediction.
The measured per-trial elimination cost supports the model's "$c_"trial"$ unchanged"
assumption to within about 4% (generic) to 9% (family A) for this solver.
