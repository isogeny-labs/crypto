# Stage B readings (attempt 3; Amendments 19–20), scored by the frozen tool

Derived by `prime_fall_degree_tools score` (frozen binary `7e8a6703…`) over the 9 `B-*` closure files of `pfd-b-rp-1` and the
8 `DB-*` F4 files of `pfd-b-rp-2`. Inputs: `STAGE_B/inputs.sha256`; output verbatim in `STAGE_B/score_frozen.txt`. Every number
is `[pcc:unverified]`. These are the first admitted stage B rows of this study.

## Gates

- **E3.7 positive control** on both stage B pods: X1/X3 reproduced exactly (`CONTROL_E37_20261115.md`).
- **G1:** pass, both pods.
- **G2 (`d_F ≤ D`):** 41 draws checked, **0 violations**. Repeated systems 0; planted violations 0.
- **G3** (Amendment 11 scope, rerun under Amendment 20 with `--jobs 1`): **123 capped runs, 0 disagreements, 0 timeouts**, tower
  and null rows included. The 22 runs of the crashed first attempt are reproduced exactly (`pfd-b-g3b/`).

## Ladder readings (§6 as registered; frozen scorer's words)

| p | kind | m | control | t | median `d_F` | H-LFD | H-FFD |
|--:|:--|--:|:--|:--|:--|:--|:--|
| P1 | kummer | 3 | tower | 2–6 | 5 6 6 7 ≥7 | **grows** | FFD assumption fails |
| P1 | kummer | 3 | null | 2–6 | 5 6 6 7 ≥7 | **grows** | no D |
| P1 | kummer | 4 | tower | 2–4 | 6 7 ≥7 | mixed | consistent |
| P1 | kummer | 5 | tower | 2–5 | 6 ≥8 ≥8 ≥7 | insufficient | FFD assumption fails |
| P0 | kummer | 3 | tower | 2–5 | 5 6 6 ≥7 | mixed | FFD assumption fails |
| P0 | kummer | 4 | tower | 2–4 | 6 7 ≥7 | mixed | consistent |
| P0 | kummer | 5 | tower | 2–3 | 6 ≥8 | insufficient | FFD assumption fails |
| P0 | isogeny | 3 | tower | 2–5 | 6 6 6 7 | mixed | consistent |
| P0 | isogeny | 4 | tower | 2–4 | 6 7 ≥7 | mixed | consistent |
| P0 | isogeny | 5 | tower | 2–3 | 7 ≥8 | insufficient | consistent |

- **`m = 3` at P1.** `d_F` grows from 5 (`t = 2`) to 7 (`t = 5`), and is at least 7 at `t = 6`. The shape-matched null reads
  **identically** at every `t`. At `m = 3`, as at `m = 2` in stage A, the growth is shape-driven: it does not separate the Semaev
  system from its null.
- **"mixed"** is the frozen scorer's word for a ladder whose top cells are censored. `m ≥ 4` ladders reach their caps within 2–3
  levels.
- **Isogeny against Kummer at P0** (descriptive). For `m = 3` the isogeny medians are 6 6 6 7 against Kummer's 5 6 6 ≥7. At
  `m = 4` they are equal. At `m = 5`, `t = 2`, isogeny is 7 against Kummer's 6. The isogeny tower is never below the Kummer
  tower, consistent with `m = 2` (`results/ISO/SCORE/READING.md`).
- H-m (`t` fixed, `m = 3..6`): "mixed" on every ladder, because the top `m` cells are censored.
