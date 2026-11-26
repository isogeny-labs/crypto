# Shared lead ECbench end-to-end panel

The current `paired-k1-17.json` cohort puts two existing cryptanalysis leads
behind one native ECbench v2 specification with the canonical
`semaev-groebner-f2m17-v1` field profile. Each arm receives the same public
point per workload. ECbench runs the arms cold, records every execution
including failures, checks each recovered scalar, and replays the sealed
session. This L0 cohort checks end-to-end correctness and phase accounting;
its unpriced native work and host isolation do not support a performance rank.

## Current paired panel

- Spec: `paired-k1-17.json`, SHA-256
  `4c8b8510fab79cea67a2a92f1dd8e4bd4462824be41046c6636efc26c2058caa`,
  frozen before execution in commit `cd235d6a6b5`.
- Two public targets on canonical K1-17, two cold measured rounds each, no
  warmup, alternate order, 120 seconds per arm; 16 planned executions.
- Reference and A/A control: `rho.signed_frobenius_strong`.
- Root-finding lead: `ic.pipeline`, standard subspace dimension 6, complete
  `m=2` root-finding with negation folding.
- Large-prime lead: `ic.large_prime_v2`, small dimension 4, envelope dimension
  6, two summands and up to two large primes. The v2 path charges subgroup
  membership and final recovery checks; its final check is inside the online
  interval. It also performs fast and independent scalar verification.

Plan and run from the repository root with a newly built `ecbench` binary:

```sh
ecbench plan --spec research/lead_e2e_bench_20261125/paired-k1-17.json
ecbench run --spec research/lead_e2e_bench_20261125/paired-k1-17.json \
  --out research/lead_e2e_bench_20261125/runs/<new-session>
ecbench verify --dir research/lead_e2e_bench_20261125/runs/<new-session> \
  --replay-all
```

Acceptance requires both workload IDs, 16 retained records including any
failed or timed-out arms, matching recovered scalars within each workload,
outer verification, v2 internal verification and a nonzero
`target_recovery_check` on recovered v2 records, and 16/16 deterministic
replays with zero audit problems. A failed arm remains in the denominator and
fails this correctness panel. The first completed panel and receipt are in
`RESULT.md`, `runs/panel-k1-17-20261125T2303Z/`, and
`audit-paired-k1-17.json`.

## Historical first smoke

The initial `smoke-n11.json` used the historical v1 specification and the
original `ic.large_prime` accounting path. It exposed the missing online
recovery check that the current v2 panel addresses. Its raw records remain
available for provenance, but current ECbench admission requires v2 and an
explicit curve policy for new sessions.

The historical spec SHA-256 is
`2fd78b9e20fdfa7e15ef4c46274df7cc4272803f0a91848336a3ed9d6fa608c8`.
It used one public n=11 target, one round, a dimension-4 root-finding base,
and the original `ic.large_prime` with `n-1` summands. Its records and replay
receipt remain in `runs/smoke-20261125T2230Z/` and `audit-smoke-n11.json`.

The current lead-to-arm mapping is in `leads.json`. The historical lead
records are in `crypto-autoresearcher:knowledge/leads/ecdlp/`; they describe
different scales and previous measurements. This panel does not replace their
protocols or promote their observations. `docs/ecbench/README.md` specifies
workload IDs, cold setup, phase accounting, replay, and failed-run retention.

A later performance comparison needs an isolated host, priced native work
across the section 5a band, more same-target workloads, and independent
replay before leaderboard admission.

## Sources checked

- `docs/ecbench/README.md`, sections 1, 3, 9 and 11: use its existing native
  method registry and sealed runner rather than adding a second measurement
  implementation.
- `docs/ecbench/specs/smoke-binary-large-prime.json` and
  `docs/ecbench/specs/smoke-koblitz-rootfind-vsrho-cycle4.json`: existing
  parameter spellings and compatible Koblitz methods.
- `crypto-autoresearcher:knowledge/leads/ecdlp/ic-engine/LEAD-ICE-rootfind-pairtable-oracle.yaml`
  and `.../ic-binary/LEAD-ICB-large-prime-variants.yaml`: exact lead scope,
  previous evidence, and open qualification steps.
- Shared KB revision `750797a540fb4c38837c67b12df84d5b` (refreshed
  2026-11-25 03:19 UTC), queries `shared end-to-end lead benchmark ECbench
  adapter same-target rho` and `unify candidate leads benchmark workload
  identity independent replay`; `source:b3c90d39a347ef661ccf10680a742186`
  points to an existing ECbench adapter plan, and
  `source:4850fb02f0ff80e7b780b595d969daa2` identifies the frozen
  manifest and result checks used by the isolated benchmark service.

The KB search is a pointer to those primary files, not a verification of
this new cohort. The checked literature and prior-result boundary comes from
the two lead atlas records; this change adds no new mathematical method.
