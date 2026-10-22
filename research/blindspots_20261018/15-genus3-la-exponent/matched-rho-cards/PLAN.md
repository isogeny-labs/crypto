# Item 15, addendum M: frozen task cards for the matched rho

Decision: `../PROTOCOL-ADDENDUM-matched-rho.md` (committed in 7555a42a6c before any run). Instrument:
`research/blindspots_20261018/instruments/blindspots_15b_genus3_la_rho.rs`, sha256
`79a821504c5b7a4639470d8e04dfd35fc6ad1a91d34bacee466461f36811e991` (pinned in every card).

## Cards

| card | card_id | cell | receipt |
|---|---|---|---|
| `card-m00.yaml` | CARD-BLINDSPOT-15-M00 | b = 20, curve 0 | `raw/matched/receipt-card-m00.json` |
| `card-m01.yaml` | CARD-BLINDSPOT-15-M01 | b = 20, curve 1 | `raw/matched/receipt-card-m01.json` |
| `card-m02.yaml` | CARD-BLINDSPOT-15-M02 | b = 20, curve 2 | `raw/matched/receipt-card-m02.json` |
| `card-m03.yaml` | CARD-BLINDSPOT-15-M03 | b = 20, curve 3 | `raw/matched/receipt-card-m03.json` |

(Paths relative to `research/blindspots_20261018/15-genus3-la-exponent/`.) No commit pin. Each card is
`kind: frozen_command`, `background: true`, env `RAYON_NUM_THREADS=1`, `OMP_NUM_THREADS=1` on every command.

## What each card reproduces

One process per curve c, the argv of the addendum (and of cards 06 to 09 for every flag they share):

1. `mkdir -p raw/matched/bin-c<c>`
2. `prlimit --as=4294967296 rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -o raw/matched/bin-c<c>/b15b <instrument>`
3. `prlimit --as=4294967296 raw/matched/bin-c<c>/b15b --bits 20 --curve <c> --kappa 1.3059 --cx 8 --streams r2 --las lpsgebw2@8,lpsgebw2*,lpsgebw4x4@8,lpsgebl4@8 --orders orders.csv --no-c1 --no-gp-check --arms ic2lp,rho --rho-targets 16 --rho-r 64 --out raw/matched/b20_c<c>_matched.jsonl`
   with timeout 7200 s. The four recorded IC arms (as cards 06 to 09) then 16 rho walks, one process.
4. Four continuity gates, `b15b gate <arm> raw/matched/b20_c<c>_matched.jsonl raw/b20_c<c>_r2.jsonl`, arms
   `r2:lpsgebw2@8`, `r2:lpsgebw2*`, `r2:lpsgebw4x4@8`, `r2:lpsgebl4@8`.
5. `rm -f` the binary, `rmdir` its directory.

Flag check against the addendum and cards 06 to 09: every flag shared with cards 06 to 09 is identical
(bits, curve, kappa, cx, streams, las, orders, no-c1, no-gp-check, prlimit, rustc flags); the new flags
`--arms ic2lp,rho --rho-targets 16 --rho-r 64` are the addendum's. All of them are accepted by
`parse_args` of the 15b source. Differences, none in a counted value:

* **Paths.** The addendum writes `raw/b20_c<c>_matched.jsonl` and builds into `target/blindspots-15b/`.
  The cards write under `raw/matched/` (the owner's write-scope instruction), so the outputs and the
  binary are inside `write_scope`. Nothing else differs.
* **Gate invocation.** Cards 06 to 09 ran the gate under `prlimit`; the addendum's gate line and these
  cards run it without, as the user's instruction and the addendum say.
* **Reference for the gate.** The addendum names the recorded `raw/b20_c<c>_r2.jsonl`. Cards 06 to 09
  gated against `research/ic_improve4_20261015/.../b20_c<c>_r2.jsonl`. The cards here use the
  addendum's file, pinned by sha256.
* **Timeout** 7200 s for the run: the addendum's stated deviation from the 8-minute card rule.

## Gate condition

The card format has no stdout-content success test. Success of a gate command is its exit status: the
source (`gate_cmd`, `main`) exits 0 exactly when it printed `GATE OK cells=<n>` and exits 2 when it printed
`GATE FAILED mismatches=<m> cells=<n>`. Each gate's stdout is also recorded in `raw/matched/gate-c<c>-<arm>.txt`
and `stop_when` stops the executor if no printed gate line begins with `GATE OK`.

Fields the gate compares (record for the arm in the new file against the same arm in the reference): `k`,
`k_true`, `correct`, `cost`, `cost_cl9`, `rel_cost`, `la_cost`, `rel_counters`, `la_counters`,
`sge_counters`, `total_counters`, `steps`, `rows_collected`, `checks`, `la_cols`, `la_nnz`, `matvecs`. Wall
times and `vmhwm_kb` are exempt (addendum, continuity control 1).

## Outputs (all under `raw/matched/`, all in must_not_exist, write_scope is the directory)

Per curve c: `b20_c<c>_matched.jsonl` (the record, hashed in the receipt), `run-c<c>.stdout.log`,
`run-c<c>.stderr.log`, `build-c<c>.stdout.log`, `build-c<c>.stderr.log`,
`gate-c<c>-w2at8.txt`, `gate-c<c>-w2star.txt`, `gate-c<c>-w4x4at8.txt`, `gate-c<c>-l4at8.txt`
(arm names `r2:lpsgebw2@8`, `r2:lpsgebw2*`, `r2:lpsgebw4x4@8`, `r2:lpsgebl4@8`),
`receipt-card-m0<c>.json`. The binary and `bin-c<c>/` exist only between steps 2 and 5. The binary is
deliberately not a declared output: `tools/task_card.py` re-hashes declared outputs at assessment and
the binary is gone by then.

If a gate fails the card stops there and `bin-c<c>/b15b` stays on disk inside `raw/matched/`; it must
be deleted by hand before any commit (it is a binary, not a record).

## Controls not in the cards (addendum, "Continuity controls" 2 to 4)

Rho target 0 against round 4's cost and steps, every walk and IC arm returning the planted k, and
`strongest_rho_symmetry == "negation"` are read from the jsonl in analysis, not checked by the cards.

## Running a card longer than 10 minutes

The dispatch text (`python3 tools/task_card.py prompt <card>`) says "Run it in the background: yes" and
the card's note says: Bash tool with `run_in_background: true` and `timeout: 7200000` (the tool's
maximum), then return the printed summary after the tool exits. The command is
`python3 tools/task_card.py run <card>` from the repository root. Foreground Bash is capped at 600000 ms,
so the card cannot run in the foreground. Cards may run concurrently (distinct binaries, outputs and
receipts; the 4-vCPU host fits three single-thread jobs).

