#set page(paper: "a4", margin: 2cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Lift-dense factor bases: twist avoidance through the trace

Study `research/lift_dense_factor_base_20261027/`, 2026-10-27.
- Run: `reports/run-20261027T0356Z/`, master seed `0x11F7_2026_1027_C`.
- Protocol: `PROTOCOL.md`, frozen at `80ac6c421` before the run.
  Amendment 1 is at `397f055bf`, made after the run and before the rerun of
  the line cells.
- Harness: `src/main.rs`, std-only Rust.
- Real-parameter checks: `c2pnb/`.
- Scoring: `summarize.py`. Plotting: `plot.py`, layout only.

Each statement is tagged *theorem*, *exact computation*, *measured*,
*model output* or *not established*.

== Question

On $E: y^2 + x y = x^3 + a x^2 + b$ over $FF_(2^N)$, a nonzero $x$ lifts to
$E(FF_q)$ iff $"Tr"(x + a + b\/x^2) = 0$; otherwise it lifts to the
quadratic twist.

The earlier twist-obstruction study showed that twist points add no
relations. So in a factor-base subspace $V$, the twist-lifting abscissae
are dead weight. This study asks whether $V$ can avoid the twist, and what
the trace homomorphism, used as a membership condition rather than a sum
constraint, buys.

== Statements and their status

+ *Yield law (theorem).* Every ordered $m$-tuple of the factor base $F$
  sums to exactly one point. So the mean number of ordered decompositions
  of a target is $|F|^m \/ \#E$, with $|F| = 2|V inter L|$. A subspace with
  lift density $rho$ yields $(2 rho)^m$ times a random subspace of the same
  dimension.
  - *Measured:* 87 rows of point-arithmetic counts, $m = 2, 3, 4$. The
    pooled measured/predicted ratio per cell is 0.906–1.022, and every
    pooled cell is within 4 empirical SE.
  - The registered Poisson rule (H-A) *failed* on 25 rows: ordered counts
    are clustered, and the rule ignored that.
  - The amended rule (H-A′, empirical per-target variance, same targets)
    fails on *one* row: $N = 15$, standard line, $z = -4.35$. Over all 87
    rows, $z$ has mean $-0.28$ and sd $1.24$.
  - No bias is indicated. The residual overdispersion is a limit of the
    sampling model, not of the identity.

+ *Twisted line (theorem).* Let $N = d n$, $T = "Tr"_(N\/d)$ and
  $c = sqrt(b)$. For $x = u y$ with $y in FF_(2^d)$:
  - $"Tr"_N (c\/x) = "Tr"_d (T(c\/u)\/y)$;
  - $"Tr"_N (x) = "Tr"_d (T(u) y)$.

  So when $T(u) = T(c\/u) = 0$, the lift condition on the $FF_(2^d)$-line
  $u FF_(2^d)$ is the constant $"Tr"(a)$: all-lift if $"Tr"(a) = 0$,
  all-twist if $"Tr"(a) = 1$. The line is an $FF_(2^d)$-subspace, so
  Gaudry–Diem descent applies with the same monomial support.

  Existence of $u$ is a projective condition on $P(ker T)$:
  - $n = 2$: $ker T = FF_(2^d)$, so $u$ exists iff $c in FF_(2^d)$;
  - $n = 3$: a binary quadratic form, so the count is $0$ or
    $2(2^d - 1)$, with an isotropy (Artin–Schreier) criterion;
  - $n = 4$: a plane cubic, so the count is $(2^d - 1)$ times its points.

  *Exact computation* (H-B): pass on 72 of 72 curves.
  - With $"Tr"(a) = 0$, every twisted line lifts 100 %.
  - With $"Tr"(a) = 1$, every one lifts 0 %.
  - $n = 2$: no $u$ on 32 of 32 random curves.
  - $n = 3$: counts in $\{0, 2(2^d - 1)\}$ on all 24 curves; 11 of 24 had
    a $u$.
  - $n = 4$: counts are multiples of $2^d - 1$ (180–1116), and $u$ exists
    on 16 of 16 curves.

+ *Real parameters, c2pnb (exact computation).* For the ANSI X9.62 c2pnb
  curves ($N = 16k$, $a, b in FF_(2^16)$), $u$ is taken in $FF_(2^16)$, so
  $u FF_Q$ stays $pi_16$-stable.
  - *c2pnb208w1* and *c2pnb368w1* have $"Tr"(a) = 0$. They have 180 and 330
    valid $u$. 40/40 sampled twisted elements lift, against 23/40 and 25/40
    on the standard base $FF_Q$: *applicable*.
  - *c2pnb176v1, 272w1, 304w1* have $"Tr"(a) = 1$. The lines lift 0/40:
    *inapplicable*. The trace homomorphism puts the whole line on the twist.

+ *Cost (model output, not a measurement).* The upstream C1 Gaudry–Diem
  $n = 4$ model was rerun with only two inputs changed: base doubled and
  $p_"dec" = 16\/24$. The model is unchanged and copied with its sha256.
  - Under double large primes, for the applicable rows at
    $c_"trial" = 2^27$ and $2^30$:
    - time, memory model (a) and mesh-a: *+0.50 to +0.78 bits*;
    - memory model (b), whose storage grows with the base: $-0.51$ to
      $+0.57$ bits.
  - The time figure matches the analytic $T^* prop sqrt(N^3 \/ p)$,
    factor $2^(-1\/2)$.
  - The line always contains a standard-size sub-base: keep only half its
    points and the relation rate equals the standard one. So a solver can
    fall back, and the option is weakly dominant (an argument, not a
    measurement).
  - Under basic Gaudry the model is LA-dominated, and the doubled base costs
    $-2$ bits.
  - The model holds $c_"trial"$ fixed, an assumption. A measured
    improvement needs the upstream C1 toy ladder run with both bases.

