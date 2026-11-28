# Primary N83 canonical six-summand high-capacity pair

[GitHub Actions run 39100306846](https://github.com/aburan28/crypto/actions/runs/39100306846)
completed successfully on two distinct Linux ARM runners from source
`206adfd27888b4a8bc7d39a3f30695f5f21cfcac`. The frozen
[dispatch](dispatch.json) reused the exact public `a=0` N83 K600 base,
Gray-prefix residual pool and hash-derived residual pool from the earlier
canonical pair. The source provider ZIP was checked byte for byte; no new
factor-base construction choice was made in this rung. Each arm used one CPU,
4 GiB, the same public target and canonical 4+2 solver shape, a 7,200-second
worker cap and **100,000 conflicts**. The [provider audit](results/39100306846/audit.json)
bound both terminal jobs, distinct runners, both GitHub artifact digests,
the exact provider ZIP and extracted inputs, native commands, resource
inspections, cleanup and raw worker results. It returned
`PASS_BOUND_TWO_HOSTED_SOLVER_RECEIPTS`.

| Arm | Residual signed orbits | Search wall (L0) | Conflicts | Models | Classification |
| --- | ---: | ---: | ---: | ---: | --- |
| Gray-prefix pool | 97 | 4,421.716 s | 100,000 | 0 | `UNKNOWN_search_cap` |
| Hash-derived pool | 600 | 6,460.065 s | 100,000 | 0 | `UNKNOWN_search_cap` |

Each arm made one solver call on the first identity pattern. Neither result
refutes the full SAT instance or measures a relation-yield rate. The separate
[Gray](results/39100306846/gray_pool.model-replay.json) and
[hash](results/39100306846/hash_pool.model-replay.json) model-replay receipts classify both as
`NO_MODEL_TO_REPLAY`. Natural partial yield, useful rank, linear algebra,
individual target descent, matched rho and complete cold IC runtime remain
unmeasured. The conservative two-job charge is **21,600 seconds**, independent
of the shorter observed L0 search walls. The primary
`selected_best_total_runtime` remains `null`.

The raw Gray and hash ZIPs were stored at content-addressed, conditional
write-once keys under the requested S3 prefix. Their exact `s3://` URIs are
in the [S3 audit receipt](results/39100306846/audit-s3.json). GitHub reports
the same SHA-256 digests. Both local ZIPs
passed safe-member and CRC checks. Fresh complete S3 GETs matched the original
ZIPs byte for byte, and the independent [S3 auditor](audit_s3.py) checked
lengths, SHA-256 provider bindings, single-part ETags, metadata, encryption
and the exact two-object [inventory](results/39100306846/provider-inventory.json).
Its [receipt](results/39100306846/audit-s3.json) passed; a changed fresh-GET
length was [rejected](results/39100306846/s3-mutation-control.json).

The move from the prior 20,000-conflict pair to 100,000 conflicts produced no
model in either arm. This is evidence about **this exposed-target,
one-pattern solver configuration**, not a performance ordering of factor
bases. A multi-target natural relation and rank panel is still required
before selecting any primary N83 base by total runtime.

The N83 Python discovery suite passed [63 tests](results/39100306846/validation-python.log)
after the new S3 auditor was added. The branch's prior full release library
run passed 4,085 tests with 117 ignored; this result commit changes no Rust
source.
