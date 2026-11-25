# Lead to ECbench end-to-end smoke

This cohort puts two existing cryptanalysis leads behind one native ECbench
specification. Each arm receives the same public point on the same binary curve;
ECbench runs each arm cold, records every execution including failures, checks
the returned logarithm against the point, and can replay the sealed session.
The cohort checks integration and accounting. It is not an isolated performance
measurement or a ranking of the methods.

## Frozen inputs and lead mapping

- `smoke-n11.json` SHA-256:
  `2fd78b9e20fdfa7e15ef4c46274df7cc4272803f0a91848336a3ed9d6fa608c8`.
- `leads.json` SHA-256:
  `e49095fa75faffe59c359a79d8d4ba665626a0afbf3c46418eab1bf8fff818b9`.
- Workload: explicit Koblitz `n=11`, subgroup order `991`, one public target
  from seed `20261125`, one round, no warmup, 120 seconds per execution.
- Reference and A/A control: `rho.signed_frobenius_strong` on that same target.
- Root-finding lead: `ic.pipeline` with a dimension-4 standard subspace and
  complete `m=2` root-finding oracle. This is the existing
  `LEAD-ICE-rootfind-pairtable-oracle` mechanism.
- Large-prime lead: `ic.large_prime` with two large primes and `n-1` summands.
  This is the existing `LEAD-ICB-large-prime-variants` mechanism.

The exact lead-to-arm mapping is in `leads.json`. The historical lead records
are in `crypto-autoresearcher:knowledge/leads/ecdlp/`; they describe different
scales and previous measurements. This run does not replace their protocols or
promote their observations. `docs/ecbench/README.md` specifies workload IDs,
cold setup, phase accounting, `verify --replay-all`, and failed-run retention.

## Acceptance and execution

Before running, `ecbench plan --spec smoke-n11.json` must accept all four arms
and print one workload. Run into a new directory; never overwrite a prior
session. The integration smoke passes only when all four records are present,
both candidate arms and both rho arms verify the same point, and
`ecbench verify --replay-all` accepts the sealed session. Preserve failed,
timed-out, and interrupted records with the same denominator. If any arm fails,
the smoke fails and its records stay in the session.

```sh
ecbench plan --spec research/lead_e2e_bench_20261125/smoke-n11.json
ecbench run --spec research/lead_e2e_bench_20261125/smoke-n11.json \
  --out research/lead_e2e_bench_20261125/runs/<new-session>
ecbench verify --dir research/lead_e2e_bench_20261125/runs/<new-session> --replay-all
```

The smoke permits L0 for correctness only. No speedup, extrapolation, or
scientific promotion follows from this run. A later comparison must freeze a
larger same-target panel, use an isolated host, price native work across the
section 5a band, retain A/A and positive controls, and independently replay
before it enters a leaderboard.

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
