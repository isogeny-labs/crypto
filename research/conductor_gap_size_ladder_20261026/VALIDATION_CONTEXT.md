# Parent artifact migration and local validation

Parent revision a7038c5b8ee882e4165276a2b41f11016474c698 moves data into
the artifact store, after PR #2560 merged at 02fcefddf. AGENTS.md and the
validation-gate implementation are unchanged. This follow-up starts from
that current parent revision and does not alter either benchmark row,
their VALIDATION.json files, frozen raw evidence, or the gate/workflow.

Before hydration, `python3 scripts/test_validation_gate.py` runs 49 tests
with one failure, `Committed.test_gate_passes_on_the_committed_tree`.
`validation_gate.py check --base a7038c5b...` reports two invalid rows:

- benchmarks/20261022-deep-volcano-prime-gap
- benchmarks/20261023-volcano-gap-speedups

Fifteen distinct raw data files referenced by those rows are absent, with
replacement .artifacts.json manifests in their parent directories. The
gate reads local files; the current workflow has no artifact hydration
step. The observed errors, rather than a speculative service failure, are
in checks/gate-tests.log and checks/gate-check.log. Their nonzero exit
statuses are retained.

For local validation, only missing files were restored from pre-migration
commit 02fcefddfd3953e4cd45909baa7ec4e70560e80b. Every restored file was
hash-checked before being placed at its ignored original path; all 53
referenced input digests were checked. Inputs and results are in
checks/validation-hydration-inputs.tsv and checks/validation-hydration.log.
No frozen record was generated, recomputed, edited or committed again.

After hydration, all 49 gate tests and the base-aware gate pass; logs are
checks/gate-tests-hydrated.log and checks/gate-check-hydrated.log. These
local passes do not establish that a clean CI checkout will hydrate its
inputs. This baseline integration issue remains a PR gate blocker unless
the exact-head CI/check evidence establishes otherwise. It is not repaired
by this parameter-search PR or by the new native certificate-check workflow.

The initial site-test invocation also used an incorrect import path;
checks/site-tests-attempt1.log retains that error. The canonical discovery
invocation, with PYTHONPATH=scripts/site, passes all 49 tests. Native
compilation's first attempt had an initializer-list type mismatch; the
retained error predates the frozen source and was corrected before the
parameter search. Neither issue is presented as a passing attempt.
