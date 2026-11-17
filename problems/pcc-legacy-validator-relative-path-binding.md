# Legacy validator path binding blocks the shared PCC gate

- **Status:** open
- **Kind:** inherited verification-tool/evidence-format incompatibility
- **Affected work:** [PR3452](https://github.com/isogeny-labs/crypto/pull/3452)
- **Last updated:**2026-11-05

## Problem and observed evidence

After404 exact tracked archive inputs were materialized, local PCC verify-all
still exits1 for13 existing main claims. The recompute checker treats the legacy
validator raw_data path `../../../research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json`
as escaping the repository. The shared verifier, keyring,13claim records and
validator report are unchanged from main; none was written by this continuation.
[Policy receipt](../research/prime_ecdlp_three_leads_20261029/continuation-20261030/validation/main-d30b9e0db-20261105/policy-captures/receipt.json)
retains every failure. These are unsigned local observations. [pcc:unverified]

The documented recompute contract makes paths relative to evidence_dir, defaulting
to the report directory. Whether the old record has the required complete digest
bindings remains a separate question; some reported entries also lack sha256.
A parser compatibility repair alone would not authenticate or fully cover them.
No signed verdict, external execution proof or scientific admission follows.

## Resolution criteria and next discriminating step

Review the shared verifier's evidence_dir/relative-path interpretation against
the existing contract and frozen report. Fix the canonical shared implementation
through its proper upstream path, with canaries that reject true repository
escapes and distinguish missing digest bindings from verified recomputation.
Preserve all frozen reports, claim records, signatures and append-only ledgers.
Do not edit their pinned bytes, relax verification or introduce a local trust
exception to pass this PR. Repeat the unchanged failing cases and the full gate;
exact-head required CI must pass before merge. This is a policy-tool dependency,
not a mathematical obstruction or a failure of the new PKM contracts.

Unresolved. The new five PKM theorem-binding checks locally exit0 but standing
remains unverified; none of the13legacy failures is a new PKM claim. Benchrec
now reports37records/current progress after exact checkout restoration. [pcc:unverified]
