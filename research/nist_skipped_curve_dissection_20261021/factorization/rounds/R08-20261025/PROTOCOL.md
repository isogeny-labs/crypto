# R08: one two-hour local lattice-sieving continuation

Frozen before production. R07 completed filtering of R06's dataset and
requested more relations: 4,518,304 unique relations including free
relations reduced to 92 singleton survivors, with no matrix. Continue the
original R06 checkpoint, never its filter-modified copy.

Fixed residual:

```
33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
```

Require R06's completed certificate/product and common-root checks, archive
SHA-256 7c3ae630b80ede91ff3fa3a4d85e68ee1fc598df4c3010550d90979059da8181,
and the passing binary-bound native timeout control. Copy only the current
native state and input/certificate data into a new work directory; retain
the predecessor archive and its history unchanged. Perform a fresh baseline
certification and separate whole-target replay. Verify the original
relation file hash 1c4926cfcb63c8821ef209ebad4d3b0562980f73807574a8e32b3201430cced9,
range ledger, selected job and parameters. Preserve the quarantined worker
outside active state. Snapshot and blob-check all 14 runtime source files
against the recorded source commit before execution.

One local allocation on the existing host: patched YAFU hash
bc7680adbdd7450a68120e73ef8aa88d0336c8ffa141343f50966c595ca6bae9,
unchanged pinned lattice sievers, seed 20261022, two workers, -gnfs -xover 81
-pbatch 120, native limit 7200 seconds, outer 7260 seconds plus 30-second
kill grace, 4 GiB virtual memory per native process and 1 GiB per output
file; core dumps disabled. Require at least 1 GiB free workspace after
preparing the new state. Preserve the ordinary native collection target;
do not carry R07's filter-dispatch override into production.

Stop on certified factorization, an execution failure, output cap or time
bound. Retain all surviving raw outputs, source/executable hashes, exact
commands, pre-resume snapshot and exit statuses. A proper factor hint needs
exact division, recursive prime certificates and separate whole-target
replay. Raw relations do not establish matrix rank or completion time.

An interim report must timestamp actual process state. Final status,
archive, replay and completion report stay pending until termination.
This record creates no paid/remote machine, task or lease and defines no
later allocation. The separate distributed backlog card remains unqueued.
