# Frozen local full-D18 m4 model-construction gate

Commit this protocol and the opt-in `primary-full-d18-m4-model` worker before
the retained run. Use the exact complete, previously fresh-fetched S3 object
and manifest in this directory, plus the public primary-curve fixture zero
in `research/koblitz_n83_factor_base_sweep_20261008/pilot-01/`. The importer
must recheck every archived point and label, all signed-Frobenius orbit maps,
the committed S3 custody and the public point. The binary must attest the
clean source commit and record its own SHA-256.

Construct one four-summand chained-S3 circuit for identity mask zero and the
complete 32,643-representative signed-Frobenius x-domain. Set 400,000 maximum
variables and 12,000,000 maximum domain clauses. The exact analytic
preflight is 329,726 required variables and 11,238,656 domain clauses; reject
a change. Record the actual SAT variables, all CNF clauses, XOR rows, AND
gates, S3 nodes, import/model/process wall and process memory. Write a
durable `*.preflight.json` before model construction, then the result to a
new file. Use one process and a 600-second independent outer wall cap; retain
its exit code, stdout/stderr and `/usr/bin/time -l` output. If the cap or
memory limit stops the process, preserve the partial evidence as
`UNKNOWN_budget`. The worker performs **no SAT search**, group lift, rank
update or target-log stage. This shared macOS host is L0 and does not satisfy
the hard-Linux-cgroup gate in [`NATURAL_PROTOCOL.md`](NATURAL_PROTOCOL.md).

The construction result is a capacity measurement only. It can inform a
bounded Linux host choice but cannot rank the factor base or set
`selected_best_total_runtime`.
