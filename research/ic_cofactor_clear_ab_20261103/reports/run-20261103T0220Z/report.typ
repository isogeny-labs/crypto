#set page(margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set table(stroke: 0.4pt)

= Run report: cofactor clearing A/B (observations only)

Study `research/ic_cofactor_clear_ab_20261103`. Run started 2026-11-03T02:20Z (UTC) on the local cloud VM. This report states what ran and what the harness recorded; it makes no claim about what the numbers mean (see the study README for that). Every number is labelled *measured* (read from an ecbench record or an `ecbench compare` output) or *derived* (computed from measured values, formula given).

== What ran

#table(columns: (auto, 1fr, 1fr), inset: 4pt,
  [], [V2 treatment session], [h=1 companion session],
  [spec], [`specs/v2-treatment.json`], [`specs/h1-negative-control.json`],
  [ecbench session id], [`ECBS1hd7774432e6f8`], [`ECBS1ha02d58b9932d`],
  [curves], [14 census V2 curves, p=42310123, group order 42313696, r=1322303, h=32], [14 prime-order 26-bit curves (prime_search seeds 20261200..20261213), h=1],
  [targets], [8 planted_shared per curve, target seed 20261103], [8 per curve, target seed 20261104],
  [arms], [rho-neg (reference), rho-plain, ic-raw, ic-clear, ic-raw-aa], [same five],
  [rounds], [4 measured + 1 warm-up], [4 measured + 1 warm-up],
  [executions], [2800 requested, 2800 recorded, 2800 verified], [2800 requested, 2800 recorded, 2800 verified],
  [wall time of `run`], [116.894 s], [193.763 s],
  [audit], [`verify --replay-all`: 2240 replays reproduced, 0 problems], [same: 2240 reproduced, 0 problems],
)

Commit recorded by both sessions: `11173ecd0230a9ceedc4d833736402934fdcb95e`, git_dirty false. Binary sha256 `b38cffb4f64c4ec7289ee4ef19bf60560d2d99684d4f996114eb403560081ec4`. Command: card `cards/CARD-ic-cofactor-clear-ab-20261103-2.yaml` (receipt sha256 `068b15fa...0dceffeeff4`) running `ecbench run` then `ecbench verify --replay-all` for each spec. A first attempt (card 1) was refused by ecbench before any measurement: `host is not quiet (PSI cpu some avg10 5.23 > 5.00)`; it produced no session. Its receipt is in `cards/attempt1-host-busy/`.

Arms `ic-raw` and `ic-clear` are `ic.pipeline` with `oracle=mitm:negation_folded=1`; they differ only in `factor_base`: `prime-abscissa:size=64` against `prime-projected-abscissa:size=64,scan_cap=4096`.

== Host and isolation

Shared KVM VM, 4 vCPU Intel Xeon 2.1 GHz, no readable hardware PMU, other processes present. ecbench graded each record individually; as printed in the run logs, V2 had 1121 records at L1 and 1679 at L2, the h=1 session 511 at L1 and 2289 at L2 (counted by the assessor from the run output). No wall-clock figure is used here; all values below are counted S (group operations over the session floor), which does not depend on isolation.

== Measured values

Ratios are `ecbench compare` ratios of totals B/A over matched (workload, round) pairs, 448 pairs on 112 workloads per session; the interval is ecbench's primary linearised 95% interval (workloads as clusters), the percentile bootstrap beside it is in the JSON. Weight w prices every unpriced native operation at w GAE on both arms.

#table(columns: (auto, auto, auto, auto), inset: 4pt,
  [comparison (B / A)], [w=0 GAE], [w=0.1 GAE], [w=1 GAE],
  [V2: ic-raw / ic-clear (measured)], [6.180 [5.915, 6.456]], [6.692 [6.403, 6.995]], [9.087 [8.674, 9.520]],
  [h=1: ic-raw / ic-clear (measured)], [0.989 [0.989, 0.989]], [0.991 [0.990, 0.991]], [0.996 [0.996, 0.996]],
  [V2: ic-raw / rho-neg (measured)], [60.57 [56.69, 64.71]], [65.85 [61.60, 70.39]], [91.06 [84.96, 97.60]],
  [V2: ic-clear / rho-neg (measured)], [9.801 [9.329, 10.297]], [9.839 [9.357, 10.346]], [10.021 [9.490, 10.581]],
  [h=1: ic-raw / rho-neg (measured)], [29.97 [28.17, 31.88]], [32.69 [30.71, 34.78]], [45.23 [42.46, 48.19]],
  [h=1: ic-clear / rho-neg (measured)], [30.31 [28.49, 32.23]], [33.00 [31.01, 35.10]], [45.43 [42.64, 48.39]],
)

Mean S per arm at w=0 (measured, 448 measured records each): V2 ic-raw 76.95, ic-clear 12.45, rho-neg 1.27, rho-plain 1.56; h=1 ic-raw 31.67, ic-clear 32.03, rho-neg 1.06, rho-plain 1.39.

Controls (measured, w=0): A/A ic-raw-aa / ic-raw = 1.000 [1.000, 1.000] in both sessions (the A/A arm's records are bit-identical to its twin's, because arms of one (workload, round) share an algorithm seed). Positive control rho-plain / rho-neg: V2 1.227 [1.149, 1.311]; h=1 1.313 [1.231, 1.401].

Mechanism counters (derived: total `trials` divided by total `relations` over the 448 measured records of an arm, from `phases[relations].native`): V2 ic-raw 5433.2 trials per relation (36934665 trials, 6798 relations), ic-clear 161.8 (2775477 trials, 17158 relations); h=1 ic-raw and ic-clear both 5602.8 (94071172 trials, 16790 relations).

#figure(image("fig1.svg", width: 100%),
  caption: [Recorded ratios with ecbench's linearised 95% intervals. Left: gain of the cleared base against the raw base at three prices of unpriced native work, on the V2 curves and on the h=1 control. Middle: the A/A and positive controls. Right: the V2 arms against the matched rho-neg. Sessions ECBS1hd7774432e6f8 and ECBS1ha02d58b9932d, 448 matched pairs each. Source data: `figure-data.json`; script: `figures.py`.])

== Missing or not done

No timing result is reported (L1 host). No second-host replay certificate, validator report or matched-rho `VALIDATION.json` exists for these sessions. The executor's returned text was not checked against the receipt with `task_card.py check-relay`.

== Raw records

`sessions/v2/` and `sessions/h1/` (`records.jsonl`, `session.json`, `host.json`, `plan.json`, `spec.json`, `analysis/`), exports in `exports/`, frozen protocols `protocol-v2.json` and `protocol-h1-control.json`.
