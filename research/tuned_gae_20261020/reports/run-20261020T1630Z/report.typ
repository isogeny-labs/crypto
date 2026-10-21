#set page(paper: "us-letter", margin: 1.8cm)
#set text(size: 10pt)
#set heading(numbering: "1.")
#show raw: set text(size: 8.5pt)
#set table(stroke: 0.4pt + luma(160), inset: 4pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[Run report: gae-reference sweep and rho.prime_fast sessions] \
  Start 2026-10-20 16:30 UTC (first session start 16:30:46). Observations only; interpretation is in `research/tuned_gae_20261020/README.md`.
]

= What ran

#table(columns: (3.2em, 1fr, 1fr), align: left,
  [*run*], [*command or spec*], [*result*],
  [sweep], [`./target/release/gae-reference --from-calibration` (stdout `sweep/gae_reference.jsonl`, stderr `sweep/gae_reference.stderr`), 16:31:29 to 16:32:15 UTC], [exit 0; 33 rows; stderr empty],
  [field], [`./target/release/gae-reference field --from-calibration` (`sweep/field.jsonl`), from 16:50 UTC, 44 s], [exit 0; 33 rows],
  [tables], [`gae-reference retable`; `gae-reference reprice-ledger`], [`docs/ic/calibration_tuned.json` (25 rows), `full_prices_tuned.json` (8 rows), `reprice/table_b.json` (13 instances; `reproduced_all` true, `mismatches` empty, tolerance 1e-9)],
  [session 1], [`ecbench run`, spec `specs/rho_prime_fast.json`, session ECBS1h3e27cfbb061d, spec ECS1hde00e583dfec, commit d30a63b89 (dirty), 16:30:46 to 16:32:30], [complete; 1,024 of 1,024 records (768 measured, 256 warmup), all verified],
  [session 2], [spec `specs/rho_prime_fast_v2.json`, ECBS1h3da70394827f, same spec id, commit eac4a3dd4 (dirty), 17:44:44 to 17:46:25], [complete; 1,024 of 1,024 (768 measured, 256 warmup), all verified],
  [session 3], [spec `specs/rho_prime_fast_v2_n256.json`, ECBS1h522578c510c4, ECS1ha4176973d735, commit eac4a3dd4 (dirty), 17:30:29 to 17:37:19], [complete; 4,096 of 4,096 (2,048 measured, 2,048 warmup), all verified],
)

Failures: none recorded (every execution status is `verified`; sweep exit codes 0). Not run: any isolated (L2 or higher) session; no A/A control (the specs permit L0).

= Host and isolation

Intel Xeon at 2.80 GHz (signature 6/85/7; avx2, avx512, pclmulqdq), 4 logical CPUs, 1 package, Linux 6.18.44, a cloud VM. ecbench sessions: `--cpus none --allow-busy`, isolation level L0 for every pair (levels L0 x 384, 384, 1,024 pairs; quality score 0). The sweep and session 1 overlap in time (sweep 16:31:29 to 16:32:15; session 1 16:30:46 to 16:32:30) on the same 4-CPU VM; no contention telemetry was recorded for the sweep.

= Arms, workloads, seeds

Arms: `rho-neg` (`rho.negation`, reference) and `rho-fast` (`rho.prime_fast`, candidate), alternating order, 1 warmup, 3 rounds (sessions 1 and 2) or 1 round (session 3). Workloads: four prime-order curves, `icv1-fp28-tm24531-24294e5d`, `icv1-fp32-t93403-ef655380`, `icv1-fp36-t360541-9f646956`, `icv1-fp40-tm62443-8b49d5b3`; 32 targets per curve and 3 rounds (sessions 1 and 2, target seed 20261020, 96 pairs per curve) and 256 targets per curve and 1 round (session 3, target seed 20261021).

= Measured values

#figure(image("../../figures/fig3_rho_prime_fast.svg", width: 100%),
  caption: [Mean S with the harness's 95% interval, and median online solve wall in ms (log scale) per curve. Sessions ECBS1h3e27cfbb061d (rho.prime_fast old config, 32 targets x 3 rounds per curve) and ECBS1h522578c510c4 (tuned config and rho.negation, 256 per curve). Values read from `tables/*.table.md`.])

#table(columns: (auto, auto, auto, auto), align: left,
  [*session*], [*pairs*], [*ops fast / negation (95%)*], [*median wall fast / negation*],
  [ECBS1h3e27cfbb061d], [384], [2.0523 \[1.8974, 2.2265\] cluster], [0.8206 \[0.7825, 0.8551\]],
  [ECBS1h3da70394827f], [384], [1.0972 \[0.9959, 1.2095\] cluster], [0.2241 \[0.2104, 0.2366\]],
  [ECBS1h522578c510c4], [1,024], [1.0271 \[0.9827, 1.0747\] cluster], [0.2175 \[0.2175, 0.2175\]],
)

Units: operations in `ecbench.gae` (lower bound; canonicalisations unpriced on both arms), wall in online solve time. The harness admitted the wall ratio at level L0 for all three. The third session's wall interval is degenerate in the record. The measured mean S per curve are in `tables/` (for example session 3, `rho.negation` 0.930, 0.938, 0.982, 0.898 and `rho.prime_fast` 0.963, 0.972, 0.968, 0.947 for fp28, fp32, fp36, fp40; theory S 0.886).

#figure(image("../../figures/fig1_ns_per_add.svg", width: 100%),
  caption: [Sweep `sweep/gae_reference.jsonl`: ns per affine addition, reference (median of 3 runs, range shown) and tuned (median of 15 passes, min to max shown), 33 rows, 256 lanes, 1,000 iterations per pass. Reference over tuned: prime 3.16 to 4.93, binary 8.68 to 21.69. Every row verified 10,240 additions against the reference; four rows report lane restarts (14, 6, 1 and 14; a restart is a lane that reached the point at infinity, as `calibrate_group` also restarts).])

= Raw records

`research/tuned_gae_20261020/`: `sweep/` (jsonl, stderr, exit codes, `RUN.md`), `sessions/<id>/` (`spec.json`, `plan.json`, `host.json`, `session.json`, `records.jsonl`, `exec/`, `comparisons/rho-neg__rho-fast.json`), `reprice/table_b.json`, `tables/`. Comparison ids: ECC1h1fb816346ca3, ECC1hb4a055725bcc, ECC1hdcae8d76b124.
