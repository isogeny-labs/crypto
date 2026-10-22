# Item 15, validation-gate replay: frozen task cards

Purpose: the independent replay of recorded runs with scalars verified that `docs/VALIDATION_GATE.md` asks of
a replay certificate. Written 2026-10-22 by the experiment-director lane. Nothing here has been run.
Paths below are relative to `research/blindspots_20261018/15-genus3-la-exponent/` unless they start with `research/`.

## What is replayed, and against what

* Instrument: the ORIGINAL pinned instrument `research/blindspots_20261018/instruments/blindspots_15_genus3_la.rs`
  (the old `examples/blindspots_15_genus3_la.rs`), sha256
  `73cfa4891f8eb4651b728e426aa4fd47f2180120b06c60081dab1b230ad5462b`, pinned in every card's `binds`. Not the
  15b (rho) instrument.
* Build, flags and env exactly as cards 01 to 09: `prlimit --as=4294967296 rustc --edition 2021 -C opt-level=3
  -C codegen-units=1`, env `RAYON_NUM_THREADS=1`, `OMP_NUM_THREADS=1` on every command, the same `--bits --curve
  --kappa --cx --streams --las --orders --no-c1 --no-gp-check` per cell (argv copied from `card-01` to `card-09`
  per cell; `orders.csv` pinned by sha256 `d31864a0...`). Seeds are not flags: the instrument derives
  `20261014000 + 10 b + curve` itself, and the recorded `seed` field says which one ran.
* Reference for every comparison: the recorded `raw/b<b>_c<c>_<r2|aa>.jsonl` of cards 01 to 09, each pinned by sha256
  in the card's `binds`. (Cards 01 to 09 gated against the `ic_improve4` files; the replay gates against the
  item's own recorded output, which is what is being certified.)
* Outputs only under `replay/`: `b<b>_c<c>_<stream>.jsonl`, `gate-r<NN>-<tag>.txt`, `selftest.txt` (r01), `build-r<NN>.*.log`,
  `r<NN>-<cell>.stderr.log`, `receipt-card-r<NN>.json`. The binary is built at `replay/bin-r<NN>/b15` and removed (with its
  directory) by the last two commands. `write_scope` is `replay/`. Every output, log, the binary and its
  directory are in `must_not_exist`. No `commit` pin.
* Gate: one `b15 gate <arm> <new> <ref> [<new> <ref> ...]` command per arm, covering every replayed cell of the card,
  so every recorded arm of every replayed record is compared. Exit 0 means `GATE OK`, exit 2 means
  `GATE FAILED` (instrument `gate_cmd`). `success_exit_codes` is `[0]`, so a mismatch stops the card at that gate
  (status `stopped at command N`), and the remaining gates are not run and `bin-r<NN>/` stays on disk in `replay/`
  until the director's assessment records it and it is deleted by hand (as in `matched-rho-cards/PLAN.md`).

## Mapping: card -> recorded run

| card | card_id | cells replayed (each: r2 collection with every arm, plus the aa stream where recorded) | recorded run it replays (the original card, its receipt and files) | gates | est. CPU s | background |
|---|---|---|---|---|---|---|
| `card-r01.yaml` | CARD-BLINDSPOT-15-R01 | selftest; b = 16 curves 0,1,2,3; b = 17 curves 0,1,2,3 | card-01 (`raw/receipt-card-01.json`): `raw/selftest.txt`, `raw/b16_c{0..3}_{r2,aa}.jsonl`, `raw/b17_c{0..3}_{r2,aa}.jsonl` | 9 (8 r2 arms, aa) | 189 | true |
| `card-r02.yaml` | CARD-BLINDSPOT-15-R02 | b = 18 curves 0,1 | card-02: `raw/b18_c{0,1}_{r2,aa}.jsonl` | 9 | 192 | true |
| `card-r03.yaml` | CARD-BLINDSPOT-15-R03 | b = 18 curves 2,3 | card-03: `raw/b18_c{2,3}_{r2,aa}.jsonl` | 9 | 195 | true |
| `card-r04.yaml` | CARD-BLINDSPOT-15-R04 | b = 19 curve 0 | card-04: `raw/b19_c0_{r2,aa}.jsonl` | 6 (5 r2 arms, aa) | 170 | true |
| `card-r05.yaml` | CARD-BLINDSPOT-15-R05 | b = 19 curve 3 | card-05: `raw/b19_c3_{r2,aa}.jsonl` | 6 | 174 | true |
| `card-r06.yaml` | CARD-BLINDSPOT-15-R06 | b = 20 curve 0 (r2 only; no aa was recorded at b = 20) | card-06: `raw/b20_c0_r2.jsonl` | 4 | 280 | true |
| `card-r07.yaml` | CARD-BLINDSPOT-15-R07 | b = 20 curve 3 | card-09: `raw/b20_c3_r2.jsonl` | 4 | 269 | true |

Arms covered (every arm of every recorded file of the replayed cells; the gate reports `GATE MISSING` for an arm
absent from the replay):

* b = 16, 17, 18: `r2:lpsgebw2@N`, `r2:lpsgebw2*`, `r2:lpsgebw4x4@N`, `r2:lpsgebw4x4*`, `r2:lpsgebl4@N`, `r2:lpsgebl4*`,
  `r2:lpsgebl1@N`, `r2:lpsgebw2!1@N`, and `aa:lpsgebw2@N` (N = 16 for b = 16, 17; N = 8 for b = 18).
* b = 19: `r2:lpsgebw2@8`, `r2:lpsgebw2*`, `r2:lpsgebw4x4@8`, `r2:lpsgebl4@8`, `r2:lpsgebw2!1@8`, `aa:lpsgebw2@8`.
* b = 20: `r2:lpsgebw2@8`, `r2:lpsgebw2*`, `r2:lpsgebw4x4@8`, `r2:lpsgebl4@8`.

