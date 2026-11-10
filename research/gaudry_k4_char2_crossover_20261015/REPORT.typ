#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show figure.caption: it => it.body
#show table: set text(size: 8.3pt)

= $k = 4$ in characteristic 2 past one word: IC falls from 3.6× to 0.45× of rho's theoretical cost over $m = 15$–18 (clean sessions, single targets); a crossover is indicated, not established

2026-10-15 to 2026-11-10. `research/gaudry_k4_char2_crossover_20261015/`. Protocol registered (commit `e403626cb`) before any claim run. Stacked on #1883. *Class: engineering* (Parts D and F) and a *comparison against rho on non-standard curves* (Part E).

== What finished

The container restarted many times during the claim runs and each restart kills every run in flight (`ecbench` cannot resume a session). The first attempts, on the registered binary (`c143cf62…`), were killed; they are kept as `*.attemptN-killed` and *no number below uses them*. The remaining runs completed on a second binary (`ad1dd91a…`) that adds checkpoint/resume (a run logs its oracle calls and resumes by replaying the walk against the log; unit tests compare every counter with an uninterrupted run). *Clean, complete sessions:* Part D at $m = 8, 10, 12, 14$ (21/21), Part E at $m = 15$ (6/6), $m = 16$ (4/4), $m = 17$ (IC 2/2 in two sessions, rho 2/2 and 1/1), $m = 18$ IC (1/1), Part F (15/15). All pass the integrity audit (`verify --replay 0`). *Not done:* the $m = 18$ rho run was still running when this was written; Part G (isolated hardware: no isolab credentials in this container); *no session of this study has been replayed from scratch* (hours to days each, against a restart every few hours). The sessions on the second binary rest on the tested equivalence, not on a replay. The $m = 18$ IC session resumed after a filesystem fault (see EXPERIMENTS W33). The two replayed sessions are #1883's, re-executed with the first binary (27/27 reproduced).

== 1. The matrix was the obstacle

In #1883 the dense matrix's `row_ops` grew as $|F|^3$ and would have reached rho's cost near $m = 18$. The `wiedemann` matrix (singleton-filtered core, Wiedemann on $R^T D R$, Berlekamp–Massey, exact check) grows as $|F|^2$:

#table(columns: 6, align: (right, right, right, right, right, right),
  [$m$], [columns], [dense `row_ops`], [sparse `row_ops`], [dense ÷ sparse], [trials, sparse ÷ dense],
  [8], [62], [11,370; 14,252], [69,871; 86,869], [0.16; 0.16], [1.0000],
  [10], [262], [718,813; 536,268], [1,536,771; 1,423,931], [0.47; 0.38], [1.0000],
  [12], [1,045], [32.2 M; 28.0 M], [22.2 M; 21.7 M], [*1.45; 1.29*], [1.0008; 1.0040],
  [14], [4,123], [1.88 B (Part D), 1.91 B (#1883)], [0.356 B (Part D), 0.353 B (Part F)], [*5.29* (paired); 5.4], [1.0006; 1.0003],
)

The sparse solver loses where the matrices are tiny and wins from $m = 12$. Its share of the IC cost is 0.36 %, 0.68 %, 1.36 % and 2.57 % at $m = 15, 16, 17, 18$, doubling per $m$ (5 % at $m = 19$). The $m = 14$ row is now paired (one workload).

#figure(image("figures/la_scaling.svg", width: 62%), caption: [`row_ops` against factor-base columns. `raw/summary.json`.])

== 2. IC against rho, cofactor $h = 2$, $m = 15$–18

#table(columns: 8, align: (right, right, left, right, right, right, right, right),
  [$m$], [$log_2 n$], [target], [IC $S$], [rho $S$], [IC ÷ rho theory (0.886)], [IC ÷ rho run], [matrix share],
  [15], [60], [`W588ac37c7439`], [3.198], [0.168], [3.61], [19.1], [0.36 %],
  [15], [60], [`W761fd6d9429f`], [3.253], [0.856], [3.67], [3.8], [0.36 %],
  [16], [64], [`Wa1b3b10e6968`], [1.482], [0.899], [1.67], [1.65], [0.68 %],
  [16], [64], [`W37a8388076b0`], [1.490], [2.235], [1.68], [0.67], [0.68 %],
  [17], [68], [`Wd5f7dba84d0e`], [0.842], [1.361], [0.95], [0.62], [1.36 %],
  [17], [68], [`W591d11cef5d4`], [0.849], [0.447], [0.96], [1.90], [1.36 %],
  [18], [72], [`We3b266046ebb`], [0.400], [_running_], [*0.45*], [–], [2.57 %],
)

