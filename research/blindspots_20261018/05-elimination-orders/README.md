# BLINDSPOT-05: monomial, variable and block orders tuned to the descent structure

## Verdict

**INCONCLUSIVE**, level **none** (no level can be
stated: the rule's comparison was never made). `b05 summarize` over the
recorded lines prints `VERDICT: INCONCLUSIVE (fewer than 4 of 6 cells
countable)`; countable cells are **0/6**.

The cause is a defect in the frozen cards, not a property of the orders. In
every multi-arm command (cards 02 to 06) the card writes
`--arms, grevlex,wdeg,blkIL,...` as a YAML flow sequence, which splits on the
commas, so `task_card.py` passed each name as a separate argv item. The
instrument reads only the token after `--arms`, so **only `grevlex` ran** in
those commands (card 02's 8-target main run took 4.8 s, which is 8 grevlex
jobs). The orders `wdeg`, `blkIL`, `mindeg`, `tw`, the reverse arms and
`rand0..rand7` were never measured in the four full cells, so no random band
exists and no cell is countable. The cards are frozen and hash-bound; this is
reported, not repaired. A corrected card revision (quote the list as one argv
string) is needed before the question is answered.

This is a stage diagnostic on a native oracle's matrix-entry "touches". It
computes no `S`, no group-operation count and no rho ratio, so there is no
cross-method row: the gap to matched Pollard rho is not computed here, and the
0 / 0.1 / 1 GAE pricing of AGENTS.md section 5a has nothing to price. This is
not an improvement over the index-calculus baseline and not a regression of it.

## What was measured (all from `raw/`, measured unless stated)

* Controls (card 01, passed): selftest OK (84/84 committed unsat draws refuted,
  `D_mat(grevlex)` equals the committed closure degree on 84/84 and msolve on
  32/32); planted-order sensitivity OK (smallest block-order touches ratio
  11.18 over 6 systems, against the 1.5 rule).
* `grevlex` refuted every target it was given with `D_mat = 4`: 8/8 in each of
  `c17-4-3`, `c19-4-3`, `c13-3-4`, `c15-3-4`; 2/2 in `c17-4-4`; 1/1 in
  `c31-15-2`. Geometric-mean touches per cell: 2.82e8, 5.96e8, 8.04e8, 1.74e9
  (the (15,3,4) targets split into 5 near 4.6e8 and 3 near 1.6e10), 7.87e10,
  1.33e11.
* `blkLI` (2 targets per cell, paired with grevlex on the same targets):
  `c13-3-4` refuted at `D_mat = 6`, touches ratio grevlex/blkLI **0.062 and
  0.064**, largest-matrix ratio 0.21; `c15-3-4` refuted at `D_mat = 6`, ratio
  **0.038 and 0.039**, matrix ratio 0.046; `c17-4-3` and `c19-4-3` hit the
  40 s cap (status `timelimit`, `D_mat` reached 5) with grevlex ratio at most
  0.053 to 0.072 and 0.042 to 0.043 (upper bounds, since blkLI had not
  finished). blkLI is 16 to 26 times costlier than grevlex where it finished
  (the sensitivity control shows the same order is 11 to 68 times cheaper on
  planted systems, so the instrument does see block structure; these
  systems do not have it).
* `wdeg` on `c17-4-4` (cards 08, 10): `timelimit` at 370 s on both targets
  (`D_mat` reached 6, 2.9e10 and 3.4e10 touches) while grevlex refuted in
  170 s and 161 s: a cap hit where grevlex finished, counted as worse.
* `c31-15-2` target 0 (cards 11 to 15, one arm each): grevlex, `mindeg` and `tw`
  have identical touches (1.3283e11; the three orders coincide on this
  system) and `rand0`, `rand1` refuted with 1.2727e11 touches, ratio 1.044
  (4% fewer than grevlex). One target, two random draws: this shows only that
  a random order is not worse here; there is no interval.
* No interval is given: the frozen rule uses geometric-mean ratios over at
  most 8 targets (2 for blkLI) and this run has no replicate of a job.

## Rule outcome by arm (from `b05 summarize`)

| arm | cells meeting the constant criterion | cells meeting primitive | cells at least 1.5x worse |
|---|---|---|---|
| blkLI, blkIL, wdeg, mindeg, tw | 0/6 | 0/6 | 0 (judged only where the random band exists; it exists nowhere) |

## Controls, deviations, caveats

* Controls passed; they validate the instrument, not the cards' arm lists.
* Deviation 2 (rustc, not cargo) stands. New deviation: the arm-list defect
  above; cards 02 to 06 carry it, cards 07 to 15 are single-arm and ran as
  written. Cards 06, 08, 10, 14 and 15 had no result when the relay arrived;
  they were run in this stage with `tools/task_card.py run` (receipts in
  `raw/`, outputs in `raw/relay-NN.txt`) and each ended `completed`.
* Relay: `check-relay` passes for cards 06, 08, 10, 14, 15 (their relays are the
  tool output). For cards 01 to 05, 07, 09, 11 to 13 the executor text was not
  re-saved to `raw/relay-N.txt`; the on-disk receipts and logs were read
  instead. Card 03's relayed text was visibly corrupted (the blkLI target-1
  line repeats `ms_back`/`ms_add` and drops fields); the on-disk log is
  intact (1268 bytes, parses as JSON).
* Host is a shared 4-vCPU virtualized container (level L0 isolation, another
  lane running concurrently), so wall times are descriptive only. The systems
  are toy scale (29 to 50 unknowns); `c17-4-4` and `c31-15-2` were BLOCKED for
  the rule by budget by design (PROTOCOL.md).
* Not a speed or cost result; nothing is copied to `benchmarks/`.

## Files

`PROTOCOL.md`, `draws.tsv`, `card-01..15.yaml`, `raw/`, `figures/order_ratios.svg`,
`figures/order_ratios.png`, `figures/make_figure.py` (plots recorded values
only), `REPORT.pdf`.

![ratios](figures/order_ratios.png)
