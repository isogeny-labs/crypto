# Local full-D18 positive-control diagnostic

This diagnostic runs only the constructed four-summand control from
`NATURAL_PROTOCOL.md` on a shared macOS host. It imports the same complete
32,643-column S3 archive, checks the frozen public fixture, installs the same
chained-S3 encoding and orbit domains, pins the four known summands and
intermediate x blocks, and uses the same SAT-model, original-field group,
signed-orbit row, tamper and rank-one checks. It does not run a natural query.

Freeze the source in a clean commit before execution. Retain the source
commit, executable SHA-256, input GET receipts and hashes, process stdout,
stderr, UTC interval, exit status, phase/shape/result/cap records, host
architecture, `/usr/bin/time -l` resource record, and a raw-file SHA-256
inventory. Run the new `primary-full-d18-m4-l0-positive BASE_DIR FIXTURE_DIR
NEW_OUTPUT_JSON` command with a separate 310-second outer process cap. Its
internal whole-process cap is 300 seconds; the SAT conflict budget is 100,000.
Use no more than one CPU by the supervisor where possible, and record whether
that was enforced. A nonzero exit, absent receipt, OOM or cap is censored.

The worker labels every result `L0_shared_host_no_hard_cgroup`. A passing
control establishes only that this source, full base and encoding can recover
and validate the planted relation under local conditions. It cannot promote
the frozen hard-32-GiB Linux gate, estimate natural relation yield, or select
a best factor base. Keep `selected_best_total_runtime=null`.
