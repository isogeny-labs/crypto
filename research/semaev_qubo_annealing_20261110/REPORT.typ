#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#show raw: set text(size: 8pt)
#let pu = text(size: 7pt)[`[pcc:unverified]`]
= F6: local search and annealing on the m = 2 decomposition landscape

Run F6-RUN-01 of study `semaev_qubo_annealing_20261110`. Class: boundary diagnostic; no S, no rho row, no leaderboard change. All measured numbers below are #pu (no pcc claim id exists for this run yet).

== Question
Does any local search or annealing method find a planted decomposition R = P₁ + P₂ with x(P_i) ≤ B in fewer cost evaluations than the exact baselines (box enumeration Θ(B²), root-solving enumeration Θ(B)), with an exponent in B below theirs? (Moonshot G1.)

== Mechanism
State (x₁, x₂) ∈ [1, B]²; cost c = min(v, p − v), v = S₃(x₁, x₂, x_R) mod p; c = 0 iff a decomposition exists at those abscissae (checked by lifting and adding). Moves: bit flips of a Gray-coded coordinate, ±1 steps, random restart. If S₃ mod p is pseudo-random off the solution set there is no usable gradient and local search reduces to sampling.

== Method
p = largest prime ≡ 3 (mod 4) below 2ⁿ, n ∈ {12, 16, 20, 24, 28}, one prime-order curve per n; B = ⌈√p⌉; 20 planted instances per n (seed `F6/inst/<n>/<i>`). Solvers: uniform random sampling (control), greedy with restarts, simulated annealing at three schedules (sa_B, sa_4B, sa_16B), parallel tempering (8 replicas); baseline root_enum counts x₁ values tried. Metric: cost evaluations to the first verified c = 0, cap min(50·B², 2·10⁹) (§6 amendment). Exponent: ln-ln OLS of the median on B, bootstrap 95% CI over instances (1000 reps), over all n and over n ≥ 20. Landscape: lag-1..8 autocorrelation along bit-flip walks vs a random landscape with the same marginal, n ≤ 20. Native Rust tool, local host (4 vCPU), counts only.

== Results #pu
Median evaluations over 20 instances (censored count in brackets); p, B, B², mean box solutions per n.

#include "figures/table.typ"

#figure(image("figures/evals_vs_B.svg", width: 100%),
  caption: [Left: median cost evaluations to first c = 0 vs B (log–log) per solver, with B² and B reference lines and the measured root_enum; hollow markers: at least one of 20 instances censored (n = 28: sa_B, sa_4B, pt, 2/20 each). Right: fitted exponent e with bootstrap 95% CI per solver, all n (filled) and n ≥ 20 (hollow), with e = 1 (root-solving), e = 2 (box) and the P1 threshold 1.8. Run F6-RUN-01, 20 instances per n, local host. Source: `results/run.json`; `figures/make_figure.py`.])

Exponents: uniform 1.948 (n ≥ 20: 1.929); greedy 1.820 (1.851); sa_B 2.197 (2.095); sa_4B 2.192 (2.199); sa_16B 2.058 (2.171); pt 2.124 (2.179); root_enum 0.901 (0.879). Autocorrelation at lag 1 is in band at n = 12, 16, 20 (walk means −0.0006, +0.0003, −0.0012; bands about ±0.094); the lag-2 mean excess over the random landscape is recorded (advisory z 49.6, 36.6, 26.3) and not scored. Costs: 0 verification failures, 0 instrument errors, 0 spurious zero-cost states. #pu

== Reading (PROTOCOL.md §4)
+ Instrument failure: not triggered. P3: uniform e = 1.948, within 2 ± 0.25; 0 verification failures; 0 instrument errors. #pu
+ Lead alive: not met. Smallest n ≥ 20 upper CI over the six solvers is 1.614 (greedy), none below 1.0; P2 in band at lag 1 at all three sizes (0 failing sizes, rule needs ≥ 3). #pu
+ Rule 3 applies, verbatim: "Closed: otherwise: no local-search method beats root-solving enumeration on the `m = 2` decomposition landscape at these sizes." #pu

P1 (e ≥ 1.8 for every solver): held under the protocol's wording, by point estimates; greedy is the margin (1.820 all n, 1.851 n ≥ 20); its CIs [1.668, 2.003] and [1.614, 2.105] contain 1.8 and the protocol froze no CI test. P2: held under the in-band convention at all 8 lags, with the lag-2 and lag-4 advisory z noted above. P3: held (|1.948 − 2| = 0.052 ≤ 0.25). No solver reached e ≤ 1 (prior 0.005). #pu

== Cap note
The §6 amendment sets the cap to min(50·B², 2·10⁹); the tool applies it (every censored row records 2,000,000,000). The stdout header ("cap 50 B^2"), `header.eval_cap` and `summary.baselines_per_n[].cap_50B2` (13,421,772,800 at n = 28) still carry the unamended label, so at n = 28 the labelled cap differs from the applied one; for n ≤ 24 they coincide. Six of 140 n = 28 rows are censored: sa_B, sa_4B, pt, two instances each; the medians there are exact (fewer than half censored). #pu

== Scope
m = 2 only, these solvers only, n ≤ 28 (landscape n ≤ 20); no QUBO hardware. The §9a isolab-eks confirmatory run is open; the host was shared; no timing is a result. The tool ran with 4 threads (no `--threads` flag); results are seeded per instance and solver, so by the tool's design they do not depend on the thread count (an assertion of design, not measured). The research-kb server was unavailable; no novelty claim is made.

== Provenance
#table(columns: (auto, 1fr), stroke: 0.4pt, inset: 3pt,
  [Card], [`CARD-semaev-qubo-annealing-20261110-F6-RUN-01`, sha256 `2ba610affa718256691e81900195fff1b7c6a9859b44ef8c6a8e7a2d72bd9ece`],
  [Commit], [`8c3b356ff45ec268ea7d0a21f28050200aa4c515` (8c3b356ff4)],
  [Receipt], [sha256 `7fce30d49275716076fa84a840441a24e896bd4d95059491388714e481fd1920`; status completed, 13 commands; tree dirty before and after (other studies, card outputs)],
  [Binary], [sha256 `07371fffa1bb890de18d3d000ee5418987499e01bc78e9526f96a6653a3d9cae` (receipt); the card's pre-build `binds` entry lists `c70c8310f6752cd64f326c6feb8aa625f5b9a556e6b0ea93a535a250c187ba9f`, not reconciled],
  [run.json], [sha256 `9bbea2ea5dd017b85461ce9fc619cf99768bcdc2cdfb0a0a9e58586d786b3cdd`],
  [FREEZE.sha256], [PROTOCOL.md `7c427397febd48f48ba9941152997628ab6f0ec308fcb0dc7d550c12222387eb`; tool/src/main.rs `88d7dd340c8b48164696db4d0a670537e1e574c3d32938f3d0c8fbd540dad61f`; tool/Cargo.toml `69fa9a7ee61e4e8814462bb57765f56295802a455d1795a631936c702fc8f61f`; tool/Cargo.lock `a1e70b445783b45e612a8ab1c862018b98769babefefb8c293684f7ec1c94aea`],
  [Report build], [typst Python wheel 0.15.2 (scratch venv); figure by matplotlib 3.11.2],
)
