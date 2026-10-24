# Continue the remaining P-256 NFS factorization

Status: **draft, not queued**. PR #2379 supplies the executable restore,
bounded native resume and independent certificate verifier. R02 completed
eight of nine targets; its [P-256 checkpoint](../../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R02-20261022/README.md)
is the remaining 444-bit composite lineage. R03 corrected a native restore
failure; its validated compact master checkpoint is the fallback input.
R04 preserves a selected sieving job. The card now selects the newer
[R05 relation checkpoint](../../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R05-20261022/README.md),
which requires actual artifact-store publication before ready promotion.
R03/R04 fallback archives omit R05's relations and are not equivalent resumes.
The [native timeout correction](../../research/nist_skipped_curve_dissection_20261021/factorization/native-timeout-diagnostic-20261023/README.md)
and a binary-bound passing native control are required by the checked wrapper.
This card is a later allocation, never a claim that
another worker or cloud task is running.

Question: can one further allocation produce a proper split, or advance
the retained polynomial/sieving state? A certified split changes the known
factor partition; no split leaves the complete factorization unresolved.
Progress in native state is recorded separately from the arithmetic result.

Before ready promotion, pin the merged implementation, publish and bind
the R05 checkpoint and checksum, select a compatible AVX2/BMI2 host and lab,
and verify the fixed input. Build the recorded two-line native patch and
run the bounded original-versus-patched controls on that host. Set
`YAFU_PATCHED_BIN` and `YAFU_TIMEOUT_CONTROL_DIR` to the actual validated
binary and its control record. The checked wrapper retains the old
checkpoint executable and installs only the exact tested binary under both
existing locks; its prepare-only replay passes. `NIST_FACTOR_CHECKPOINT` selects the explicit
archive and its sibling `.sha256`; continuation of an existing output
directory uses that directory's state. Exact command and verifier are in
[card.json](card.json), with bounds in [PROTOCOL.md](PROTOCOL.md).

The resource reservation is two CPUs, 16 GiB RAM and 32 GiB disk, no GPU.
The native computation is bounded by 1800 seconds, the process by 1860
seconds plus 30 seconds kill grace, and the task by 2400 seconds. These are
configured envelopes. There is no verified end-to-end completion estimate
for this 134-digit input, so the runtime/core-hour estimates remain null.
The prior genuine-GNFS control is a correctness check, not a performance
model for this larger input. Funding is zero while unqueued; no existing
funded card is preempted and no lease, task or paid machine is created.

Done means the bounded attempt's output, status and preserved native state
are accounted for, independently checked against the literal input, and
landed with a report. Mathematical completion additionally requires every
factor to be prime-certified and the whole original target product checked.
A verified partial partition does not meet that mathematical exit condition.
The [open obstruction](../../problems/nist-extension-twist-composite-cofactors.md)
retains the unmet requirement. Historical skip reasons and curve security
are outside this factorization experiment's conclusions.
