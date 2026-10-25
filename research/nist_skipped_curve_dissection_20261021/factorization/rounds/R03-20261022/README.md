# R03: correct the interrupted polynomial checkpoint

R03's P-256 attempt was stopped after discovering that restoring a
per-thread `.p` file did not restore native polynomial-search progress.
It repeated the earlier first candidate and began at the original range.
SIGINT did not stop it; SIGTERM ended it with exit 143. This is an
**inconclusive infrastructure failure**, not a new negative factorization.
The full target remains a verified partial partition with a 444-bit residual.

The corrected [master file](master-polynomials.p) contains **242 complete
saved polynomial candidates**, each checked natively for an invertible
`Y1` and exact common root modulo the literal input. No tested-range or
elapsed-time record was invented. A wrong-input control rejects; YAFU's
separate three-second diagnostic confirms the input match and resumes
at leading coefficient **660**. This verifies saved candidates and native
recognition, not optimal polynomial quality or completed range coverage.

See [report](reports/run-20261022T0913Z/REPORT.md),
[inspected PDF](reports/run-20261022T0913Z/report.pdf),
[native polynomial verdict](polynomial-verification.json),
[partial partition](result.json), [summary](summary.json), and the frozen
[recovery protocol](PROTOCOL-CHECKPOINT-RECOVERY.md).

## Recheck the master file

Use a fresh output path. GP errors can exit zero, so require the JSON
verdict as well as retaining stderr:

```bash
round=research/nist_skipped_curve_dissection_20261021/factorization/rounds/R03-20261022
NFS_INPUT=33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353 \
POLY_INPUT="$round/master-polynomials.p" POLY_OUTPUT=/tmp/nist-master-replayed.p \
  gp -q -f "$round/recover_polynomials.gp" > /tmp/nist-master-verdict.json
jq -e '.verified and .candidate_count==242' /tmp/nist-master-verdict.json
cmp "$round/master-polynomials.p" /tmp/nist-master-replayed.p
```

## Portable continuation

The checksum-bound [checkpoint excerpt](checkpoint-skipped_256_876-twist2.tar.xz)
contains the recognized master file, unchanged native executable, parent
partition, certificates and attempt history. The small excerpt is a
derived runnable input; the complete raw tree is separately archived as
`runs.tar.xz`, with checksum. Verify the archive before extraction.

```bash
round=research/nist_skipped_curve_dissection_20261021/factorization/rounds/R03-20261022
NIST_FACTOR_CHECKPOINT="$round/checkpoint-skipped_256_876-twist2.tar.xz" \
GP_BIN=gp GNFS_BIN_DIR=/opt/yafu/factor/lasieve5_64/bin NFS_POLY_BATCH=120 \
  bash "$round/../R02-20261022/resume_job.sh" skipped_256_876-twist2 /tmp/nist-p256-next 1800 --resume
```

This command starts compute; review the later [backlog card](../../../../../backlog/XP-20261022-nist-p256-nfs-resume/README.md)
and preserve its bounded allocation. [R04](../R04-20261022/PROTOCOL.md)
separately continues from the corrected state. The wrapper now refuses a
per-thread-only checkpoint instead of silently starting again. Pinned
YAFU source and all arithmetic acceptance rules remain unchanged.

The raw archive preserves the duplicate attempt, signal interventions,
initial parser failure, corrected native checks, wrong-input rejection,
diagnostic and portable replay. No new historical or security conclusion
follows; no speed measurement was attempted. R02's eight certified
factorizations and thirteen resolved timeout fields remain valid.
