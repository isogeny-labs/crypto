# Curve25519-family index calculus: the autolab improvement loop

**Class:** an automated optimisation loop (the owner's success criterion made
mechanical), with its ledger of incumbents and its rounds. Not itself a result;
each round's `README.md` and `RESULT.json` carry that round's result, and the
ledger carries the ratchet. Development evidence only: no round is a leaderboard
or `benchmarks/README.md` row until it passes the validation gate.

The driver is native (`src/bin/ic_autolab.rs`; AGENTS.md "Implementation
language: no Python"). Every measurement is an ecbench session: each solve
checked against its target by ecbench, same-host replays, the §5a pricing
band, paired comparisons. Under AGENTS.md "Sabotage is assumed" these are the
producer's own checks; a round's result is cited by its pcc claim id with its
standing (see Rounds). The driver
writes specs, calls `ecbench`, reads its JSON back, classifies under the rule
frozen in each round's `PROTOCOL.md`, and keeps `ledger.json`. It computes no
ECDLP result itself.

## The loop

```
ledger.json ──► propose ──► freeze ──► run train ──► select ──► run holdout ──► assess ──► ratchet ──► report
(incumbents,    panel       PROTOCOL   sessions/     holdout    sessions/        RESULT     ledger      README.md
 trials,        .json       specs/     train-k*      specs      holdout-k*       .json      .json       round.svg
 exposures)                 MANIFEST   verify+band              verify+band                 ratchet.svg REPORT.pdf
```

| step | what it does | rule it enforces |
|:--|:--|:--|
| `propose` | candidate arms per size from the incumbent's base-size neighbourhood, refinement of alternatives within 1.5x of the incumbent, and exploration slots for families never tried at that size; never a configuration already in the trial history | autolab portfolio: total-cost leaders, Pareto alternatives, predeclared exploration (`AUTOLAB.md`, "Iterate without discarding useful alternatives") |
| `freeze` | `PROTOCOL.md`, train specs, `MANIFEST.json` (sha256 of everything), fresh target seeds disjoint from every exposure; `ecbench plan` validates each spec first; refuses an existing round | protocol before data; no reuse of exposed targets |
| `run` | `ecbench run` per size into a new session directory, `ecbench verify --replay`, `ecbench band --json --ranks`; a failure is recorded, never retried | verified solves, replay audit, every arm retained |
| `card` | the same stage as a frozen-command task card for the director/executor relay (`tools/task_card.py lint`) | AGENTS.md "Model tiers" |
| `select` | one challenger per size: lowest mean S at w = 0 among candidates whose every run verified, and only if below the incumbent in train; writes the holdout specs; appends the dated amendment | disclosed in-sample selection; fresh holdout |
| `assess` | `ecbench compare --a inc --b sel` at w = 0 (and 0.1, 1), the A/A `inc-aa`, the band order; classifies improvement / regression / null / inadmissible | AGENTS.md success criterion and §5a |
| `ratchet` | an improvement becomes the incumbent with its delta, session id, spec and receipt sha256; all arms join the trials; all seeds join the exposures | ratchet; cumulative, visible progress |
| `report` | the round's README, `round.svg`, the loop's `ratchet.svg`; `scripts/render_pdf.sh` makes `REPORT.pdf` | research-visuals |

`loop --rounds N` runs the whole sequence on the local route. Each round is
write-once: a second `freeze`, `select`, `assess` or `ratchet` of the same
round is refused, and session directories are never rerun.

## Tracks and families

`ledger.json` keeps one incumbent per **track** (AGENTS.md "Tracks of index
calculus": generic decomposition and algebraic) and **size** (k = 20, 24, 28,
32 bits of p; the miniatures of `research/curve25519_ic_baseline_20261102`,
built by Curve25519's own selection rule). A gain in one track never ratchets
the other. `families.json` lists the candidate families per track: each is an
existing ecbench `ic.pipeline` oracle on the track's factor-base template, and
the searched knob is the base size N (the tower height t on the algebraic
track). A family that needs new code is a source change: build it, add it to
`families.json`, and the next round's exploration slot picks it up.

The seeded incumbents (round 0) and their provenance are in
`ledger.seed.json`; `ic_autolab init --from ledger.seed.json` wrote
`ledger.json` from it. Status at any time: `ic_autolab status`.

## Running a round

```bash
cargo build --release --bin ecbench --bin ic_autolab
H=research/curve25519_autolab_20261104
# one round, generic track, k = 20 and 24, local route (an explicitly requested development screen)
./target/release/ic_autolab --home $H loop --rounds 1 --track generic --sizes 20,24 --cpus none --allow-busy
$H/scripts/render_pdf.sh $H/round-001
```

Step by step, or through the model-tier relay (the lab's default route):

```bash
./target/release/ic_autolab --home $H propose --round 2 --track generic --sizes 20,24 --out /tmp/panel.json
./target/release/ic_autolab --home $H freeze  --round 2 --panel /tmp/panel.json --route eks --ecbench target/release/ecbench
./target/release/ic_autolab --home $H card    --round 2 --stage train        # -> round-002/cards/CARD-...yaml
python3 tools/task_card.py lint $H/round-002/cards/CARD-*.yaml                # experiment-director lints, executor-mechanical runs
./target/release/ic_autolab --home $H select  --round 2 --ecbench target/release/ecbench
./target/release/ic_autolab --home $H card    --round 2 --stage holdout --n 2
./target/release/ic_autolab --home $H assess  --round 2 && ./target/release/ic_autolab --home $H ratchet --round 2 && ./target/release/ic_autolab --home $H report --round 2
```

Before any run: `scripts/checkpoint.sh init` and fill `.resume/RESUME.md`
(CLAUDE.md "Resume checkpoints"). The algebraic track's arms take minutes per
target: use `--train-targets 6 --holdout-targets 12 --timeout-seconds 7200`.

## Decision rule (every round, frozen in its PROTOCOL.md)

Per size, from the fresh holdout: **improvement** if the paired `sel / inc`
95% interval is below 1 at w = 0 and `ecbench band --ranks` orders
`sel < inc` (the order holds at w = 0, 0.1 and 1 alike), with the A/A interval
containing 1; **regression** if the interval is above 1 or the band orders
`inc < sel`; **null** otherwise. Relabelling (a headline count falls while
total cost rises) is caught by the band. Distance to rho,
`S(sel) / S(rho-neg)` at each weight, is reported as the distance remaining.
The class names are labels of this rule, not claims; a round's outcome is
cited by its claim record [pcc:unverified].

## Rounds

| round | track | sizes | outcome | report |
|--:|:--|:--|:--|:--|
| 1 | generic | 20, 24 | no challenger at either size: the `ksum3` incumbents (N = 11, 20) beat their N neighbourhood and the `mitm2` / `mitm3` exploration arms in train | [round-001](round-001/README.md), [PDF](round-001/REPORT.pdf) |
| 2 | generic | 20, 24 | k = 24: **improvement**, `ksum4-n12` (`ksum:m=4`, N = 12) replaces `ksum3-n20`: paired sel / inc 0.917 [0.879, 0.957] at w = 0 (0.911 at w = 0.1, 0.880 at w = 1), band `sel < inc`, A/A exactly 1, 480/480 holdout runs verified (session `ECBS1h32d89e6cb3e5`); distance to rho 6.5x -> 6.0x. k = 20: no challenger (`ksum4-n5` 3.9x worse, `ksum5-n4` exhausted on 16/16 runs, `mitm2` refinements worse) | [round-002](round-002/README.md), [PDF](round-002/REPORT.pdf) |
| 3 | generic | 20, 24 | no challenger at either size. k = 20: the cross-size transfer of `ksum:m=4` came closest (`ksum4-n10` 10.29 vs inc 8.70 at w = 0, but 13.37 vs 15.56 at w = 1; band unresolved), `ksum4-n7` 10.95, `rootfind-mitm:m=2` 11.25. k = 24: `ksum4-n12` is a sharp optimum (N = 8 at 2.3x, N = 17 at 2.6x, N = 6 and 24 far worse); `rootfind-mitm:m=2` 12.17; `rootfind-pairs:m=3` costs 70x (k = 20) and 320x (k = 24) the incumbent | [round-003](round-003/README.md), [PDF](round-003/REPORT.pdf) |
| 4 | generic | 28, 32 | k = 28: `ksum4-n21` (`ksum:m=4`, N = 21) replaces `mitm2-n203` under the frozen rule: holdout `ECBS1h4fa56ebd629d`, paired sel / inc 0.654 [0.636, 0.672] at w = 0, band `sel < inc`; S 16.74 to 10.94, distance to rho 16.5x to 10.8x. k = 32: no challenger; `ksum4` at the transferred N = 53, 76, 106 costs 4.4x to 72x the incumbent, so the m = 4 optimum N grows more slowly than N_a (12 at k = 24, 21 at k = 28) [pcc:unverified] | [round-004](round-004/README.md), [PDF](round-004/REPORT.pdf) |
| 5 | generic | 32 | `ksum4-n26` (`ksum:m=4`, N = 26) replaces `mitm2-n512` under the frozen rule: holdout `ECBS1hbc72fda3e97f`, paired sel / inc 0.352 [0.340, 0.363] at w = 0 (0.291 at w = 1), band `sel < inc`; S 23.60 to 8.30, distance to rho 25.9x to 9.1x. The proposer's log-log fit put N at 37; the train optimum was 26 (N = 19 at 9.05, 37 at 25.2) [pcc:unverified] | [round-005](round-005/README.md), [PDF](round-005/REPORT.pdf) |
| 6 | generic | 28 | null: `ksum4-n15` costs 0.616 [0.593, 0.641] of the incumbent at w = 0 but 1.092 [1.039, 1.149] at w = 1 (holdout `ECBS1h584855d3a3bb`), so the band does not order it and `ksum4-n21` stays; `ksum3-n35` 8.91 vs 10.87 in train, also higher at w = 1 [pcc:unverified] | [round-006](round-006/README.md), [PDF](round-006/REPORT.pdf) |
| 7 | generic | 20 | regression: the train leader `ksum4-n8` (8.18 vs 9.31) costs 1.026 [0.966, 1.090] of `ksum3-n11` on the fresh holdout `ECBS1ha3ed7f4ce6e6`, band `inc < sel`; a winner's-curse selection on 16 targets, kept on record. `ksum4-n9` and `ksum5-n8` were lower only at w = 1 in train [pcc:unverified] | [round-007](round-007/README.md), [PDF](round-007/REPORT.pdf) |
| 8 | generic | 28 | `ksum4-n18` replaces `ksum4-n21` under the frozen rule (32 train targets; the finer N grid): holdout `ECBS1hc8e87821dc40`, paired sel / inc 0.678 [0.666, 0.691] at w = 0, 0.869 at w = 1, band `sel < inc`; S 10.98 to 7.44, distance to rho 11.5x to 7.8x [pcc:unverified] | [round-008](round-008/README.md), [PDF](round-008/REPORT.pdf) |
| 9 | generic | 32 | censored: stopped after 10 train records because the panel held `rootfind-pairs:m=3` at N = 179 (see round-009/STOPPED.md); four dominated families disabled | [round-009](round-009/STOPPED.md) |
| 10 | generic | 32 | null: `ksum4-n22` costs 0.889 [0.869, 0.910] of `ksum4-n26` at w = 0 but 1.202 at w = 1 (holdout `ECBS1h572a148f6fbf`), band unresolved; the same counted-work vs lookups trade as round 6 [pcc:unverified] | [round-010](round-010/README.md), [PDF](round-010/REPORT.pdf) |
| 11 | generic | 24 | no challenger on 32 train targets: `ksum4-n12` 6.61, `ksum4-n10` 6.96, `ksum3-n20` 7.82, `ksum4-n14` 9.00 [pcc:unverified] | [round-011](round-011/README.md), [PDF](round-011/REPORT.pdf) |
| 12 | generic | 24 | no challenger; tests proposal P1 (`targets=stride`): the incumbent configuration on the stride stream costs 7.41 vs 6.86 on the jump walk (1.08x), so P1's predicted 0.84 is falsified [pcc:unverified] | [round-012](round-012/README.md), [PDF](round-012/REPORT.pdf) |
| 13 | generic | 20 | null: the first free-rows arm `ksum4f-n8` (`ksum:m=4,free=1`) costs 0.971 [0.931, 1.012] of `ksum3-n11` on holdout `ECBS1ha91af1cc9a88`, band `sel < inc`, interval spans 1; in train free rows took ksum4 at N = 8 below the incumbent (8.28 vs 9.04) and ksum5 at N = 8 to the lowest w = 1 cost (13.40) [pcc:unverified] | [round-013](round-013/README.md), [PDF](round-013/REPORT.pdf) |

Claim records (AGENTS.md "Sabotage is assumed: proof-carrying claims"): round 1
`PCC-20261104-6ca5d2`, round 2 `PCC-20261104-fa70c6`, round 3
`PCC-20261104-31cd98`, round 4 `PCC-20261104-6c0998`, round 5 `PCC-20261104-0213f5`, round 6 `PCC-20261104-249a57`, round 7 `PCC-20261104-15a1d6`, round 8 `PCC-20261104-ff48c0`, round 10 `PCC-20261104-87cbc0`, round 11 `PCC-20261104-f74cbc`, round 12 `PCC-20261104-4afdff`, round 13 `PCC-20261104-342ea5`, all `statement` claims with standing `unverified` until a
second host signs a replay and a validator signs a recompute [pcc:unverified].

The ledger (`ledger.json`, `history`) carries every ratchet with its delta and
evidence; `trials` holds every arm ever measured (60 after round 5) and
`exposures` every target seed (35).

![ratchet](ratchet.svg)

## Deviations and limits (stated, not hidden)

- The local route is counted units at L0/L1 on a shared host: deterministic,
  replayed, no wall-time claim. The lab's default is Iso Lab on EKS through a
  task card (`card`); a second-host replay is required before any leaderboard
  row.
- Selection in train is in sample and disclosed; the holdout is fresh.
- The candidate space is configurations of existing methods. Source changes
  (a new oracle, a new table layout, linear-algebra or walk changes) are the
  next lever and need their own build and family entry.
- `ic sweep` (docs/ic/SWEEPER.md) is the TPE search over planted instances;
  this loop is the frozen-protocol confirmation on fresh public targets that
  a sweep's hypothesis has to pass. The two can be chained: sweep to propose,
  this loop to confirm and ratchet.