+ *Prime degree, repulsion (measured, confirmatory at $N = 11$).* Exact
  unpruned counts were taken of all-lift $k$-dimensional subspaces,
  normalised by the exact expectation for a uniformly random set of the
  same size. The nulls are paired one-to-one by density.

  At $k = 4$, curves against nulls:

  #table(columns: 4, [$N$], [curves (mean ± sd)], [nulls (mean ± sd)], [one-sided permutation $p$],
    [9 (26 vs 26)], [0.683 ± 0.166], [1.021 ± 0.262], [$2 dot 10^(-5)$],
    [11 (26 vs 26)], [0.921 ± 0.018], [1.027 ± 0.053], [$2 dot 10^(-5)$ — *confirmed*],
    [12 (6 vs 6, composite)], [0.989 ± 0.001], [0.991 ± 0.020], [0.51],
    [13 (6 vs 6)], [0.981 ± 0.005], [0.994 ± 0.010], [0.019])

  Lift sets contain *fewer* all-lift 4-subspaces than random sets, and
  their counts are far less variable. The deficit shrinks with $N$ at fixed
  $k$. Exact maxima at prime $N <= 13$: the curve median kmax is at most the null median (4 vs 4 at $N = 9$, 4 vs 5 at $N = 11$, 5 vs 5 at $N = 13$). At composite $N = 12$ the koblitz curves reach 6 against 5, through the subfield construction. The
  planted subspaces were recovered on 100 % of instances.

  Above $N = 13$ the budgeted search recovered *none* of the planted
  subspaces, so those kmax values (all 5–6) are "found under budget" only.
  The subfield construction at composite $N$ shows what the budgeted
  search misses: dimensions 8–11 at $N = 16$–$22$.

+ *ECC2K-130 ($N = 131$, prime): no gain (theorem plus toy evidence).*
  - There is no subfield, so no twisted line exists.
  - Weil: an all-lift subspace satisfies
    $sum_(u in V^perp) K(c(1+u)) = plus.minus 2^N$ with
    $|K| <= 2^(N\/2+1)$. Hence its dimension is $<= N\/2 + 1$.
  - Random-set heuristic: the largest all-lift dimension is about 10 at
    $N = 131$, far below the 33–44 that index calculus needs.
  - The toy data show repulsion, not attraction.
  - *Not established:* that no algebraic all-lift family exists at prime
    $N$. The search cannot rule out needles above $N = 13$.

== Figures

#figure(image("reports/run-20261027T0356Z/fig1_lines.png", width: 100%), caption: [Composite degree. Left: lift fraction of the standard and twisted lines. Right: twisted/standard relation yield, measured (dots) and predicted (rings).])
#figure(image("reports/run-20261027T0356Z/fig2_repulsion.png", width: 100%), caption: [Prime degree: normalised all-lift 4-subspace counts, curves against density-paired nulls.])
#figure(image("reports/run-20261027T0356Z/fig3_kmax.png", width: 80%), caption: [Largest all-lift subspace by $N$. Blue: curves; grey: nulls; purple: subfield construction; gold: planted dimension.])
#figure(image("reports/run-20261027T0356Z/fig4_c2pnb_delta.png", width: 80%), caption: [C1 cost-model margin change (model output). Faded bars: $"Tr"(a) = 1$, not applicable.])

== The trace homomorphism: where it acts and what it buys

- *As a sum constraint (closed).* The Kummer map
  $P |-> "Tr"(x_P) + "Tr"(a)$ is the Kosters–Yeo trace equation: one linear
  equation, falling at degree 2. Any homomorphic filter is capped at
  $log_2$ of the cofactor (2 bits on ECC2K-130).
- *On points, $"Tr"_(q^k\/q)$ (closed).* Trace-zero and extension
  embeddings either kill the rational line or enlarge the problem.
- *As a membership condition on the base (this study).*
  - $"Tr"_(N\/d)$ makes the lift condition constant on a twisted
    subfield line. That is worth $2^n$ per trial at composite degree with
    $"Tr"(a) = 0$, and nothing at prime degree.
  - It also decides applicability: on c2pnb176v1, 272w1 and 304w1 the same
    construction lands on the twist.

== Not claimed

- No ECDLP was solved and no ECDLP cost was measured, so there is no
  leaderboard, benchmark or rho row.
- The c2pnb figure is a model delta.
- The repulsion is toy-size evidence ($N <= 13$, $k = 4$). Its mechanism
  (presumably Kloosterman-sum rigidity) is not established.

== Next

+ Run the upstream C1 toy ladder ($E\/FF_(2^4)$ lifted to $FF_(2^(4k))$)
  with both bases. This measures the relation rate per trial and
  $c_"trial"$. It decides whether the +0.5 bit is real for c2pnb208w1 and
  368w1.
+ Apply the same line to the Oakley EC2N quintic-tower lane ($n = 5$,
  $2^5 = 32$ per trial), after checking $"Tr"(a)$.
+ Derive the repulsion constant at $k = 4$ from the fourth moment of
  Kloosterman sums, and test it against $N = 15$ counts.
