# Blind spot 08: learned branching for WDSat / XOR-CDCL on Semaev CNFs

## Verdict: BLOCKED (level: constant; primary test not run)

The frozen decision test (cards 05-10: held-out n = 31, 33, 35, 37, 128 targets,
arms b, b2, c against incumbent a, at 0 / 0.1 / 1 GAE per native operation)
**never executed**. Cards 05-09 were refused by `task_card.py` at lint: each
argv names `raw/model.txt` (written by card 04) but the card's `binds` do not
pin it, so the card cannot be frozen before the file exists. Card 10 ran its
build, then `summarize` failed because `raw/test-n31.jsonl` does not exist
(its inputs are produced by cards 05-08). No receipt exists for cards 05-09.
No ADVANCES/TRADES/MATCHES/REGRESSES claim is made. No gap-to-rho row exists
(no cross-method comparison was reached; §5a pricing at 0/0.1/1 GAE not applicable).

Restart of the builds was requested; the frozen cards cannot be repaired by
this step. Fix needed (director): re-freeze cards 04-10 with the card-04 output
`model.txt` (sha256 00f0d29d6f0535b91ca3ab0bbc1c76dbb19085fce325c5b77aabed0479b60809)
pinned in `binds`, and card 10 with its input files and build product handled
the same way, then re-dispatch 05-10.

## What did run (measured)

| card | content | outcome |
|---|---|---|
| 01 | selftest, rustc build, 60-instance corpus replay | selftest OK; replica == stock WDSat decisions 60/60, verdicts 60/60, certificates 30/30 |
| 02 | training samples n=23, 25 | receipt status completed; `samples-n23/25.txt` on disk (executor relay claimed non-completion; the receipt and files show it finished: relay deviated, reported not repaired) |
| 03 | training samples n=27, 29 | completed |
| 04 | fit ridge (lin) + GBT, validation | completed; `raw/model.txt` |

Note: `samples-n23/25/27/29.txt` are all 78123 bytes (same instance/state
counts per size by design: 42 states per size, depths 0..8); they were not
checked to be distinct beyond differing content by size. Not independently audited.

## Exploratory numbers (card 04 validation, NOT the frozen test)

16 validation instances at n = 23..29 (training-adjacent sizes, 2P+2R each),
mean log2 cost (search-tree nodes, native scorer work not priced):

- identity (incumbent order): 20.2961
- b_lin 19.8471 (-0.449 bits, about 1.36x fewer nodes); b_gbt 20.3625 (+0.07)
- b2_lin 20.5642, b2_gbt 20.6178 (worse)
- cube arms: 19.85 to 20.38 (best: full=gbt dcut=6 k=5 at 19.8457; worst lin dcut=6 k=3 at 20.3788)
- mean oracle gain of the best first variable over the incumbent's first
  variable at the root: 1.0036x (24 root states): the incumbent's first choice
  is already nearly optimal at the root.

No interval was computed, the validation set is the model-selection set (16
instances, one seed), the scorer's native work is unpriced (k = 0 only), and
no shuffled-label control or a2 comparison was run. These numbers are
descriptive and cannot support an advance. A gain of 1.36x by b_lin would
at best be a k = 0 constant-factor candidate, to be tested by the frozen rule.

## Controls and deviations

Planned controls (bs, b2s shuffled labels, d, a2) were never run. Deviations
from the build notes are in PROTOCOL.md (rustc, ell = 5, 16P+16R targets,
native replica arm a). Additional deviation here: relay files for cards 1-3
are abbreviated copies of the executor text (data rows elided), so
`check-relay` reports `relay_verbatim: false` for them; receipts are
authoritative.

## Caveats

Host is L0-level (shared 4 vCPU container, no isolation); toy scale (ell = 5,
n <= 37); CMS 5.11.21 arm (card 09) not run; no wall-clock claims.

## Files

`graph-validation.svg/png` (recorded fit values; source `make_graph.py` +
`raw/fit.txt`), `REPORT.pdf`, `raw/relay-1..10.txt`, `raw/receipt-card-*.json`.
