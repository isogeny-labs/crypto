#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show table: set text(size: 8.5pt)

= Wallfix PRs on RunPod: session-level A/B with an A/A control

*Run:* `run-20261114T2240Z` (run 2, distinct binaries) and `run-20261114T2002Z` (run 1, kept as the A/A control). *Standing:* `[pcc:unverified]`, RunPod tier (shared host, below isolab L2). *Study:* `research/wallfix_runpod_20261114/`. *Record:* `benchmarks/records/20261114T1934Z-wallfix-runpod-abab.json`.

== Outcome and checks

- *Identical outputs.* After dropping only timing, host, id and binary-hash fields (`docs/perf/20261110-wall-only-rust-fixes/norm-records.sh`), the A and B records of all 18 (PR, spec, round) pairs are identical, and identical across rounds (`data/identity.txt`). Counted cost (GAE), counters, recovered logarithms and outcomes do not change: these PRs change wall time only.
- *All runs verified.* 36/36 sessions `rc=0`; 36/36 `ecbench verify --replay-all` on the pod `rc=0` (a self-audit: same binary, same host); every measured run replayed again on a second host (Intel Xeon 2.10 GHz container) with the producing binary, `verify-local/`, summarised in `replay-certificate.json`.
- *Binaries distinct.* Six binaries, six SHA-256 (`data/binaries-run2.sha256`), each build compiled the `crypto` crate (≈190–200 s each).
- *Measured (derived ratio of recorded values).* Session-total solve CPU time, merge (B) over first parent (A), geometric mean of 3 counterbalanced rounds, with the per-round range:

