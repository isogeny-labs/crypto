# Round C deviation: reciprocal setup division

Status: fail-closed correction frozen on 2026-11-18 before the corrected
source-bound controls were run. This note does not rewrite the original Round
C protocol or promote a performance claim.

Integration review found that `FastF3::from_instance` computes
`floor(2^64/q)` with one `u128` division before the existing `FieldCounts` and
`NativeCounts` snapshots. The first producer report therefore established
identity only for the counters that existed in the baseline; its wording about
total executed-work identity was too broad and is withdrawn
`[pcc:unverified]`.

The corrected candidate adds `StrongOut.reducer_setup_divisions` and emits it
in every strong-rho JSON record. It is a separate cold-setup diagnostic:

- the reciprocal candidate must report exactly 1 per strong-rho invocation;
- the pre-reciprocal baseline is normalized as 0 because its record predates
  the field and performs no reciprocal construction;
- the diagnostic is not an `F_q` multiplication, is not added to
  `FieldCounts`, and is not included in `band_ops`;
- no count or wall-time gain is inferred from replacing the baseline's
  run-time remainders with this setup operation `[pcc:unverified]`.

The corrected normalized baseline and candidate records are expected to differ
at `reducer_setup_divisions` only (0 versus 1 in each same-width pair). A
second counter-identity view removes that one declared diagnostic and must
then be byte-identical. Stop if the candidate reports any value other than 1,
or if any walk outcome, existing field/native count, or other normalized field
differs.

This correction leaves the performance obligation open. A future admitted
profile and end-to-end comparison must retain the setup diagnostic and include
the reciprocal construction in cold work. Checkpoint refusal still stops that
work before profiling or timing.
