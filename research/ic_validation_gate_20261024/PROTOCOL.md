# Protocol (frozen 2026-10-24, before any replay of this study ran)

Conductor task T-2. Scope: the 13 rows of leaderboard Table B and the 9 rows of
Table C in `docs/ic/leaderboard.json` (board entries listed in README.md). Table A
and the v4 run are not in this protocol; v4 is a later job.

This is a validation study, not an experiment. It measures nothing new about IC
or rho. Its outputs are artifacts the validation gate reads
(`docs/VALIDATION_GATE.md`) and a per-row standing. Nothing here is an
improvement; relabelling and accounting changes are not improvements.

## Arms and controls

- Replay arm: the current `ic` binary built from the working tree
  (`cargo build --release --bin ic`), run in a separate process from the one that
  produced the archive, single thread (`RAYON_NUM_THREADS=1`), frozen seed and
  configuration read from the archive's `config` block.
- Independent verifier: pure-Python EC arithmetic over GF(p) and GF(2^n)
  written for this study (`scripts/ec_verify.py`), sharing no code with the repository.
- Independent recompute: `scripts/validate_B.py` and `scripts/validate_C.py`,
  stdlib Python, reading only committed raw records, not the builder and not the
  sweep's jq scripts.
- Positive control: the verifier must accept every archived planted pair.
- Negative control: the verifier must reject recovered scalar + 1 for every
  distinct run (must fail on all of them), and the recompute must flag a
  deliberately perturbed claim (S_IC times 1.001) as a mismatch.
- A/A: the replay is compared with the archive field by field; fields that are
  clocks (`wall_ns`, `elapsed_seconds`, measured calibration) are excluded and
  named in the certificate. Every counted field is compared exactly.

## Decision rule (per row; all four must hold to write a passing VALIDATION.json)

R1 Replay. Every recorded run re-executed on the current binary reproduces
   its counted fields exactly (`total_gae` of each of the 27 IC variant
   repetitions per instance, the recovered scalar, `verified`, each rho run's
   `gae` and counters) and the matched rho re-priced by `ic rho --reprice` on the
   replay's own output reproduces the archived per-run `gae`. One mismatch fails
   the row; a mismatch is recorded with its field and values.
R2 Scalars. For every distinct recorded run, `[d]G` equals `[t]G` for its planted
   target t, and `[r]G = O`, by the independent verifier; scalar+1 is rejected.
R3 Recompute. S_IC, S_reference, x reference and x floor recomputed from the
   raw ledger and re-priced rho runs agree with the board row to relative 1e-9
   (the board stores 12 significant digits).
R4 Rho reference, all of: names the protocol's reference; is the strongest
   available for the curve (negation is the only automorphism of prime and
   random binary curves: the archive's `exclusions` say so); same unit (GAE per sqrt r);
   same binary or process (the replay binary runs both arms and reproduces both);
   and the section 5a band at weight w = 0, 0.1, 1 per unpriced native
   operation, applied to both arms. The band uses the repository's own formula
   (`ecbench/band.rs` `priced_gae`: G - C + w(U + n_C), C and n_C the GAE and count
   of the calibration-priced natives, U the arm's own unpriced counts) and a
   conservative extension that also charges IC's pair-table inserts and rho one
   table operation per walk step (an upper bound, since rho does not emit
   its table counts). `order_holds_across_band` is true only if IC stays above rho
   at every weight in both versions.

Outcomes. All four hold: validated, VALIDATION.json written (or staged, see
README, where the evidence directory is under `docs/ic/`). Any fails: no
VALIDATION.json; the failing criterion is recorded in README.md. A pass says
the numbers are what the archive says and the reference is fair. It does not
say IC is competitive with rho: every Table B and C row is above its matched
rho, and the gap is reported as distance remaining.

## Table C expectations, stated before looking at a replay

The board's Table C IC figure is `s_before / (paired wall-time speedup)` and its
reference is batch rho from ledger section 20 (a different study, 2026-09-26)
priced at the canonical step. R4's "same binary or process" and "same unit" are
therefore expected to fail for Table C unless the committed evidence holds a
same-process rho arm for the row. The replay (R1) and recompute (R3) still run
for Table C and are reported; they decide whether the numbers are what the archive
says, not whether the comparison is admissible.

## Known-issue question (the sweep's "96 mismatches")

The sweep says the 96 root-finder replay mismatches are inherited-F4
`solver_ops` rows rather than root-finder rows. Test: current-binary
`ecbench verify replay-all` on the root-finder session, mismatches grouped by
arm and field. Table B and C rows read `ic boundary` / `ic price` oracle counts,
not ecbench solver counters; R1 tests that directly.

## Table C replay sample (frozen before the draw)

Seed string `ic_validation_gate_20261024`. For each of the nine curves, list the archived main-comparison
price files `runs-isolated/main/k{a}n{n}/M{j}/r{i}-{baseline,candidate}.price.json` (first attempt of
each slot only, no retries), key each run `sha256(seed|k{a}n{n}|M{j}/r{i}-{arm})`, take the 3 smallest
keys. No run is dropped or swapped after the draw. Each is replayed with `ic price --params <the
run's params file> --json`, single thread, and compared on `counts` (archived keys only; keys the
current binary adds are listed) and `recovered` (all 32 scalars) and `all_verified`.
