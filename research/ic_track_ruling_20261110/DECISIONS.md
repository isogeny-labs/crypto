# Owner ruling, 2026-11-10: only algebraic index calculus counts

The owner gave this ruling in the claude.ai session of 2026-11-10, in their own
words: "Algebraic index calculus is what we want to only allow", and "Index
distance [to] rho is not the only measure." It supersedes, for every
index-calculus ratchet, leaderboard and autolab in this repository:

- the two-track Option C of
  [`research/prime_ic_program_20261029/DECISIONS.md`](../prime_ic_program_20261029/DECISIONS.md) R1;
- the "Generic decomposition" track of `AGENTS.md`, "Tracks of index
  calculus" (2026-10-29).

The ruling is not a judgment on the generic results. They stay as committed
evidence. It is a statement of what the programme is trying to improve.

## Why

On the Curve25519 miniatures the autolab ratcheted the generic track for 37
rounds. Its incumbent became a factor base of one or two abscissae inside a
random walk (`wwalk`, `twcj`). Its own studies call that "one unknown away
from rho" and not decomposition index calculus
(`research/curve25519_tw_scaling_20261108/README.md`,
`research/curve25519_f1_targetwalk_20261108/README.md`). The algebraic track
got one seeded incumbent at one size and no rounds. The loop optimised what
it was pointed at: the lowest `S` of any arm.

## R1. Admission: algebraic only

An arm is index calculus only when its relations come from a **solver
applied to a system that contains the summation polynomial and the
factor-base constraint**. Examples:

- ⟨S_{m+1}, F_V⟩ handed to F4 or msolve;
- Weil descent over a subspace base;
- a PKM tower system solved by F4.

Generic decomposition does not count. That covers enumeration, pair, k-sum
and coefficient-box tables, random walks, root finding by group addition, and
BSGS-boundary methods. Arms of that kind may appear in a table only as
**references**, labelled so. They never hold an incumbent and never ratchet.

**Mechanical form.** `src/bin/ic_autolab.rs` enforces the ruling:

| item | content |
|:--|:--|
| `ALGEBRAIC_ORACLES` | `pkm-semaev` with engine `f4`; `descent-algebraic` and `descent-algebraic-fiber`; `pdp3-koblitz` with engine `inherited-f4` or `f6-ic`; `gaudry-x-bin` |
| `GENERIC_ORACLES` | named so that a refusal says why |
| `check_track` | refuses the retired `generic` track in `propose`, `freeze`, `select`, `assess`, `ratchet` and `loop` |
| `admit_arm` | refuses any non-algebraic incumbent or candidate at `freeze`, and any non-algebraic `sel` at `assess` |
| `oracle_admission` | refuses an oracle that is on neither list until it passes the random-base test below |

Three more ways an arm can enumerate while looking algebraic are refused:

- **Enumerating engines.** Exhaustive and root-finding engines of an
  algebraic oracle (for example `pkm-semaev:engine=exhaustive` or
  `pdp3-koblitz:engine=rootfind`) enumerate instead of solving.
- **Solver bounds.** `pkm-semaev` with a lowered `degree`, or with `max_nnz`
  or `staircase`, decides undecided systems on the grid, which is
  enumeration with its multiplications unpriced. Its other parameters are
  allowlisted (`PKM_PARAMS`).
- **Arm parameters.** An arm's further parameters are allowlisted
  (`ARM_PARAMS`: `targets` = `walk` or `stride`, `linalg`, `max_trials`).
  A `params` entry named `oracle` or `factor_base` would replace the
  admitted oracle in the emitted method, so `admit_arm` checks the method
  the arm actually emits.

Two further checks apply at `assess` and `ratchet`:

- `assess` marks a `sel` inadmissible when it hands more than 1% of its
  solver calls to the grid, and more than the incumbent does
  (`grid_fallback`, from the records).
- `ratchet` re-checks admission before it installs a new incumbent.

**The random-base test**, for adding an oracle to `ALGEBRAIC_ORACLES`
(classification Option B, `research/prime_ic_program_20261029/ruling/README.md` §5.1):

1. Run the oracle on its algebraic factor base and on a random base of the
   same size, on the same targets, under a frozen protocol.
2. If it runs, and its relation yield and solver work per attempt match the
   random base within the paired 95% interval, it is generic and is refused.
