# Item 14 replay plan (validation gate, replay certificate)

Purpose: an independent replay of recorded item-14 runs, with scalars compared
(`docs/VALIDATION_GATE.md`, "replay certificate"). Nothing here reinterprets the
item's verdict. The cards in this directory were written, not run; the replay
certificate is written only after the replay runs and the comparison below is done.

## What is replayed, and how it differs from the original cards

* Same instrument bytes: `research/blindspots_20261018/instruments/blindspots_14_prime_m4.rs`,
  sha256 `f6696fc0cafff9120189072a7785452422d68c7944a2812d4667e27c8872b376`
  (the old `examples/blindspots_14_prime_m4.rs`; `instruments/README.md` has the map).
* Same build argv as the original cards (`prlimit --as=4294967296 rustc --edition 2021
  -C opt-level=3 -C codegen-units=1 -C target-cpu=native`), only the source path and the
  output binary path differ. The binary is built under `replay/bin-rNN/` and removed by the
  card, as the originals removed theirs under `target/`.
* Same arguments as the recorded runs, so same default seed `20261018` (printed in each
  `cell` row) and the same targets and planted values (derived from the seed and the run or
  target index). No `--seed` is passed anywhere.
* New output paths only: every output, stdout, stderr and receipt is under
  `research/blindspots_20261018/14-prime-m4-cells/replay/raw/`, and every card lists them
  in `must_not_exist`, so a rerun refuses rather than overwrites.
* Pinned commit: `f543b5523357eb11490a30fcac84d17359ce98ad` (HEAD when the cards were written).
  `tools/task_card.py run` refuses when HEAD differs. If the cards are committed first, HEAD moves and
  the `commit:` line of each card must be re-pinned (then re-lint) before it is relayed.
* Run the three cards one after another, never at the same time (wall time is descriptive, but
  concurrent runs on a 4-vCPU host distort it and the arm (b) deadlines are wall-clock).

## Mapping: replay card, command, recorded run

"Recorded" paths are under `research/blindspots_20261018/14-prime-m4-cells/raw/`; "replay" paths
under `.../replay/raw/`. Same file name unless stated.

### card-r01 (replay of card-01; arm (a) and matched rho; 16-bit and 28-bit; about 30 s)

| replay command id | arm | recorded file (recorded card) | rows |
|:--|:--|:--|:--|
| selftest | positive controls | `card-01-selftest.stdout.log` (card-01) | text, see below |
| rho-b16 | matched rho, 16 bit | `rho-b16.jsonl` (card-01) | 16 `rho` rows |
| rho-b28 | matched rho, 28 bit | `rho-b28.jsonl` (card-01) | 16 `rho` rows |
| tdec-a-b16 | (a) T_dec, design cell bx = 8 | `tdec-a-b16.jsonl` (card-01) | `cell`, `setup`, 16 random + 4 planted `tdec` |
| tdec-a-b28 | (a) T_dec, design cell bx = 63 | `tdec-a-b28.jsonl` (card-01) | same shape |
| tdec-a-b16-bx3 | (a) T_dec, bx = 3 (paired with arm (b) bx = 3) | `tdec-a-b16-bx3.jsonl` (card-01) | same shape |
| fullic-a-b16 | (a) full IC, bx = 8 | `fullic-a-b16.jsonl` (card-01) | 16 `fullic` rows |
| fullic-a-b28 | (a) full IC, bx = 63 | `fullic-a-b28.jsonl` (card-01) | 16 `fullic` rows |
| fullic-a-b16-bx3 | (a) full IC, bx = 3 | `fullic-a-b16-bx3.jsonl` (card-01) | 16 `fullic` rows |

### card-r02 (replay of cards 02, 09, 10; arm (b), 16-bit; about 2.5 min)

| replay command id | arm | recorded file (recorded card) | rows |
|:--|:--|:--|:--|
| tdec-b-b16-bx3 | (b) T_dec, bx = 3 | `tdec-b-b16-bx3.jsonl` (card-02) | `cell`, `setup`, 16 random + 2 planted `tdec` |
| tdec-b-b16-bx4 | (b) T_dec, bx = 4 | `tdec-b-b16-bx4.jsonl` (card-02) | same shape |
| tdec-b-b16-bx5 | (b) T_dec, bx = 5 | `tdec-b-b16-bx5.jsonl` (card-02) | same shape |
| fullic-b-b16-bx3 | (b) full IC, bx = 3 | `fullic-b-b16-bx3.jsonl` (card-09) | 16 `fullic` rows |
| fullic-b-b16-bx4-r00-02 | (b) full IC, bx = 4, runs 0 and 1 | first two rows (`run` 0 and 1) of `fullic-b-b16-bx4-r00-06.jsonl` (card-10) | 2 `fullic` rows |

### card-r03 (replay of cards 09 and 04; arm (b), largest field and design-cell bx = 8; about 2.5 min)

| replay command id | arm | recorded file (recorded card) | rows |
|:--|:--|:--|:--|
| tdec-b-b28-bx63 | (b) T_dec, 28-bit design cell, run to the cap | `tdec-b-b28-bx63.jsonl` (card-09) | `cell`, `setup`, 16 random `tdec` (status `budget-nnz`) |
| tdec-b-b16-bx8-t00-01 | (b) T_dec, 16-bit design cell bx = 8, target index 0 | the `cell` row, the `setup` row and the `tdec` row with `target` 0 of `tdec-b-b16-bx8-t00-05.jsonl` (card-04) | 3 rows |

