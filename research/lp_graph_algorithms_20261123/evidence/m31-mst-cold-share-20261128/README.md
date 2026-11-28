# MST filter stage as a share of a cold M31 solve (2026-11-28)

[pcc:unverified]

## Which cold solve

The M31 stream used so far (#5076, 613 rows) is the ten-query prefix of the
producer's graph-control run. It does not recover the discrete log
(target rank 452 of 509). The same producer
(`research/prime31_graph_ecdlp_20261123/native/mitm_four.rs` at
`577d6c2c`, binary `prime31-graph-mitm-control 40`) is a complete
single-target cold solver:
- It builds a fixture and a signed pair table, then collects four-sum relations query by query.
- It keeps an incremental graph/rank state and stops when the target scalar is recovered.
- Its recorded 40-cap run (`evidence/mitm-b677-control-01/forty-cap.json`) recovers `987654321` after 11 queries, with replay.

I rebuilt that solver at `577d6c2c` here: 81 files exported with
`git archive`, plus one file its build script reads,
`cargo build --locked --release`. I ran it cold five times, each as a
separate process. Every run reproduces the recorded run's 707 relation rows,
recovered scalar and charged group work exactly (`cold-solve-runs.json`).

| stage of the cold solve (median of 5, this host) | ms | share |
|---|---:|---:|
| fixture | 1 | 0.0 % |
| signed pair table | 531 | 10.4 % |
| relation collection, 11 queries | 4,516 | 88.8 % |
| producer's own graph + rank + replay | 25 | 0.5 % |
| **cold wall** | **5,084** (5,035–5,209) | 100 % |

## MST stage on the solve's own stream

The 707 rows were converted with `prime31_b677_capture_any.rs`. This is a
copy of PR #5076's `prime31_b677_capture.rs` that takes its dimension
checks from the input, not from ten-query constants. Every per-row group
replay and classification check is kept.

**Validation:** on the frozen ten-query input it reproduces `capture.json`
(`6dc0fb37…`), the conversion audit (`6058b62e…`) and `admitted.jsonl`
(`ed57ac0d…`) byte for byte.

On the 11-query solve, 707 rows replay, 669 are kept and 38 are excluded
for more than two large primes. After group admission the stream
(`SHA256SUMS`) holds 669 rows, and its first 613 rows are exactly the
ten-query stream.

| step on the 669-row solve stream (this host) | wall | share of cold wall |
|---|---:|---:|
| MST filter, exhaustive (`lp_mst_strategies`, median of 201, three runs) | 1.45–1.47 ms | 0.029 % |
| MST filter, shared-key | 1.40–1.42 ms | 0.028 % |
| MST filter, overlap-sensitive | 2.08–2.11 ms | 0.041 % |
| MST filter, exhaustive, pre-#5183 loop (same input, saved pre-change build) | 4.78–4.90 ms | 0.094–0.096 % |
| MST filter inside `lp_graph_eval` (warm runs 2–3) | 1.48–1.50 ms | 0.029 % |
| exact projected-rank audit of the MST output (`lp_graph_eval`) | 97–125 ms | 1.9–2.5 % |

**Outputs.**
- All three arms give identical output: 507 complete relations, 0 replay failures.
- The exact rank of that output is 506 useful and 506 augmented, the rank at which the producer recovered the scalar.
- The rank audit is a dense correctness auditor, not a production solver; the producer's own incremental rank step does the equivalent work in about 25 ms.
- The first `lp_graph_eval` run, a cold cache, measured 3.67 ms for the MST step.

## Reading

- **The MST filter is about 0.03 % of a cold M31 solve.** Relation collection is
  89 % and the pair table 10 %.
- **The earlier loop (before #5183) would also have been negligible.** Its build measured 4.78–4.90 ms on this stream (median of 101, three runs; 3.3x the current loop), about 0.09–0.10 %.
- **Further work on the MST stage cannot move the cold solve by a measurable amount.**
  Speeding up the M31 solve means reducing collection: probe additions,
  of which 10.07 M were charged, and the table.
- **Pipeline difference.** This compares stages of two different pipelines. The producer does
  its own large-prime elimination incrementally and never calls the MST
  filter. The share asks how much the MST filter would cost if it handled
  this solve's large-prime elimination. It does not describe the producer.

Host: shared 4-vCPU x86_64 cloud container, unpinned; wall times only.
