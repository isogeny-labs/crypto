#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
#show table: set text(size: 7.5pt)

= The linear stage after F4, end to end, on a corpus of Gröbner bases

2026-10-27 · `research/postgb_linear_stage_20261026` · protocol frozen in `544eb84f95` before any measured session;
amendments 1 (`4d55b7306e`, harness F4-budget bug, every card re-run as R2) and 2 (`42bb4d54fb`, host change).
253 systems in five families, 7 interleaved rounds per system, single thread. Every arm agreed on every system.

== Question

The owner asked for the linear-algebra stage to be sped up until it takes the leaderboard's linear-algebra section.
The measurement is end to end for the linear stage alone, not the whole index calculus. It runs on a variety of systems
whose Gröbner bases come out of F4.

*The linear stage* takes a monic grevlex basis (`f4_fp`) to every $F_p$-rational point, each checked against the input:
staircase, normal forms, $M_(x_n)$, minimal polynomial, roots, coordinates and the check.

== Arms

- *dense:* all $M_i$, Hessenberg characteristic polynomial, a null space per root.
- *sparse (the incumbent,* the leader of the IC leaderboard section "Linear stage after F4 (sparse FGLM)"*):*
  Wiedemann/Berlekamp–Massey on $M_(x_n)$, then shift-chain eliminations per root.
- *fast (new, `zero_dim_la::fast_stage`):*
  - $M_(x_n)$ as hybrid dense/sparse columns;
  - one transposed Krylov pass $(M^T)^j u$ for every coordinate;
  - Berlekamp–Massey;
  - roots from $gcd(f, x^p - x)$, with Karatsuba/Barrett powering;
  - coordinates read out by the rational univariate representation;
  - lazy u32/u64 arithmetic with AVX-512/AVX2 kernels chosen at run time;
  - falls back to dense inside its own clock.

== Result

#table(columns: 3, align: (left, left, left),
  [*prediction*], [*statement*], [*outcome*],
  [P1], [every family sparse/fast ≥ 1.5, interval above 1], [*held*: 7.1–45.5×],
  [P2], [in `dense`, sparse/fast grows with δ], [*held*: Spearman ρ = 0.93 (153 systems)],
  [P3], [fast gives up on ≤ 5%], [*held*: 1 of 253 (`gaudry-d4 p97-4`, $x_n$ not separating)],
  [P4], [DLP logs correct; counted muls within ±25% on every system; S within ±10%; lower linear-stage wall time], [*failed* on the counted clause (0.76–1.90; outside on 87 systems); the other three held],
  [P5], [portable fast (no vector kernels) still ≥ 1.2], [*held*: 6.3–11.1×],
)

=== The linear stage is 7–46× faster than the incumbent in every family (host B)

#table(columns: 8, align: (left, right, right, right, right, left, left, left),
  [*family*], [*systems*], [*δ*], [*sparse ms*], [*fast ms*], [*sparse ÷ fast [95%]*], [*dense ÷ fast*], [*counted sparse ÷ fast*],
  [jv-nagao], [48], [64], [3.05], [0.40], [*7.60 [7.43, 7.77]*], [17.9], [0.82–1.10],
  [gaudry-s4], [24], [64], [2.21], [0.32], [*7.13 [6.79, 7.48]*], [14.6], [0.84–1.22],
  [gaudry-d4], [24], [512], [313], [16.5], [*17.8 [13.7, 20.5]*], [109], [0.80–1.36],
  [gaudry-s5], [4], [4096], [229,741], [4,849], [*45.5 [42.8, 47.2]*], [—], [0.76–0.88],
  [dense], [153], [32–1024], [104], [4.64], [*20.0 [18.3, 21.8]*], [60.6], [0.92–1.90],
)

The ms columns give each family's median system. The ratio is the geometric mean of the per-system median wall-time
ratios, with a bootstrap interval over systems.

#figure(image("figures/family_speedup.png", width: 92%),
  caption: [Per-family speed-up. The host-A bars are the first runs L1–L4 on the other host. The triangles are the
  portable ablation.])

#figure(image("figures/per_system.png", width: 62%),
  caption: [Every system: sparse against fast median wall time.])

*Where the gain comes from.* It grows with δ (Spearman ρ = 0.93), and it is in the cost per multiplication, not in
their number.

