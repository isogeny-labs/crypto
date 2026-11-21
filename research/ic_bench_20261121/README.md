# ic-bench: benchmark harness for index-calculus relation finders

A solver-agnostic harness for the point-decomposition (relation-finding) step of
index calculus on binary curves: LRF root finding, enumeration, meet in the
middle, a plain-Python reference, and MutantXL. It also has a cost model
against Pollard rho (ECC2K-130 presets), a protocol whose predictions were
fixed before the runs, calibration records, and frozen fixtures with their
expected solution sets.

Moved here from isogeny-labs/lfd-summation-systems PR #43, which had opened it
in the wrong repository. The code and data are unchanged. See `PROVENANCE.md`.

## Layout

- `csd/bench/` is the harness: specs, solver adapters, an isolated runner with
  timeouts, RSS and mismatch checks, statistics, the cost model, grids and
  fixtures.
- `csd/*.py`, `csd/*.c` are the solver modules the harness wraps. They are
  vendored from lfd-summation-systems at a pinned commit (`PROVENANCE.md`).
- `scripts/bench.py` is the CLI. `scripts/run_ic_rootfind.py` is the original
  instance driver, kept because the tests check seed compatibility against it.
- `experiments/ic-bench/` holds `PROTOCOL.md`, `RESULTS.md` (C1/C1b/C2, with
  falsified predictions listed), calibration JSONL and fixtures.
- `experiments/ic-rootfind/` holds the reference records that the fixtures and
  the cost model read.

## Use

Run these from this directory. You need Python 3 with numpy, and a C
compiler for the C kernels.

    python3 -m pytest -q tests                          # 29 tests
    python3 scripts/bench.py run --grid smoke --solvers py_ref,enum,lrf --out /tmp/smoke.jsonl
    python3 scripts/bench.py summarize /tmp/smoke.jsonl --baseline enum
    python3 scripts/bench.py fixtures verify
    python3 scripts/bench.py model --n 131 --m 3 --preset ecc2k130

## Status in this repository

This is a measurement tool. No row in it reaches the IC leaderboard or
`benchmarks/README.md`. Those rows need the validation gate (replay
certificate, validator report and matched rho at the §5a band) and
`bench.record/v1` records, as AGENTS.md requires. The calibration records
were produced on a 4-core Xeon development box, not on isolab-eks. They are
[pcc:unverified] assertions until someone re-runs them through the repo's
runners.