3. If it needs the algebraic structure to run at all (a tower, a subspace or
   a Weil restriction), record that, with the code path, as the test result.

Record the result in a dated amendment to this file before the oracle joins
the list.

## R2. Measures: distance to rho is reported, not optimised

Each algebraic arm is judged against the **algebraic incumbent** at its size,
under the AGENTS.md success criterion. The paired `sel/inc` interval at
`w = 0` and the band order at every weight decide the class, as before.
Beside the class, `assess` now reports, from the session records:

| measure | what it answers |
|:--|:--|
| relations per decomposition attempt | does the factor base or system yield more? |
| solver work per call (F_p multiply-adds, elimination only) | is each solve cheaper? |
| paired `sel/inc` ratio of solver work per call, 95% bootstrap interval (pairs by target seed, target index and round; refuses duplicate keys) | a **stage improvement**, reported as one (it ratchets only once it moves end-to-end `S`) |
| attempts per relation, paired | |
| share of calls that hit the solver budget | censoring |
| largest solving degree seen | the fall-degree question, measured in the loop |
| share of counted cost per phase | where the cost lives |

`status` and every `RESULT.json` also report the **growth exponent** of the
track's incumbents. Total cost grows as r^e with e = 1/2 + the slope of
log₂ S against log₂ r. It is fitted at each weight w = 0, 0.1 and 1, with
the matched rho's exponent beside it. Two cautions apply:

- At w = 0 the solver's native work is unpriced, so w = 1 is the exponent
  that prices it.
- A fit over fewer than three sizes, or across incumbents of different
  families, is flagged as indicative only. That holds for the current
  ledger: two sizes, from different studies, with rho drawn from 6 and 2
  targets.
The programme's question is whether e can be brought below rho's. The
distance to rho stays in every table, as the scope of the comparison, and is
not the objective.

## R3. The ledger

- The generic track's incumbents stay in `ledger.json` as history. `status`
  prints the track as RETIRED.
- The algebraic track is seeded at k = 24 from the scale-up study's bridge arm
  `pkm-iso-t6-g4`, 2 of 2 finished
  (`research/curve25519_algebraic_ecbench_scale_20261105/README.md`, Table 1).
  Its S at w = 0.1 and 1 are published to three significant figures. Its rho
  is derived as S divided by the published "x rho-neg" ratio on the same
  chunks, so it is rounded too.
- k = 28 and k = 32 have no algebraic incumbent. Section R4 lists the
  blockers.

## R4. Work queue, in order

| # | item | why | where |
|:--|:--|:--|:--|
| Q1 | Run the algebraic autolab at k = 20, 24 (`--track algebraic`, now the default) | the ratchet the programme wants | `research/curve25519_autolab_20261104/` |
| Q2 | Rerun the Curve25519 exponent study at m = 3, 4 with real resources | it was censored at 3.5 GB / 1,200 s, so its verdict is "undetermined"; it is the direct measurement of the growth exponent | [`research/curve25519_exponent_rerun_20261110/PROTOCOL.md`](../curve25519_exponent_rerun_20261110/PROTOCOL.md) |
| Q3 | F4 tower engine past p ≥ 2^31, and a wall budget that lets k = 28 finish | without them the algebraic track cannot reach k = 28, 32, and no growth exponent over four sizes exists | engineering; see Q2's protocol, §Blockers |
| Q4 | Certify last-fall-degree instances at n = 33–51 (E13/E14 failed to complete) | the strongest theory result, and the paper | [`backlog/2026-11-10-fall-degree-certificates.md`](../../backlog/2026-11-10-fall-degree-certificates.md) |
| Q5 | Finish the k = 4 characteristic-two m = 18 rho run and replay the sessions | the only measured IC crossover candidate on record (weak family) | [`backlog/2026-11-10-k4-char2-m18-completion.md`](../../backlog/2026-11-10-k4-char2-m18-completion.md) |

Items Q2–Q5 run on isolab-eks under their frozen protocols (AGENTS.md 9a).
The session that wrote this file has no AWS identity, so it ran none of them.

## Not changed

- The validation gate, the §5a band and the matched-rho rules.
- Committed generic evidence, and the treatise chapters that report it.
- The sweeper code (`src/bin/ic/sweep/space.rs`). It already keeps its
  `table` and `unclassified` spaces unranked against the algebraic track.
  `docs/ic/SWEEPER.md` now says those spaces are references, not counted.