#table(columns: 6, align: (left, left, right, right, right, left),
  [*PR*], [*spec*], [*B/A*], [*range (3 rounds)*], [*A/A spread run 2*], [*dominant arm (share), arm B/A range*],
  [\#4009], [tensor], [*0.654*], [0.645–0.664], [1.012], [ic-tensor (100%) 0.645–0.664],
  [\#3850], [prime], [*0.905*], [0.905–0.905], [1.001], [ic-rf-pairs (94%) 0.901–0.902],
  [\#4009], [koblitz], [*0.926*], [0.925–0.927], [1.015], [da-f4 (93%) 0.919–0.923],
  [\#3850], [tensor], [0.964], [0.958–0.968], [1.025], [ic-tensor (100%) 0.958–0.968],
  [\#3850], [koblitz], [0.977], [0.973–0.981], [1.001], [rf-pairs (8%) 0.73–0.80; da-f4 (92%) 0.993–0.996],
  [\#3991], [prime], [*1.013*], [1.011–1.015], [1.004], [ic-rf-pairs (94%) 1.011–1.014],
)

A/A spread is max/min of the three A-arm session totals in run 2. Solve wall time gives the same ratios to within 0.003 (`data/results-ab.json`).

#figure(image("ab-ratios.svg", width: 100%),
  caption: [Paired B/A of session-total solve CPU time per round (blue), geometric mean (red), and the same statistic between byte-identical binaries in run 1 (grey band, A/A control). 20 measured runs per ecbench arm per session; pod `bdr-wallfix`, AMD EPYC 9575F shared host.])

== Reading

- \#4009 (GPU/F4 fixes in `pq_f4_f2.rs`, `pq_groebner_f2.rs`) makes the `ic-tensor` solve *35% faster* (0.654) and `da-f4` 8% faster, far outside every A/A range.
- \#3850 makes `ic-rf-pairs` *10% faster* on the prime spec (0.905, all three rounds 0.905), and `ic-tensor` 3.6% faster.
- \#3991 makes the prime spec *1.3% slower* (1.011–1.015 in all three rounds, A/A 1.004; run-1 control 1.001–1.003). Small, consistent, on `ic-rf-pairs`; a candidate regression to profile, not yet attributed.
- The run-1 A/A band for tensor is wide (up to 1.43) because run 1 ran at load 12–44 on the shared host; run 2 ran at load 4–22 and its own A/A spread is 1.2–2.5%. \#3850 tensor (0.964) is therefore resolved only against run 2's own A spread (1.025), not against run 1's band; the other five rows lie outside both.
- *Scope: the solve interval.* Whole-session wall time from `runlog-run2.tsv` (process start-up, workload set-up, every arm and warmup) gives #4009 tensor 0.657, #3850 prime 0.910, #3850 tensor 0.966, but leaves the short specs unresolved: #3850 koblitz 1.000 (rounds 0.886–1.128), #4009 koblitz 1.008 (0.975–1.063), #3991 prime 1.024 (0.991–1.067). The koblitz sessions last about 5 s, mostly outside the solves.
- *Per-arm ratios.* `time.schedstat_solve.cpu_ns` advances on scheduler ticks, so solves under about 1 ms read 0 (2,131 of 4,320 measured records, 0.0115% of solve wall time). Per-arm ratios are given only for arms whose every round has a nonzero sum; the rho arms on these small curves have none. One arm carries 92.5–100% of each session sum; on #3850 koblitz the 0.977 comes mostly from `rf-pairs` (0.769), `da-f4` alone being 0.995.
- *Order.* Counterbalanced; an order-balanced estimate differs from the three-round geometric mean by at most 0.4% (#4009 tensor 0.656 vs 0.654).
- *Independent recompute.* `validator/validator-report.json` (status pass): all 12 headline geometric means recomputed from the raw records with separate code, output identity, outcomes and binary identity re-checked.

== Cross-method band (AGENTS.md §5a)

Each session runs a rho reference arm (`rho.negation` on prime curves, `rho-strong` on Koblitz) beside the IC arms. Mean S at w = 0, 0.1, 1 GAE per unpriced native operation (`band/`), the same for A and B:

#table(columns: 4, align: (left, left, right, right),
  [*curve*], [*arm*], [*S (w = 0 / 0.1 / 1)*], [*verdict*],
  [icv1-fp24-t1059], [ic-rf-pairs], [7151.5 / 7839.1 / 14027.6], [rho < IC across the band],
  [icv1-fp24-t1059], [rho-neg], [1.052 / 1.148 / 2.013], [],
  [icv1-f2m17-tm101], [da-f4], [11.33 / 200.7 / 1904.8], [rho < IC across the band],
  [icv1-f2m17-tm101], [rho-strong], [4.16 / 4.33 / 5.84], [],
  [icv1-fp14-tm143], [ic-tensor], [269098 / 323103 / 809149], [rho < IC across the band],
  [icv1-fp14-tm143], [rho-neg], [2.239 / 2.401 / 3.858], [],
)

The PRs change wall time, not counted cost, so the distance to matched rho in S is unchanged by them.

== Method

- *Arms.* A = first parent, B = merge commit: \#3850 `5382b439bc`/`43e68a1d56`; \#3991 `6745ba0a5f`/`94c682e021`; \#4009 `2c1dc30e16`/`21f21a45a3`.
- *Source and build.* `mksrc.sh` (`git archive` of Cargo.toml, referenced paths and `include_str!` files; tarball SHA-256 in `tarballs.txt`), `build.sh` on the pod, rustc 1.100.0, `--locked`, one shared target dir.
- *Specs.* `specs/{prime,koblitz,tensor}.json` (the director's isolab drafts, isolation L0); spec ids `ECS1hba9d4455caee` (prime), `ECS1hd60887529f89` (koblitz), `ECS1hb4d9aaeee574` (tensor).
- *Schedule.* `run.sh`: 3 rounds, every (PR, spec) pair interleaved inside each round, odd rounds A first, even rounds B first; `RAYON_NUM_THREADS=1`, `--cpus inherit --allow-busy`.
- *Host.* RunPod CPU pod (cpu5c, 8 vCPU) on an AMD EPYC 9575F 128-thread shared host, kernel 6.8.0-147, governor schedutil, SMT on, no PMU access. Load average 4–22 in run 2.
- *Metric.* Sum over measured (non-warmup) records of `time.schedstat_solve.cpu_ns` per session; solve wall time as a cross-check. Ratios and geometric means are derived from recorded values (`analysis/analyze.py`).

== Run 1: why it is only an A/A control

Run 1 (36 sessions, all verified) used byte-identical binaries for A and B: builds 2–6 took 0–1 s because `git archive` stamps files with the commit time, older than the artifacts in the shared target dir, so cargo's mtime fingerprint reused the first binary. `build.sh` now touches the unpacked tree and refuses a build log without `Compiling crypto`. Run 1's data stays as the A/A control (`sessions/run1-aa/`, `data/results-aa.json`): prime A/A paired geometric means 1.002, koblitz 1.001–1.004, tensor 1.025–1.116.

== Limits

RunPod shared host: no core isolation, no frequency control, other tenants. This is not isolab L2 evidence; isolab-eks was unavailable (monthly budget exceeded). Wall-time ratios at this tier are admitted for ranking these binaries on this host only.
