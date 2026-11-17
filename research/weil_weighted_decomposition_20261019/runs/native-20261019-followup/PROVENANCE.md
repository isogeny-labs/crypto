# Full-recovery follow-up provenance

Protocol and runtime source: 040b8e1a (FOLLOWUP.md committed before execution).
The retained binary was copied from the release example before subsequent
builds. It was compiled from the same runtime source before that commit;
the additional budget unit test was added after its build and is excluded
from release code. Its exact SHA-256 is in binary.sha256. The source snapshot
is ../../sources/followup.rs, with Cargo.lock and the protocol in this branch.

Exact argv, seeds and budgets are in config.json. The process completed
with exit status 0; stdout.log and records.jsonl preserve every case.
The independent PARI checker completed with exit status 0 on all four
fixtures; see pari.log and pari_checks.jsonl. Dictionary checks and group
verification are included separately in every case's raw ledgers.

Host: the same AMD EPYC KVM container as the primary run; four-CPU quota,
16 GiB, isolation L0 with concurrent compilation/testing. No wall time is
admitted. This is an exploratory extension after seeing the primary results,
not an independent holdout or another host. The curves and stage target
panels overlap the primary run; do not count them as new distinct fixtures.
