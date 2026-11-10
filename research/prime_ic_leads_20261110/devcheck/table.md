# Development-check rows: tower vs multiples (`[pcc:unverified]`, local host)

| file | m | t | N | target | d_F tower | d_F multiples | Δ | ffd tower / mult | falls tower / mult | n_vars t/m | n_eq t/m | wall s t/m |
|:--|--:|--:|--:|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| agent_smoke_m2_t4.jsonl | 2 | 4 | 8 | planted #0 | none (planted/consistent) | none (planted/consistent) | — | 4 / 4 | [4] / [4] | 8 / 8 | 9 / 10 | 0 / 0 |
| agent_smoke_m2_t4.jsonl | 2 | 4 | 8 | random #1 | 4 | 4 | 0 | 4 / 4 | [4] / [4] | 8 / 8 | 9 / 10 | 0 / 0 |
| agent_smoke_m4_t2-3.jsonl | 4 | 2 | 8 | planted #0 | none (planted/consistent) | none (planted/consistent) | — | 4 / 4 | [4, 5, 6] / [4, 5] | 10 / 12 | 11 / 16 | 4 / 39 |
| agent_smoke_m4_t2-3.jsonl | 4 | 2 | 8 | random #1 | 6 | 5 | -1 | 4 / 4 | [4, 5, 6] / [4, 5] | 10 / 12 | 11 / 16 | 1 / 1 |
| agent_smoke_m4_t2-3.jsonl | 4 | 3 | 12 | planted #0 | > 7 (censored) | — | — | 4 / — | [4, 5, 6] / — | 14 / — | 15 / — | 611 / — |
| agent_smoke_m4_t2-3.jsonl | 4 | 3 | 12 | random #1 | 7 | — | — | 4 / — | [4, 5, 6, 7] / — | 14 / — | 15 / — | 450 / — |
| m2_t2-10.jsonl | 2 | 2 | 4 | random #0 | 4 | — | — | 4 / — | [4] / — | 4 / — | 5 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 2 | 4 | random #1 | 4 | — | — | 4 / — | [4] / — | 4 / — | 5 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 3 | 6 | random #0 | 4 | — | — | 4 / — | [4] / — | 6 / — | 7 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 3 | 6 | random #1 | 4 | — | — | 4 / — | [4] / — | 6 / — | 7 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 4 | 8 | random #0 | 4 | — | — | 4 / — | [4] / — | 8 / — | 9 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 4 | 8 | random #1 | 4 | — | — | 4 / — | [4] / — | 8 / — | 9 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 5 | 10 | random #0 | 4 | — | — | 4 / — | [4] / — | 10 / — | 11 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 5 | 10 | random #1 | 4 | — | — | 4 / — | [4] / — | 10 / — | 11 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 6 | 12 | random #0 | 5 | — | — | 4 / — | [4, 5] / — | 12 / — | 13 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 6 | 12 | random #1 | 5 | — | — | 4 / — | [4, 5] / — | 12 / — | 13 / — | 0 / — |
| m2_t2-10.jsonl | 2 | 7 | 14 | random #0 | 5 | — | — | 4 / — | [4, 5] / — | 14 / — | 15 / — | 2 / — |
| m2_t2-10.jsonl | 2 | 7 | 14 | random #1 | 5 | — | — | 4 / — | [4, 5] / — | 14 / — | 15 / — | 2 / — |
| m2_t2-10.jsonl | 2 | 8 | 16 | random #0 | 5 | — | — | 4 / — | [4, 5] / — | 16 / — | 17 / — | 13 / — |
| m2_t2-10.jsonl | 2 | 8 | 16 | random #1 | 5 | — | — | 4 / — | [4, 5] / — | 16 / — | 17 / — | 14 / — |
| m2_t2-10.jsonl | 2 | 9 | 18 | random #0 | 5 | — | — | 4 / — | [4, 5] / — | 18 / — | 19 / — | 57 / — |
| m2_t2-10.jsonl | 2 | 9 | 18 | random #1 | 5 | — | — | 4 / — | [4, 5] / — | 18 / — | 19 / — | 65 / — |
