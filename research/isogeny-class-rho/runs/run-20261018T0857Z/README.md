# run-20261018T0857Z — ECDLP cost across four isogeny classes

Run report (research-visuals "Every experiment run"): what ran, on what,
with which records. The reading of the numbers is in the study README
([`../../README.md`](../../README.md)) and [`READING.md`](READING.md).

| | |
|:--|:--|
| protocol | [`../../PROTOCOL.md`](../../PROTOCOL.md), sha256 `81c3232bdab481244324101d10e2d50199da1990ceba10c8dc66cc206286409e`, frozen before the first measurement |
| command card | [`COMMANDS.md`](COMMANDS.md); every command ran exactly as written |
| binary | `target/release/isoclass-rho` from `src/bin/isoclass_rho/` at commit `e541289ac` (walks P1, P0, B1) and the same source for K and the measurements (`5e4d1917f` adds records only) |
| host | Intel Xeon @ 2.10 GHz, 4 vCPU, 15 GiB, shared cloud container, no isolation (L0); other jobs (the K walk, plotting) ran alongside the measurements, so wall times are indicative |
| execution | walks: a Sonnet-tier agent ran the four `walk` commands verbatim (owner's instruction); the K walk was killed by an environment restart at its second level and re-run by the same agent with the same command and seed; measurements and replays: the main session, `measure-pbb.sh` (P1, P0, B1) and `measure-k.sh` (K) |
| seeds | `--seed 20261018` everywhere (curve instances, generators, trial seeds `S ^ s`) |

## Walks

| class | explored / total vertices | measured | strata (conductors) | edges realised = verified | routes | wall |
|:--|:--|--:|:--|--:|:--|--:|
| P1 | 3137 / 3137 | 104 | 12: 1, 3, 5, 7, 9, 15, 21, 35, 45, 63, 105, 315 | 168 | Elkies 168 | 66 s |
| P0 | 68 / 68 | 45 | 12: 1, 2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 36 | 44 | Elkies 27, cubic 13, division 4 | 0.4 s |
| B1 | 177 / 1608 (level-only vertices included) | 40 | 6: 1, 7, 17, 49, 119, 833 | 523 | division 523 | 53 s |
| K | 25 / 25 | 11 | 4: 1, 409, 1721, 703889 (all of them) | 10 | torsion 10 (sampled: 3 kernels per vertex and prime; 54 Montgomery ladders of 8,347–8,794 bits in F_2^{41·204} / F_2^{41·215}, 39 s each on average) | 4,922 s (82 min, sharing the host with the measurements) |

`#E` re-counted on every measured curve (`order_checked` = measured in each
`walk.json`); every edge passed the point checks; no warning in any walk
log except the expected "Elkies declined (SpecialJ)" at P0's `j = 0` root and
its `j = 0` neighbours, which took the division route. The K walk was run twice with the same command and seed: the first run was killed by an environment restart after the root and one child (its log was overwritten by the re-run, which used the same output directory); the record and `K/walk.log` are from the second.

## Measurements

| class | mode | arms | solves | all `k` correct | rho wall |
|:--|:--|--:|--:|:--|--:|
| P1 | class | 104 + 3 iso + A/A + 2 plants = 110 | 11,000 | yes | 275 s |
| P1 | generators | 6 curves × 100 | 15,000 | yes | 449 s |
| P1 | shift | 5 sources × 104 | 13,000 | yes | 320 s |
| P0 | class | 45 + 3 + 1 + 2 + `fold-aut` = 52 | 5,200 | yes | 68 s |
| P0 | generators | 4 × 100 | 10,000 | yes | 187 s |
| P0 | shift | 4 × 45 | 4,500 | yes | 77 s |
| B1 | class | 40 + 3 rep + 1 + 2 = 46 | 4,600 | yes | 218 s |
| B1 | generators | 6 × 64 | 9,600 | yes | 466 s |
| B1 | shift | 5 × 40 | 5,000 | yes | 246 s |
| K | class | 11 + 3 rep + 1 + 2 + `fold-frob` + `fold-frobneg` = 19 | 1,900 | yes | 218 s |
| K | generators | 3 × 64 | 4,800 | yes | 563 s |
| K | shift | 3 × 11 | 825 | yes | 91 s |

## Replay

Each class mode re-run with `--trials 5` (`<class>/replay/`): P1 550/550,
P0 260/260, B1 230/230, K 95/95 iteration counts identical to the recorded
`class/trials.csv` rows with the same arm and seed.

## Files

`<class>/walk.json` (the walk: class arithmetic, primes, every vertex with
its model, levels, conductor, stratum class number, neighbours, edge
record with kernel polynomial, transport cost, carried instances),
`<class>/walk.log`; `<class>/<mode>/results.json` (every arm's summary,
tests, groups, per-vertex analyses, shift matrix), `trials.csv` (one row per
solve: arm, kind, vertex, source, generator, seed, iterations, restarts,
distinguished points, escapes, ns), `<mode>.log`; figures (`volcano-ell*.svg/png`
with their `.diagram.json`, `volcano3d-ops`, `volcano3d-e2e`, `per_curve`,
`per_group`, `z_scores`, `generators`, `generator_means`, `shift_matrix`)
drawn by `../../plot.py`; `tables.md` by `../../make_report.py`;
`search/` the curve-search records.
