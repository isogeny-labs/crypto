# Implementation notes (where SPEC.md is silent)

- Python 3.13, standard library only, one thread. Layout as SPEC section 1 (`l01/*.py`, `run.py`, `analyze.py`, `tests/test_l01.py`);
  helper files not in SPEC: `logcmd.sh` (appends command, exit code and output sha256 to `raw/COMMANDS.log`), `run_all.sh`
  (the loops of PROTOCOL section 10 verbatim, each command through `logcmd.sh`), `make_results.py` (tables and `plot.svg` from the raw files).
- Polynomial helpers in `curve.py` follow SPEC 5 literally (coefficient lists, lowest degree first); `k_mul` skips zero coefficients in the
  inner loop without changing the tick (`len(a)*len(b)`).
- `sieve_tuple` calls the counted `F.mul`/`F.sub` once per point; `FD.step` adds `d` to `fd_add` per step (SPEC 7.3).
  The per-tuple count-formula gate (SPEC 7.4) runs in every P_full tuple of every run and never fired.
- `Acc` (rows.py) dedups by `canon`, builds the signed row over base columns, and keeps statistics over all accepted rows;
  `rows_digest` hashes the first `N_target` row lines.
- `DSU` vertex 0 is the no-large-prime vertex; a `(0,0)` edge always counts as a cycle (SPEC 8.4 literal).
- `analyze.py` implements SPEC 10.5 exactly; the verdict code branch follows PROTOCOL section 8; G6 compares every field except
  `cpu_seconds_diag` and the file name.
- `sparse_eliminate` selects the minimum `(weight, column)` by a full scan each pivot; peak non-zeros are measured after each pivot
  (this is what gives `peak_nnz = 6` in unit test 11).
- The CPU-time guard (540 s) is `time.process_time()` checked per tuple (P arms), per 1024 lambdas (P_naive1) and per 256 trials (B arms).
- Time was never read by `analyze.py`.
