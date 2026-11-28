#set page(margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show raw: set text(size: 8pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[PS-6 run report: five counted ecbench sessions, S4 baseline, group-law kernel, MITM] \
  #text(size: 10pt)[`research/pdp_oracle_speedup_20261011/ps6_charge/raw/sessions/counted_n*` --- 2026-10-18, 23:27--23:44 UTC]
]

= What ran

`ps6_charge/run_ps6.sh` (stage `counted`), specs `ps6_charge/specs/counted_n15 .. n27.json` (written and committed before any session ran), `ecbench run --wait --cpus none`, then `ecbench verify` (`--replay-all` at $n <= 21$, `--replay 12` at $n = 24, 27$). Binary SHA-256 `c914f6328634...`, commit `9e7e9a8a5` (clean tree, `raw/build_env_premerge.txt`). Host: Intel Xeon 2.10 GHz KVM guest, 4 vCPU, kernel 6.18.44-fc-v139; isolation level L0 on every run (operation counts need none). Curves: the boundary ledger's random binary curves, `random_binary_instance(n, 0x1CB00BDA7A, 8)`; factor base `binary-subspace:dimension=ceil(n/3)`; three summands; random targets; one target per run.

*Arms.* `rho-negation` (matched reference), `s4-baseline`, `s4-grouplaw`, `s4-grouplaw-audit` (the kernel with every target also decided by the baseline and the witnesses compared), `mitm`, `mitm-negfold`.

*Runs requested and completed.* 96 + 96 + 96 + 96 + 48 = 432 requested, 432 completed, 432 verified, 0 failed, 0 timed out, 0 interrupted. Audits (`raw/sessions/counted_n*.audit.json`): all `ok`, 0 problems. Sessions `ECBS1h35288032d470` (n = 15), `ECBS1h9c1f6e592bd2` (18), `ECBS1h513dba00a329` (21), `ECBS1h08f67a4fa7eb` (24), `ECBS1ha751ceb1f30b` (27).

= Observations

#figure(
  image("s_over_floor.svg", width: 94%),
  caption: [`S / floor` per rung and arm from the five counted sessions (mean over targets; 95% bootstrap intervals over targets; shaded: unpriced operations priced at 0 to 1 GAE each). Source `ps6_charge/raw/summary.json`.],
)

#table(
  columns: (auto, auto, auto, auto, auto, auto, auto, auto),
  align: (right,) * 8,
  table.header[*n*][*runs per arm*][*rho*][*baseline*][*kernel (recorded, lower bound)*][*kernel = audit arm*][*MITM*][*MITM neg.-folded*],
  [15], [16], [2.10], [1,270], [33.75], [33.75], [57.89], [56.13],
  [18], [16], [1.97], [3,041], [26.85], [26.85], [65.87], [60.21],
  [21], [16], [1.31], [8,560], [22.90], [22.90], [73.56], [69.93],
  [24], [16], [1.29], [19,067], [16.28], [16.28], [93.85], [80.89],
  [27], [8], [1.09], [114,574], [22.38], [22.38], [197.0], [182.4],
)

Mean `S`, recorded values (every arm at weight 0 for its unpriced operations; the kernel's pairs are not in it). Rho, the baseline and the kernel list unpriced counters (`canonicalisations_uncharged`; `s4_oracle_*_uncharged`; the kernel's pairs, one inversion per target, its base list) and are marked lower bounds; the MITM runs list none. The kernel's and its audit arm's recorded `S` are equal on every run, as they must be: the audit's baseline work is counted apart and not charged.

*Verdict parity, from the records.* In all 72 (workload, round) groups the baseline, kernel and audit arms tried the same number of targets, found the same number of relations, reached the same matrix rank and recovered the same scalar. The audit arm compared the kernel's witness with the baseline's on 27,938 non-degenerate targets and found 0 mismatches; 4 degenerate targets across the sessions were decided by the baseline inside the kernel's call.

*Replays.* The 2026-10-31 binary (commit `7ded1ca84`, the base branch merged since) replayed every run of each session at $n <= 24$ and 12 runs at $n = 27$: all reproduced exactly (`raw/sessions/counted_n*.audit_postmerge.json`; receipt digests in the study README). Failures: none.

= Not in this report

No wall-time figure (L0 sessions). No pricing of the kernel's pairs beyond the 0 / 0.1 / 1 GAE band; the results report and the study README carry the comparison, the exponents and their limits.
