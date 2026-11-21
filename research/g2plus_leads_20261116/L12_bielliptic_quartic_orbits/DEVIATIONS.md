# L12 deviations from PROTOCOL.md / SPEC.md (executor record)

PROTOCOL.md and SPEC.md were not edited (`frozen.sha256` re-checked after all runs: unchanged). No threshold, arm,
seed rule, prime, decision rule or instance count was changed. Items below are where the literal specification
was not executable, ambiguous, or silent. All numbers are `[pcc:unverified]`.

## D1. Attempt-success test includes the uncounted certificate (SPEC 8.3 / 8.4) -- substantive

Literal rule: an LA attempt succeeds iff `Av(x) == b`. Observed during the first end-to-end test (SPEC test 14,
F1 p=263, k=1, s=1, arm C): the attempt succeeded at attempt 1 but certification failed on 2 columns.
Dense rank of the final row set mod r (done in a scratch script, not part of the deliverable): arm C had rank 92
of 94 columns, nullity 2 (one is the scale freedom). The extra kernel vector was supported on two columns
(41, 68) that each occur in exactly one common row, so the stop rule "every column covered by at least one row"
does not force a unique solution, and Wiedemann on `A^T D A` with `y = A^T D b` returns a solution of a singular
consistent system without noticing. The frozen G6 gate (certification) would then fail for correct-by-spec runs.

Minimal change made: an attempt counts as successful only if `Av(x) == b` AND the uncounted certificate of
SPEC 8.4 passes (`certify` in `code/ic_run.py`). A certificate failure is handled exactly like any other failed
attempt (odd attempt: retry same rows with the next LA seed; even attempt: resume collection by
`ceil(0.05 ncols)+1` rows; at most 12 attempts). Every failed attempt is charged in `LA_FPM`; the certificate
itself is not charged. Applied identically to every arm. New output field `cert_rejected_attempts`
(count of attempts rejected only by the certificate), reported in RESULTS.md.
Effect on the data: cells with `la_attempts > 1` exist for arms A, B, C (RESULTS.md table 1, columns la_attempts and
cert_rejected_attempts). Consequence for G6: with this change an arm run with status `ok` is certified by
construction; G6 failures can only appear as `la_fail`/`trial_cap`. One run did: F1 p=523 j=0 s=2 arm C ended
`la_fail` after 12 attempts (12 certificate rejections); dense rank of its final matrix (R=324, ncols=202, r=257) was
200, nullity 2, kernel columns 138 (covered by 1 row) and 62.

## D2. G6 evaluation in analyze.py

`analyze.py` evaluates G6 over all non-`not_run` runs of arms A, B, C, Bp (F1) and A, B, C (F0): a run with status
`la_fail` counts as uncertified (G6 false). The mechanical verdict then follows PROTOCOL section 9 ("invalid when a
gate other than G3 fails"): `decision.json` contains `"verdict": "invalid"` because of exactly this one run. It was not
re-run, patched or excluded. No instance, arm or seed was changed after data.

## D3. Interface details not fixed by SPEC

- `verify_relation(curve, points, i1, i2, terms)` takes the point list and indices (SPEC wrote `(curve, P1, P2, terms)`
  while defining `terms` as point-index keyed; the verifier needs coordinates).
- `la.attempt(rows, ncols, j0, r, ss, a, ops)` is one Wiedemann attempt; `la.solve_fixed` is the fixed-row retry loop used by
  unit test 11; resume-collection logic lives in `ic_run.py` (SPEC 8.3).
- `la.bm`: Berlekamp-Massey as in SPEC; `Cp` is zero-padded to length `L+1` after an `L` update so index reads
  `Cp[i]`, `i <= L`, are always defined (counts unchanged: `ops.mul += L` per discrepancy, as specified).
- Matrix-vector products are computed with `sum(map(mul, ...))` over precomputed row/column lists; counted
  `ops.mul` is added by the SPEC formulas (`W`, `R`, `N`), so counts are independent of this implementation detail.
- G1 in `analyze.py` ("select exit status"): evaluated as `instances.jsonl` has 32 lines and `raw/select.log` has no
  `INSTRUMENT ERROR`; the select command's exit code 0 is in `raw/COMMANDS.log`.
- G0 in `analyze.py` reads `raw/unittest.log` next to `--summary`: last line containing `OK` must start with `OK`.
- Rho (`rho_run.py`): `group_ops` = number of `eadd` calls charged (setup `M_j`, start point and walk steps);
  the points of `E` for choosing `G` come from an uncounted brute-force scan over `(x, w)`.
- Generic control (`generic_run.py`): the `SQRT` table is built on a scratch `Ops` (setup is not charged since the
  control is "collection only"); powers of `x1` by repeated `F.mul` start at `g[0][jj] * 1` (that product is counted);
  the double-root branch charges 1 inv + 1 mul as in 7.2.
- `code/runlog.sh` (copy of the wrapper used) appends each protocol command, its exit code and the sha256 of its
  outputs to `raw/COMMANDS.log`; the protocol commands themselves were run unchanged, wrapped by it.
- `code/make_results.py` builds `raw/results_tables.md` and `plot.svg` from `raw/` (not in the SPEC file table).
- Added unit tests beyond SPEC section 10 numbering: none removed; test 3 cross-check uses the three listed curves.

## D4. Resource limits

No instance size was reduced. Every run finished inside `ulimit -t 600 -v 2000000` (273 of 273 runs returned 0;
0 `not_run`). Largest run by wall time: F0 p=2069 j=1 s=1 arm A, 59 s; all 273 run files sum to 528 s (informational only).

## D5. Hooks

No repository hook refused any command; `raw/HOOK_REFUSALS.txt` was therefore not created.
No `research-kb` lookup was made (the protocol itself records the tool as not installed); no novelty claim is made.
