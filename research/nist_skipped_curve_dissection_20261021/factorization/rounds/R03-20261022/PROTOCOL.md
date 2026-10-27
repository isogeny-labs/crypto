# R03: one bounded resume of the P-256 polynomial search

Frozen before execution, 2026-10-22. This is a separate follow-up to R02,
whose published-P-224 resume protocol explicitly excluded an R02 P-256
resume. R02 has now certified eight of nine targets. Do not reset or
reinterpret any R02 allocation or edit its records.

Question: can one further native NFS allocation advance or finish the
remaining P-256 block-876 cofactor? The exact input is

```
33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
```

Restore the checksum-verified R02 compact checkpoint into a new R03 raw
directory. Preserve the inherited first-attempt records and a pre-resume
snapshot. This checkpoint contains polynomial-search state, not a selected
polynomial or sieve relations. Its prior exit 137 is a terminated search,
not a valid negative arithmetic result.

Use R02's pinned YAFU source c7d90283972820f99b3b4920bfc1eff05c1f6342,
unchanged native executable and sievers, seed 20261022, `-gnfs -xover 81`,
two sieve/linear-algebra threads, per-process virtual memory limit 4 GiB,
native limit 1800 seconds and outer limit 1860 seconds, SIGINT followed by
30 seconds kill grace. At most one factor-search worker runs. No paid
resource or remote machine is launched. Stop on certified completion or
the bound. Do not start another production allocation in this round.

The genuine-GNFS known-answer control and checkpoint-resume control from
R02 must have passed before execution. Treat every YAFU factor label as an
untrusted hint. Exact divisibility, native PARI prime certificates and a
separate product/certificate replay against the original full target are
required. Partial partitions remain explicit; failure, signal or timeout
is not a factor-size lower bound. Preserve errors, command hashes, inputs,
outputs and native state. Report this attempt separately from R02 and do
not infer speed, security or historical seed-selection conclusions.