IC used 190–196 trials per relation at every size. The two targets of each size agree within 1 % in IC's cost. The fit of $log_2$(IC ÷ rho theory) against $log_2 n$ has slope $-0.246$ ($-1/4$ expected) and the interpolation crosses 1 between $m = 16$ and 17, at $log_2 n^* approx 67.7$ ($m^* approx 16.9$), against #1883's extrapolated $2^68.1$. *Measured:* IC's cost relative to rho's _theoretical_ cost falls steadily from 3.6× to 0.45× over $m = 15$–18. *Not shown:* that IC beats rho in practice. The $m = 18$ point is one target, individual rho runs scatter about 5× around their expectation (IC ÷ rho run spans 0.62–19 at $m = 15$–17), and the rho half of $m = 18$ is unfinished.

*Open (relayed from an independent red-team review of this study, not re-verified here: its report was said to be on branch `claude/k4-char2-redteam-20261026`, which I could not fetch):* the unit price (corrections of 0.3×–3.1× are reported and unresolved), and every `rho.negation` row is a lower bound (canonicalisations are uncharged, so rho's true cost is higher, which favours IC). The review projects $m^* approx 17.6$ (16.3–19.2) and judges the crossover not established; this study agrees that two targets per size, one at $m = 18$, at an unresolved unit price, do not establish it.

#figure(image("figures/crossover.svg", width: 78%), caption: [IC ÷ rho against $log_2 n$: #1883 (grey, dense matrix, $h = 2$–14), this study's clean-session points (filled), IC ÷ the measured rho run (crosses). `raw/summary.json`.])

== 3. Large primes lose at these sizes (Part F, clean)

$S$ ÷ the same run without large primes: one large prime over 70 % of the columns 1.60, 1.48, 1.54; two large primes over 70 %: 1.13, 1.08, 1.10; over 50 %: 1.51, 1.42, 1.46 ($m = 10, 12, 14$). The matrix they shrink is under 1 % of the cost (its cost falls to 0.52–0.78×); merging and the extra trials cost more. By the protocol's model the variant pays near $n approx 2^92$. A recorded negative at every reachable size.

#figure(image("figures/large_primes.svg", width: 58%), caption: [$S$ ÷ $S$ without large primes. `raw/summary.json`.])

== Predictions and checks

D1 _not met as registered_ (a spec error: the measurement seed differs from #1883's, so the dense arms are fresh samples; the pilot with the seed reused matched exactly, and the first binary replays all 27 committed #1883 runs). D2 (5.29× at $m = 14$, paired), D3, E2 (all four sizes), E4 (2.57 %), F1, F2: held. E3: IC below rho's theory at $m = 18$ (0.45×); the measured-run half is pending. E5: the interpolation gives 16.9 (single targets, unresolved unit price). Frobenius parity: even on 375/375 curves ($m = 4$–18), measured, not proved.

*Scope.* Curves over $FF_(2^(4m))$ are not standard curves. Next: the $m = 18$ rho run, a second $m = 18$ target, and from-scratch replays of the sessions on the second binary.

Environment: Claude Code cloud container, Xeon \@ 2.10 GHz, 4 vCPU, four sessions at a time on separate cores (L1). Conductor task T-4.
