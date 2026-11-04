#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show figure.caption: it => it.body
#show table: set text(size: 8.3pt)

= $k = 4$ in characteristic 2 past one word: a sparse matrix and $u 128$ orders; IC is 3.6× rho at $m = 15$ (clean sessions); the $m = 16$–17 records are from interrupted sessions and $m = 18$ did not complete

2026-10-15 to 2026-10-26. `research/gaudry_k4_char2_crossover_20261015/`. Protocol registered (commit `e403626cb`) before any claim run; binary sha256 `c143cf62…` for every claim session. Stacked on #1883. *Class: engineering* (Parts D and F) and a *comparison against rho on non-standard curves* (Part E).

== What finished

The container restarted repeatedly during the claim runs and each restart kills every run in flight (`ecbench` cannot resume a session). *Clean sessions:* Part D at $m = 8, 10, 12$ (18/18), Part E at $m = 15$ (6/6), the rho half of $m = 17$ (2/2), Part F (15/15). *Interrupted sessions:* $m = 16$ (3 of 4 runs), the IC half of $m = 17$ (1 of 2), Part D at $m = 14$ (2 of 3). Their audit passes `records`, `records_hash`, `plan_hash`, `spec_id`, `workloads`, `env_class` and `host_hash` and fails exactly `session_complete` and `record_count`; they were not replayed, so everything resting on them is _indicated_. *Not completed:* $m = 18$ (no attempt wrote a record). Part G (isolated hardware) was not run: no isolab credentials in this container. *None of this study's own sessions has been replayed* (the replay audits were stopped for lack of time, see the README's audit table); the integrity audits of the four complete sessions pass. The two replayed sessions are #1883's, re-executed with this binary (27/27 reproduced).

== 1. The matrix was the obstacle

In #1883 the dense matrix's `row_ops` grew as $|F|^3$ and would have reached rho's cost near $m = 18$. The `wiedemann` matrix (singleton-filtered core, Wiedemann on $R^T D R$, Berlekamp–Massey, exact check) grows as $|F|^2$:

#table(columns: 6, align: (right, right, right, right, right, right),
  [$m$], [columns], [dense `row_ops`], [sparse `row_ops`], [dense ÷ sparse], [trials, sparse ÷ dense],
  [8], [62], [11,370; 14,252], [69,871; 86,869], [0.16; 0.16], [1.0000],
  [10], [262], [718,813; 536,268], [1,536,771; 1,423,931], [0.47; 0.38], [1.0000],
  [12], [1,045], [32.2 M; 28.0 M], [22.2 M; 21.7 M], [*1.45; 1.29*], [1.0008; 1.0040],
  [14], [4,123], [1.88 B (interrupted), 1.91 B (#1883)], [0.353 B (Part F)], [*5.3–5.4*], [1.0075; 1.0003],
)

The sparse solver loses where the matrices are tiny and wins from $m = 12$. At $m = 17$ it is 1.4 % of the IC cost (interrupted session); it grows as $|F|$ relative to the relations, reaching 5 % near $m = 19$. The $m = 14$ row is unpaired (different seeds).

#figure(image("figures/la_scaling.svg", width: 62%), caption: [`row_ops` against factor-base columns. `raw/summary.json`.])

== 2. IC against rho, cofactor $h = 2$, $m = 15$–17

#table(columns: 7, align: (right, right, left, right, right, right, left),
  [$m$], [$log_2 n$], [target], [IC $S$], [rho $S$], [IC ÷ rho theory (0.886)], [session],
  [15], [60], [`W588ac37c7439`], [3.198], [0.168], [*3.61*], [clean],
  [15], [60], [`W761fd6d9429f`], [3.253], [0.856], [*3.67*], [clean],
  [16], [64], [`Wa1b3b10e6968`], [1.482], [0.899], [_1.67_], [interrupted],
  [17], [68], [`Wd5f7dba84d0e`], [0.849], [1.361], [_0.96_], [IC interrupted, rho clean],
)

At every size IC used 191–196 trials per relation. *Established:* IC is 3.6× rho's theoretical cost at $m = 15$. *Indicated* (one target each, interrupted sessions): 1.67× at $m = 16$ and 0.96× at $m = 17$; the fit through the three points has slope $-0.24$ ($-1/4$ expected) and crosses 1 at $log_2 n^* approx 67.7$ ($m^* approx 16.9$), against #1883's extrapolated $2^68.1$. *Not shown:* that IC beats rho. One target at $m = 17$ from an incomplete session, at 0.96× rho's _theoretical_ cost and inside rho's run-to-run spread, is not that; the registered claim (E3) was at $m = 18$, which did not run.

#figure(image("figures/crossover.svg", width: 78%), caption: [IC ÷ rho against $log_2 n$: #1883 (grey, dense matrix, $h = 2$–14), clean-session points (filled), interrupted-session points (hollow), IC ÷ the measured rho run (crosses). `raw/summary.json`.])

== 3. Large primes lose at these sizes (Part F, clean)

$S$ ÷ the same run without large primes: one large prime over 70 % of the columns 1.60, 1.48, 1.54; two large primes over 70 %: 1.13, 1.08, 1.10; over 50 %: 1.51, 1.42, 1.46 ($m = 10, 12, 14$). The matrix they shrink is under 1 % of the cost (its cost falls to 0.52–0.78×); merging and the extra trials cost more. By the protocol's model the variant pays near $n approx 2^92$. A recorded negative at every reachable size.

#figure(image("figures/large_primes.svg", width: 58%), caption: [$S$ ÷ $S$ without large primes. `raw/summary.json`.])

== Predictions and checks

D1 _not met as registered_ (a spec error: the measurement seed differs from #1883's, so the dense arms are fresh samples; the pilot with the seed reused matched exactly, and this binary replays all 27 committed #1883 runs, `audits/reaudit-*`). D2, D3, F1, F2: held. E2: held at $m = 15$, indicated at 16 and 17. E5: indicated (16.9). E3, E4: not tested ($m = 18$). Frobenius parity: even on 375/375 curves ($m = 4$–18, 25 each), measured, not proved.

*Scope.* Curves over $FF_(2^(4m))$ are not standard curves. The established result is a trend through $m = 15$; the crossover and the $m = 18$ comparison are open. Re-running `run_claims.sh` on a host that does not restart completes them (about 15 hours for $m = 17$ IC).

Environment: Claude Code cloud container, Xeon \@ 2.10 GHz, 4 vCPU, four sessions at a time on separate cores (L1). Conductor task T-4.
