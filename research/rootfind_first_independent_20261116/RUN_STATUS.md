# First-independent collector: source and admission status

The [frozen protocol](PROTOCOL.md), [test source](../../tests/rootfind_first_independent.rs)
and [pinned harness](harness/Cargo.toml) have been committed in that order.
The locked release harness **compiled without running a test**. The
[machine-readable build record](BUILD_STATUS.json) binds the source, lockfile,
raw compiler output and produced executable by SHA-256. Its `--no-run`
command exited zero; the nonfatal `rust-objcopy` warning and library
warnings remain in `build-locked.stderr`. [pcc:unverified]

The fixed K13/K19/K23 relation trial has **zero executions**. The
repository's experiment checkpoint must contain a complete branch-specific
resume description and be pushed to S3 before the trial. Conductor T-6342
currently reserves `dir:.resume`; the existing `.resume/RESUME.md` names
other studies and contains merge markers at lines 66, 198 and 202.
`scripts/checkpoint.sh status` reports filled sections but a stale last push;
its section check cannot establish this branch's handoff content. No gate
was pushed using that file and no experiment was launched. [pcc:unverified]

After T-6342 releases the reservation: write a clean resume file naming this
protocol, source commit, known result scope, executable and exact run command;
push and pass `scripts/checkpoint.sh gate`; then run the locked harness with
`--nocapture` once, retaining stdout, stderr and resource/time data. Recompute
each curve's full-rank comparison from raw JSON, independently check scalar
641 and the cost arithmetic, and render the required graph and PDF. The
production source remains reserved by T-6708; the frozen L2 F4/F5/F6 panel
requires its separate correct-account budget and isolation gate.
[pcc:unverified]