## Time estimates

* Build: about 15 s (cards 06 to 09 receipts: 13.6 to 15.6 s).
* IC arms in the run: about 255-265 s (cards 06 to 09 receipts: 254.3 to 265.2 s for the same four arms).
* 16 rho walks: about 16 x 95-130 s = 25-35 min (addendum "Budget"; 93.2 s measured for one walk).
* Gates and cleanup: under 1 s.
* Per card about 30-40 min; the run timeout is 7200 s and a heavy rho tail is the risk. Four cards:
  about 2.3 CPU-h in total, about 1 h wall at three concurrent jobs, so the fourth card runs after one
  of the first three ends.
* The Bash background timeout of 7200000 ms covers the whole card; a run that used its full 7200 s would
  leave no margin for the build, and would already be a stop.

## Replay card r01 (validation gate: independent replay of a sample of the matched rho walks)

Card `card-r01.yaml` (CARD-BLINDSPOT-15-R01), receipt `raw/matched-replay/receipt-card-r01.json`, all outputs and
the binary under `raw/matched-replay/` (paths relative to `research/blindspots_20261018/15-genus3-la-exponent/`).
The IC arms of the m cards are replayed and gated elsewhere; this card replays only the rho arm.

### Why a prefix, and why it is exact

In the 15b source (`main`, "15b: round 4's rho dispatch") the rho loop clones the stream `base` once (the state
after the shared k was drawn) and then threads one generator `r2` through every walk: target 0 is the shared k,
target i >= 1 is `r2.randrange(1, ell)`, and each `rho_neg` call consumes `r2` for its multipliers and restarts.
So the targets and per-walk randomness do not depend on `--rho-targets`: `--rho-targets K` reproduces exactly the
first K walks of the 16-walk run. They do depend on the earlier walks, so a sample cannot skip walks: it is a
prefix. Everything before `base` (instance build, G, the phi checks, k, `measure_c_add`, `symmetry_check`)
uses only the seed (20261014 * 1000 + 10 b + curve) and not `--arms`; the IC streams run on clones of `base` and
do not advance it. Counters are `reset()` and the native counters `nrestore`d before every walk.

### Sample (recorded wall and steps from `raw/matched/b20_c<c>_matched.jsonl`, `results[]`, `arm == "rho"`)

| curve | `--rho-targets` | walks replayed | recorded steps | recorded wall (s) | cumulative (s) |
|---|---|---|---|---|---|
| 0 | 4 | targets 0, 1, 2, 3 | 232712993, 444291395, 128360662, 70657559 | 99.4, 188.2, 53.0, 29.1 | 369.6 |
| 2 | 3 | targets 0, 1, 2 | 219336676, 219819746, 111700251 | 92, 95, 48 (cumulative 92, 187, 235) | 235 |

Seven walks on two curves, target 0 (the shared IC k) on both. Estimated about 605 s of rho plus the build
(about 15 s) and two instance builds (under 1 s each): about 10.5 minutes, one thread, hence `background: true`.

### What each replay record must equal

Compare `raw/matched-replay/b20_c<c>_rho_replay.jsonl` (one line per curve) to `raw/matched/b20_c<c>_matched.jsonl`
(one line per curve), the `results[]` entries with `arm == "rho"` and `target` in the replayed range, matched by
`target`. Every field below must be equal (integers exactly; floats equal as parsed, they come from the same
integer counters and the same code):

* per rho record: `arm`, `target`, `k`, `k_true`, `correct`, `steps`, `counters` (5 values), `setup_counters`
  (5), `restarts`, `escapes`, `r`, `cost`, `cost_cl9`, `cost_per_step`, `rho_model_neg`, `steps_over_sqrt_ell`,
  `steps_over_model`, `native` (all native counters), and `gae` (`counted_fq`, `counted_gae`, `native_ops`, and
  `S` at every w);
* per curve line, the instance and the unit it is priced in: `bits`, `q`, `curve`, `seed`, `mode`, `build_tries`,
  `N`, `ell`, `c`, `h`, `a`, `b`, `r0`, `Kmod`, `checks`, `relprim`, `walks`, `instrument`, `symmetry` (including
  `strongest_rho_symmetry == "negation"`), `c_add` (`mean`, `min`, `max`, `additions`, `c_dbl_mean`, `unit`),
  `native_names`, `native_note`.

Exempt (wall or descriptive, or differ by construction): `wall_total` of every rho record; `build_wall`;
`vmhwm_kb`; `arms_run` (`["rho"]` against `["ic2lp","rho"]`); `matched_rho` (null in the replay, because it has
no IC arm); and the IC `results[]` entries, which the replay does not produce. The 16 - K recorded walks beyond
the prefix are not replayed and stay unreplayed. A difference in any listed field is a replay failure to report,
never to explain away.
