#set page(paper: "a4", margin: 2cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Lift-dense factor bases: twist avoidance through the trace

Study `research/lift_dense_factor_base_20261027/`, 2026-10-27.
- Run: `reports/run-20261027T0356Z/`, master seed `0x11F7_2026_1027_C`.
- Protocol: `PROTOCOL.md`, frozen at `80ac6c421` before the run.
  Amendments, all made after the run:
  - 1 (`397f055bf`): empirical variance;
  - 2 (`ce62b2a9e`): rustfmt only, with byte-identical outputs;
  - 3 (`0e40561a6`): review fixes.
- Harness: `src/main.rs`, std-only Rust.
- Real-parameter check: `src/bin/c2pnb.rs`, native.
- Scoring: `summarize/`, native.
- Plotting: `plot.py`, layout only.
- Historical Python scripts (`c2pnb/*.py`, `summarize.py`) are kept as
  evidence and no longer executed.
- Lean: `lean/LiftDense.lean`, with the proof map in `lean/PROOF_MAP.md`.
  Theorem 2 is proved and audited; see §Lean.

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
  - Amendment 3 fixes a population mismatch: targets are never $O$ or
    $T = (0, sqrt(b))$, so the matching expectation is
    $(|F|^m - C_O - C_T)\/(\#E - 2)$, with $C_O$ and $C_T$ counted exactly.
    Scored against it (H-A″), the same single row fails ($z = -4.35$), and
    $z$ has mean $-0.18$ and sd $1.26$.
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

  *Proposition 3 (corrected in Amendment 3).* Existence of $u$ is a
  projective condition on $P(ker T)$, with $q = 2^d$:
  - $n = 2$: $ker T = FF_q$, so $u$ exists iff $c in FF_q$, and then every
    $u in FF_q^*$ works;
  - $n = 3$: $T(c\/u) = T(c u^q u^(q^2))\/N(u)$ is a binary quadratic form
    on $ker T tilde.equiv FF_q^2$. Its zero set in $P^1$ has 0, 1 or 2
    points, or all $q+1$ if the form vanishes. So the count is in
    $\{0, q-1, 2(q-1), q^2 - 1\}$;
  - $n = 4$: a plane cubic, so the count is $(q - 1)$ times its points.

  The first version claimed $\{0, 2(q-1)\}$ at $n = 3$. That is wrong:
  Codex review found $N = 15$, $d = 5$, $b = 4$ with count 31, the
  single-root case.

  *Exact computation, corrected rule (H-B′): pass.*
  - Exhaustive census over every $b$ at $(N, d) = (12,6), (15,5), (18,6),
    (18,9), (16,4)$, plus 4096 sampled $b$ at $(21,7), (20,5)$: 0
    violations.
  - At $N = 15$ the counts are 0 (15,376 of $b$), 31 (1,023 $approx 1\/q$)
    and 62 (16,368).
  - At $n = 4$, $u$ existed for every $b$ checked (minimum 150 at
    $N = 16$).

  *Line curves (H-B as registered, also H-B′): pass on 72 of 72.*
  - With $"Tr"(a) = 0$, every twisted line lifts 100 %.
  - With $"Tr"(a) = 1$, every one lifts 0 %.
  - $n = 2$: no $u$ on 32 of 32 random curves.
  - $n = 3$: 11 of 24 curves had a $u$; none was degenerate.
  - $n = 4$: counts are multiples of $2^d - 1$ (180–1116), and $u$ exists
    on 16 of 16 curves.

+ *Real parameters, c2pnb (exact computation).* For the ANSI X9.62 c2pnb
  curves ($N = 16k$, $a, b in FF_(2^16)$), $u$ is taken in $FF_(2^16)$, so
  $u FF_Q$ stays $pi_16$-stable.
  - *c2pnb208w1* and *c2pnb368w1* have $"Tr"(a) = 0$. They have 180 and 330
    valid $u$. 200/200 sampled twisted elements lift, against 103/200 and
    95/200 on the standard base $FF_Q$: *applicable*.
  - *c2pnb176v1, 272w1, 304w1* have $"Tr"(a) = 1$. The lines lift 0/200:
    *inapplicable*. The trace homomorphism puts the whole line on the twist.
  - The native check reproduces the historical Python output on every
    deterministic field: $"Tr"(a)$, $FF_(2^16)$ membership, the kernel size
    and the u-counts.

+ *Cost (model output, not a measurement).* Natively, in closed form:
  the 2LP optimum $T^* prop sqrt(N^3\/p)$ with $N' = 2N$ and $p' = 16p$
  changes the margin by $+0.50$ bits.

  The upstream C1 Gaudry–Diem $n = 4$ model (Python, crypto-autoresearcher)
  was run once with only those two inputs changed. Its output is quoted as
  historical evidence and not regenerated:
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

== Lean

`lean/LiftDense.lean` is a Lake project on the repository Mathlib pin.
- `lake build` is clean.
- The repository proof gate passes: 11 theorems, all on `propext`,
  `Classical.choice` and `Quot.sound`, with no `sorry` and no
  `native_decide`.

Proved, over any finite field $M$ of characteristic 2 and any intermediate
field $L$:
- Lemma 1, the trace tower (`trace_mul_algebraMap`).
- Theorem 2 in trace form (`twisted_line_trace_const`).
- $"Tr"(z^2) = "Tr"(z)$ (`trace_sq`).
- The Artin–Schreier criterion $exists z, z^2 + z = w <==> "Tr"(w) = 0$
  (`artin_schreier`). The range of $z |-> z^2 + z$ lies in $ker "Tr"$, and
  both have dimension $n - 1$.
- The curve lift criterion (`lifts_iff`).
- Theorem 2 in lift form (`twisted_line_lifts`): for $y in L^*$, $y u$
  lifts to $E_(a, c^2)$ iff $"Tr"(a) = 0$.

Still open, recorded in `PROOF_MAP.md`:
- Proposition 3, the u-count classification. It is checked exhaustively
  by census.
- The yield identity.
- The Weil cap. This needs the Kloosterman bound, which is not in Mathlib.

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