- *Counted multiplications:* the arms count differently.
  - The incumbent counts products with non-zero operands, `fast` every product.
  - They are within 0.76–1.36× on the IC families, and `fast` does up to 1.90× fewer on small `dense` shapes.
  - P4's preregistered ±25% fails on that clause.
- *Vector kernels:* they are worth 1.0–4.0× (portable ÷ vector `fast`: 1.15, 1.04, 2.16, 4.01, 1.83). The remaining
  6–11× is the algorithm and the arithmetic layout.

#figure(image("figures/dense_vs_delta.png", width: 62%), caption: [`dense` control family: speed-up against δ.])

#figure(image("figures/fast_phases.png", width: 70%),
  caption: [Where `fast` spends its time.
  - The Krylov pass takes 23–90% (90% at δ = 4096).
  - The root step takes 16–37%.
  - The normal forms take up to 29%.])

=== Inside the index calculus: `jv_cover` DLP, end to end

R2-D1 and R2-D2 ran the same command with the same seeds and relations, `JV_LIN=sparse` against `JV_LIN=fast`, beside
the same-process rho. Every log was correct.

#table(columns: 7, align: (right, right, right, left, right, left, left),
  [*p*], [*seed*], [*rel.*], [*lin. stage s*], [*lin ÷*], [*S*], [*S/rho (distance to rho)*],
  [53], [1], [31], [64.8 → 12.8], [*5.07×*], [731.6 → 754.4], [664 → 685],
  [53], [2], [33], [29.5 → 6.2], [*4.79×*], [328.8 → 338.6], [286 → 294],
  [101], [1], [53], [162.0 → 22.6], [*7.16×*], [224.4 → 229.8], [151 → 155],
  [101], [2], [50], [220.7 → 36.5], [*6.05×*], [365.5 → 374.4], [292 → 300],
)

#figure(image("figures/jv_cover_dlp.png", width: 88%), caption: [`jv_cover` DLP: linear-stage wall time and S/rho.])

*Linear stage.* Its wall time inside the IC run falls 4.8–7.2×. The whole run is 27–37% shorter at p = 101 and
unchanged at p = 53, where the same-process rho dominates.

*Counted S rises 2.4–3.1%.* The fast arm counts every product, about 10–14% more linear-stage multiplications here.

*Tables A–C.* No row moves.

*§5a band.* `jv_cover` emits no unpriced native work, so S is the same at 0, 0.1 and 1 GAE per operation. Rho is
cheaper than both arms across the band. S/rho, 151–685, is the distance remaining.

=== Host dependence

The worker moved hosts between L4 and the R2 cards:
- host A: Xeon 2.1 GHz, 8 MiB L2, 260 MiB L3;
- host B: Xeon 2.8 GHz, 4 MiB L2, 33 MiB L3.

Hosts are not pooled. On host A, sparse ÷ fast was 4.01, 3.79, 8.14 and 8.75 for jv-nagao, gaudry-s4, gaudry-d4 and
dense. The incumbent slows about 3× on B's smaller caches, `fast` about 1.3×. Both hosts clear P1 by a wide margin.

== Decision

P1 and the correctness gate hold, so `fast` becomes the default linear stage:
- `JV_LIN` defaults to `fast`, and the generic entry point `solve_zero_dim` follows it;
- `JV_LIN=sparse` keeps the incumbent.

It leads a new leaderboard component section, "Linear stage after F4 across Gröbner-basis corpora". The class is
engineering, a component speed-up.

*Follow-up.* The one give-up, `p97-4`, is fixed by a random separating linear form instead of $x_n$. That is outside
this protocol.

== Deviations

+ Card L1's first attempt was refused by its disk check (588 MB free, below 1 GB), and is kept under `cards/`.
+ A harness F4-budget bug skipped 11 systems in L3 and L4. Amendment 1 fixed it, and all ten cards were re-run as R2.
+ The host changed after L4. Amendment 2 reports the hosts separately, raises the timeouts to 7000 s, and has the
  main session run the long cards.
+ Conductor was unreachable all session (its hooks let edits through).
+ No `CHECKPOINT_UPLOAD_TOKEN` was set, so the S3 resume checkpoint could not be pushed.
