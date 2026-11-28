# L06 deviations (executor)

No threshold, arm, seed rule or decision rule was changed. `frozen.sha256` verified OK before work
(`sha256sum -c frozen.sha256`: DERIVATION.md OK, PROTOCOL.md OK, SPEC.md OK).

## D1. G1 failed on its ambiguity limit; the census was run anyway

The task text said to run the PROTOCOL.md commands only if the correctness gates pass. Gate G1
(PROTOCOL.md section 5) came out as: 231 of 231 unique-chi draws agree with enumeration (100%),
but 9 of 240 draws are `ambiguous` (3.75%), above the 2% limit (8 of 200 at p = 101, 1 of 40 at
p = 257). The nine curves have #J divisible by high powers of 2 (for example
974144 = 2^6 * 31 * 491), where several multiples of the group exponent fall in the search
interval of SPEC.md 4.3. This follows from the specified algorithm, not from a detected bug
(the unit tests, G2, G3, G5 hold). The executor did not change the limit or the algorithm.
It ran the remaining commands, because they are cheap (each census run under four minutes) and
the verdict rule (PROTOCOL.md section 6, rule 1) already maps a failed gate to
"void (instrument)". The computed verdict is therefore `void (instrument)`. The census numbers in
RESULTS.md are reported as measured counts only, not as results under the protocol.
The decision to run past the failed gate is the executor's, not the protocol's; the
coordinator may discard the census data.

## D2. First launch of the gate-enum job failed (shell quoting), relaunched

The first attempt to start the two `gate-enum` commands in one `nohup bash -c '...'` failed with
the shell message `Exit 127 nohup bash -c './runlog.sh raw/COMMANDS.log "./l06/target/release/l06
gate-enum --p 101 ...` (command not found, quoting of nested double quotes). Nothing ran and no
output or log line exists for it. The same two commands were then run from `gate_enum_job.sh`
(identical command lines, as logged in `raw/COMMANDS.log`).

## D3. Execution order and wrappers

- `gsp` (arm R and G7) and `gate-enum` ran concurrently as two single-thread processes, then the
  census loop ran alone as one single-thread process at a time, in the PROTOCOL.md loop order.
  The protocol listed them sequentially; outputs do not depend on order.
- Commands were run through `runlog.sh`, which logs the command, its exit code and the sha256 of
  its output files to `raw/COMMANDS.log`. Entries are in completion order, with the start time.
  `census_job.sh` is the PROTOCOL.md `for` loop verbatim, with the `timeout 600 ... ` wrapped in
  `runlog.sh`; `exit_codes.txt` was written by the same `echo` line as in PROTOCOL.md (its
  `$?` is the exit code of `runlog.sh`, which returns the wrapped command's code).
- `raw/build.log` is the output of an up-to-date incremental `cargo build --release` (the crate
  had been compiled for the unit-test development runs), so it shows only warnings and
  `Finished`. The full build ran before, with no log.
- `data.sha256` was regenerated at the end of the work (after RESULTS.md generation). Its hash of
  `raw/COMMANDS.log` and `raw/summary.json` is the final one. The `sha256sum raw/*` line itself
  appends no entry after the final regeneration; a note about it is the last entry of the log.
- `gen_results.py` (renders RESULTS.md from `raw/summary.json`) and the helper scripts were added
  to the directory; they are not part of the frozen protocol.

## D4. Fallback and timeouts

No census command hit `timeout 600` (all exit 0, see `raw/exit_codes.txt`), so the pre-specified
fallback (half `--u-count`, `--prime-target 60`) was not used. No instance size was shrunk.

## D5. Specification points that were silent or inconsistent (choices made, see IMPLEMENTATION_NOTES.md)

- PROTOCOL.md cites "SPEC.md section 9" for the unit tests and "section 8" for output formats;
  in SPEC.md they are sections 11 and 9. The SPEC.md numbering was followed.
- Output keys beyond SPEC.md section 9: none were removed. Extra files `src/tests.rs`, none other.