## What must match exactly

Compare row by row, in file order, after deleting every key named `ms` from each JSON row.
Every other key must be equal (integers exactly; no tolerance). Keys, per row kind:

* `cell`: `bits`, `p`, `a`, `b`, `n`, `B`, `bx`, `signed`, `seed`.
* `rho`: `bits`, `run`, `ok`, `setup_gadd`, `walks`, every entry of `counts`, `gae`, `native`, `n`,
  `sqrt_pi_n_over_2`.
* arm (a) `setup`: `bits`, `B`, `bx`, `entries`, every entry of `counts`.
* arm (a) `tdec`: `bits`, `B`, `bx`, `target`, `tkind`, `hit`, `valid`, `crosscheck_decomposable`,
  `planted`, every entry of `counts`, `native`.
* arm (b) `setup`: every entry of `counts` (the `build` count), `note`.
* arm (b) `tdec`: `target`, `tkind`, `status`, `dim`, `brute_roots`, `planted_vanishes`, `steps`,
  `max_degree`, `max_rows`, `max_cols`, `peak_nnz`, `zero_rows`, every entry of `counts`
  (`madd`, `build`, and the zero counts), `native`.
* `fullic` (both arms): `arm`, `bits`, `bx`, `run`, `ok`, `attempts`, `hits`, `relations`, `rank`,
  `cols`, every entry of `setup`, `online`, `verify` and `total`, `gae_total`, `native_total`, `n`,
  `note`. These include the solve result (`ok`, `rank` = `cols`, `relations`) and the counted
  operations (`gadd`, `lookup`, `insert`, `legendre`, `sqrt`, `rowop`, `madd`, `dptest`, `fmul`,
  `fsqr`, `finv`, `build`).
* Row counts and row order equal to the "rows" column above.
* Selftest text (`card-r01-selftest.stdout.log` against `card-01-selftest.stdout.log`): line count
  equal; every line equal after removing the timing text (`in <x>s`, `<n> ms`, `(<x>s)`), and the
  last line reads `SELFTEST OK`. The `dim ... = brute ...`, `madds`, `attempts`, `hits`, `rank`,
  `adds` and `walks` figures in those lines are deterministic and must match.
* Every command in the replay receipt has `exit_code` 0.

Why exact is the right bar: every key above is a deterministic count or a result computed from the
fixed seed and the pinned source; wall time is the only run-to-run variable. The arm (b) capped cells
(`tdec-b-b28-bx63`) stop on the nonzero cap (`budget-nnz`), not on time, so their counts are deterministic.
A host slow enough to trip the 60 s per-target deadline on that cell would change `status`; that is an
infrastructure failure to report, not a pass.

## What is descriptive only (not compared)

* `ms` in every row, the selftest timing text, the per-command `seconds` in receipts, the build time.
* sha256 of the replay jsonl files: not expected to equal the recorded ones, because `ms` differs.
* The compiled binary and the `rustc` version (the recorded receipts do not pin a toolchain; the
  counts do not depend on the CPU or on `target-cpu=native`).

## Suggested comparison (run after the cards, not part of any card)

For each pair in the tables, read both jsonl files, drop the `ms` key from every row, and require
equal lists (for the two sliced pairs, first select the rows named in the table). Any difference is
a failed replay for that file, reported with the first differing key; there is no tolerance.

## What was dropped, and why

Target: under about 30 min on a shared 4-vCPU host. The three cards total about 5.5 min of recorded
compute (card-01 took 29.7 s, card-02 154 s, card-09 170 s; the arm (b) full-IC and capped cells are
the cost), so about 15 min if the shared host runs 2.5 times slower.

| dropped from the recorded set | reason |
|:--|:--|
| 20-bit and 24-bit cells (rho, arm (a), arm (b) caps) | the gate asks for the smallest (16-bit) and largest (28-bit) field; the middle sizes cost little for (a) but add no coverage of the requested ends |
| 16-bit ladder bx = 6, 7 for (a) and (b), and bx = 6 (37 s), bx = 7 (212 s) for (b) T_dec | the ladder ends bx = 3 and the bx = 8 design cell are replayed; bx = 6 and 7 cost 37 s and 212 s of recorded time for the same code path |
| arm (b) T_dec bx = 8, targets 1-4 and the planted targets 16, 17 (cards 04-07, about 940 s of recorded time) | 53 s per target; one target (index 0) replays the design cell; the planted-target solve check (`dim` = `brute_roots` = 24) is replayed at bx = 3, 4, 5 |
| arm (b) full IC bx = 4 runs 2-15 (cards 10-12, about 490 s) | runs 0 and 1 (53 s) replay the cell; the remaining runs use the same code with other run-indexed seeds |
| arm (b) T_dec at 20 and 24 bits (card-08, 185 s) and the bx = 5, 6 cells at 20, 24, 28 bits (card-03) | 28-bit arm (b) design cell is replayed; the 20/24-bit cells add 185 s without a new field-size end |
| `reach` and `reuse-pkm` commands (card-01) | arm (d) and arm (c) are not among the arms requested; they cost under 0.1 s and can be added without changing anything else |
| a 28-bit arm (b) full IC | never recorded (arm (b) hit the cap at 28 bits); there is nothing to replay |

Arm (b) full IC exists in the record only at 16 bits (bx = 3 and 4), so "smallest and largest
field size" for arm (b) is: smallest = 16-bit bx = 3 (T_dec and full IC), largest = 28-bit
bx = 63 (T_dec to the cap).
