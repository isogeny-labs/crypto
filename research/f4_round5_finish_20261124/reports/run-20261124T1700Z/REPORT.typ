#set page(paper: "a4", margin: (x: 1.6cm, y: 1.6cm))
#set text(size: 9.5pt)
#show heading.where(level: 1): set text(size: 14pt)
#show heading.where(level: 2): set text(size: 11pt)

= F4 leads finished: branch traces, deterministic traces (run-20261124T1700Z)

*Standing:* `[pcc:unverified]`. One shared 4-vCPU VM, isolation L0, no PMU.

*Protocol:* `CONFIRM_PROTOCOL.md`, frozen and pushed (`ea2c575bdd`) before data. *PR:* \#5144. Follow-up to \#4859.

== Result

#table(
  columns: (auto, auto, auto, auto),
  align: (left, left, left, left),
  table.header([*check (10 rounds × 3 arms)*], [*predicted*], [*measured*], [*outcome*]),
  [Counted cost identical within each arm], [yes], [B1/B1a 7,003,932,907 in all 20 runs; B0 7,302,797,284 in all 10], [met: deterministic],
  [Counted S, B0/B1], [1.042, exact], [1.042; 2,531 diverged replays continued on a branch], [met],
  [Wall, B0/B1], [[0.97, 1.05], a null expected], [1.018, 8/10; CPU 1.040], [met; below the A/A deviation],
  [A/A, B1/B1a], [±3 %], [1.005, median deviation 2.5 %], [met],
)

All 30 runs found 31 relations from 19,020 residuals and a correct logarithm. `cargo test --release --lib f4_fp`: 142 passed, including `diverged_replays_record_and_follow_branches`.

#figure(image("confirm.svg", width: 100%), caption: [Per-round wall times, and counted S against the warm-up length; `data/confirm/runs.jsonl`, `data/pilot_runs.jsonl`.])

== What changed

- *Branch traces.*
  - A replay that diverges at step k with new leading monomials L continues on a branch recorded under (k, L).
  - Without such a branch, it records its continuation as one. A branch is kept only from a run that ends cleanly under its rule.
  - At most 8 branches per step. `F4_FP_BRANCH=0` turns this off.
- *Deterministic trace growth.*
  - Traced `jv_cover` DLP runs grow the trace tree only during a sequential warm-up of the first 256 residuals (`JV_TRACE_WARMUP`).
  - The parallel phase is read-only. Before this change, counted cost varied by up to 0.9 % with thread scheduling.

== What was wrong in the estimate

Round 4 estimated that diverged replays carried about 47 % of counted F4 cost, and that branches could save about 30 %. The measured saving is 4.1–4.3 %, even with the whole run as warm-up (51 branches).

The estimate assumed each diverged replay restarts F4 from scratch. In fact it continues from the state where it diverged.

== Leads closed without a change

- *Lazy positions in the frozen-step rebuild:* +0.7 % F4 instructions (rebuild 62.7 → 68.1 M) with no gain in hits. Reverted.
- *AVX2 divisor scan, packed pair data:* estimated at under 1 % of F4 each after round 4, which is below this host's wall-time noise floor (2–3 %). Not built.

== Limits

One host; p = 53. The VM ran about 30 % slower than on 2026-11-21, so wall times are compared only within this run. The counted-S gain is small and deterministic; the wall gain is not significant.