Sizes: smallest b = 16 and largest b = 20 are both covered. Curves: all four at b = 16, 17, 18; first and last
(0 and 3) at b = 19 and 20.

## Estimate, what was dropped, and why

CPU seconds are the recorded wall seconds of the same commands in receipts 01 to 09 (single-threaded runs, so
wall = CPU): builds 13.6 to 15.6 s each, selftest 1.3 s, gates under 0.01 s. Sum over the seven cards: about
1469 s = 24.5 CPU-min, against about 2341 s = 39 CPU-min for the full original set. The 45 CPU-min ceiling on a
shared 4-vCPU host (load average 2.6 to 3.5 at the time of writing; the still-running matched b = 20 curve 3 card is one of the other jobs) leaves about 1.8x
headroom for contention.

Dropped, and why:

* b = 19 curves 1 and 2 (about 345 s with two builds): the arms and the size are already covered by curves 0
  and 3, and the budget.
* b = 20 curves 1 and 2 (about 550 s): their IC arms are already reproduced by the matched-rho cards' continuity
  gate in the 15b binary (`raw/matched/gate-c1-*.txt`, `gate-c2-*.txt`: each `GATE OK cells=1`, receipts
  `receipt-card-m01.json`, `m02.json` `completed`).
* Kept although partly duplicated: b = 20 curve 0 (r06). Its matched gate is also done (`gate-c0-*.txt`, four
  `GATE OK cells=1`, `receipt-card-m00.json` `completed`), but that ran the 15b binary; the replay must use the
  original instrument, and two instances at the largest b cost about 9 CPU-min. If the budget must shrink,
  r06 is the first card to drop (saves about 280 s). Curve 3 (r07) is the b = 20 cell not covered by any
  finished matched gate: at the time of writing the matched card for curve 3 is still running (process
  `raw/matched/bin-c3/b15b`, about 31 CPU-min used), so its gate files and receipt do not exist yet.
* Nothing at the arm level: every recorded arm of every replayed cell is gated. There is no aa record at b = 20
  to drop.
* The b = 20 curves 1 and 2 cells are the first to add back if the budget allows (about 275 s per card).

One value differs from the originals, and it is not a result-bearing value: the run timeout of the b = 20 command is 900 s
(originals 480 s, recorded run 254 to 265 s) because a shared host can slow a run; a run killed by the timeout is
`stopped`. All other timeouts equal the originals (b = 16: 90/60 s, b = 17: 150/60 s, b = 18: 240/90 s,
b = 19: 300/150 s r2/aa, build 240 s, selftest 120 s, gate 60 s). `background: true` on every card because the
sum of the timeouts (1080 to 2370 s) exceeds ten minutes, although the expected run is 170 to 280 s; the dispatch
text says "Run it in the background: yes" and the card note gives `run_in_background true` and `timeout 3600000`.

## Concurrency

All seven cards are independent: distinct binary directories, outputs and receipts, one thread each
(`RAYON_NUM_THREADS=1`), memory about 0.6 GB resident at b = 20 (`vmhwm_kb` 597324 to 602104 in the recorded
files) under a 4 GiB address-space `prlimit`. They can all run concurrently as far as files and memory go. With two other
jobs on a 4-vCPU host, run at most two at a time (more would contend for cores and inflate wall time, not counted values).
Two lanes of about 12.5 min each:

* lane A: r06, then r07, then r04 (280 + 269 + 170 s)
* lane B: r01, then r02, then r03, then r05 (189 + 192 + 195 + 174 s)

Run each card as `python3 tools/task_card.py run <card>` from the repository root.

## Exact versus descriptive fields

**Exact, checked by the instrument's gate** (per arm, new record equals recorded record; any difference is a
`GATE DIFF`): `k`, `k_true`, `correct`, `cost`, `cost_cl9`, `rel_cost`, `la_cost`, `rel_counters`, `la_counters`,
`sge_counters`, `total_counters`, `steps`, `rows_collected`, `checks`, `la_cols`, `la_nnz`, `matvecs`.

**Exact, expected equal, not checked by the gate** (counted or instance values; compare from the two jsonl files
when assessing; a difference is reported, not adjudicated): record-level `bits`, `q`, `curve`, `seed`, `mode`,
`build_tries`, `N`, `ell`, `c`, `h`, `a`, `b`, `r0`, `Kmod`, `checks`, `relprim`, `walks`; arm-level `arm`, `stream`,
`la`, `kappa`, `cx`, `bw_m`, `bw_n`, `bl_w`, `mw`, `seed_variant`, `os`, `walks`, `nfb`, `transfer_counters`,
`sge_cost`, `la_cost_cl9`, `la_in_rows`, `la_in_cols`, `la_in_nnz`, `la_rows`, `merges`, `singletons`, `trimmed`,
`dependent`, `gen_products`, `auto`, `auto_cx`, `auto_mw`, `lp_vertices`, `lp2`, `cycles`, `relprim`, `diag`.
`selftest.txt` (r01): expected byte-identical to `raw/selftest.txt` (sha256
`2d64c0a0fbfc41675c2b67d6c1c7966f74fe1b8e4be821607c05ac1d5a991895`; the receipt records the new stdout sha256, and
the recorded one is the same file's hash).

**Descriptive only** (not compared): `build_wall`, `la_wall`, `wall_stream`, `vmhwm_kb`, every stderr log's `wall`
and `la_wall`, command `seconds` in the receipts, and the contents of build logs.

Anything outside the gate's list is the validator's to compare; the cards do not check it. Mechanical output
is an observation, not a claim; what the replay means is the frontier/core tier's call.
