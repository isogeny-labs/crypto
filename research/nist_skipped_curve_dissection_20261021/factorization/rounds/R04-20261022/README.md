# R04: continue from the recognized P-256 checkpoint

The allocation ended with **exit 137 at its outer bound, without a new
factor**. Native exact validation preserves **329 candidate records**,
including repeats, through leading coefficient **900**. A separate bounded
native recognition diagnostic then selected the best saved polynomial by
the tool's recorded Murphy-E score and generated the job for sieving. Its
input and exact common root pass native checks in both output formats.
**The next job can start sieving directly.** No sieve relations or new
factor were produced; the full factorization remains incomplete.

The remaining target is `skipped_256_876-twist2`: a 512-bit degree-2
extension-twist order whose residual is 444 bits. The
[exact partial partition](result.json) and [round summary](summary.json)
keep mathematical status separate from native execution status. The eight
complete factorizations and thirteen resolved timeout fields from
[R02](../R02-20261022/README.md) remain the current arithmetic result.

R04 uses the corrected master checkpoint from R03, with seed 20261022,
two native threads, polynomial batch 120 and a single 1800-second native
allocation. Its first new polynomial has coefficient 660; later records
pass 800. Native merging writes a durable master before the process ends.
The count in [polynomial-verification.json](polynomial-verification.json)
counts valid saved records, including repeats. It does not measure unique
search coverage, completed coefficient ranges or polynomial optimality.
The [selected job](selected-polynomial.job),
[polynomial parameters](selected-polynomial.fb),
[exact root verification](selected-polynomial-verification.json), and
[provenance](selected-job-provenance.json) distinguish the diagnostic-derived
job from the stopped production search. The native configured relation
target is 46,000,000; it is neither an observed relation count nor a runtime
estimate or guarantee of sufficient usable relations.

Read the [run report](reports/run-20261022T0935Z/REPORT.md),
[inspected PDF](reports/run-20261022T0935Z/report.pdf), and
[frozen protocol](PROTOCOL.md). The original literal cofactor and target
identity have not changed. No historical-selection, security, speed or
smallest-prime-factor bound follows from this bounded search.

## Continue and verify

The [portable checkpoint](checkpoint-skipped_256_876-twist2.tar.xz)
contains the native master, executable, attempt history, certified parent
partition and verified result. Check its sibling SHA-256 first. With a
compatible AVX2/BMI2 host, matching libecm and the pinned native sievers:

```bash
round=research/nist_skipped_curve_dissection_20261021/factorization/rounds/R04-20261022
NIST_FACTOR_CHECKPOINT="$round/checkpoint-skipped_256_876-twist2.tar.xz" \
GP_BIN=gp GNFS_BIN_DIR=/opt/yafu/factor/lasieve5_64/bin NFS_POLY_BATCH=120 \
  bash "$round/resume_with_checkpoint.sh" skipped_256_876-twist2 /tmp/nist-p256-next 1800 --resume
bash "$round/../R02-20261022/verify_nfs_artifact.sh" /tmp/nist-p256-next \
  skipped_256_876-twist2 \
  33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
```

The command launches compute; its separately reviewed later allocation is
the [draft backlog card](../../../../../backlog/XP-20261022-nist-p256-nfs-resume/README.md).
No later task or lease is queued. A successful partial replay certifies the
partition, not completion of the factorization.

The wrapper preserves unmerged candidates after the native process stops.
[prepare_polynomial_checkpoint.sh](prepare_polynomial_checkpoint.sh)
refuses an active search, preserves originals, frames complete records,
then uses native GP for every exact root check. Consumed per-thread files
stay in evidence and cannot be merged twice. Native time records are
preserved verbatim; none is invented. Failed checks prevent checkpoint
replacement. The helper's real-fixture repeat and active-worker controls
are retained in the raw evidence.

## Raw evidence

`runs.tar.xz` is the complete deterministic raw tree; its sibling SHA-256
and [archive validation](archive-validation.json) identify the bytes and
replay checks. The portable excerpt and loose master remain separate
verifier/job inputs under the AGENTS.md section 13a exception. To inspect:

```bash
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

The separate, much larger R02 archive remains local pending the
[required artifact-store connection](../../../../../problems/nist-factorization-raw-archive-publication.md).
This small round does not claim that R02 bulk publication is complete.
