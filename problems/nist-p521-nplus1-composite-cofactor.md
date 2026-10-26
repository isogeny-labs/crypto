# P-521 n+1 retains a composite cofactor

- **Status:** `investigating`
- **Kind:** incomplete integer factorization; no impossibility claim
- **Affected work:** [four-target follow-up](../research/nist_skip_criterion_20261025/factor_followup_20261026/REPORT.md), [PR 2378](https://github.com/isogeny-labs/crypto/pull/2378)
- **Next action owner:** active study author
- **Last updated:** 2026-10-26

## Problem

The requested published P-521 `n+1` factorization is not yet complete.
The full original target has an exact verified partial product with proved
prime factors and this remaining **composite**, written in decimal:

`36267274398750192082087714560485501370968815968939229615728908768222975800577003985527583945002884540942812611007064014452869`.

It has 125 decimal digits and 414 bits. Those sizes describe a composite,
not a largest prime factor. The [partial record](../research/nist_skip_criterion_20261025/factor_followup_20261026/results/p521-plus-partial.json)
keeps every prime/composite flag. The native comparison retains all 21
curves and uses `NA` for the unknown value. The complete `n+1` feature family
stays unavailable across the cohort rather than dropping this control.

## Evidence and uncertainty

- **Observed:** the other three requested targets have independent native
  prime/product proofs. There are 83 complete target factorizations out of 84.
- **Observed:** native ECM and P-1/P+1 attempts did not finish this residual.
  Failed input admissions, bounded attempts and actual statuses remain in
  the [run index](../research/nist_skip_criterion_20261025/factor_followup_20261026/results/run-index.json).
- **Observed:** GNFS saved 2,313,299 merged relations before a requested stop.
  The [closed progress archive](../research/nist_skip_criterion_20261025/factor_followup_20261026/evidence/backend-progress-001-delivery.json)
  passed a full S3 download and all 108 member hashes. Run014 reuses that
  pool with four threads and adaptive filtering. A running job or relation
  pool is not a complete factorization.
- **Observed:** [snapshot002](../research/nist_skip_criterion_20261025/factor_followup_20261026/evidence/backend-progress-002-delivery.json)
  preserves the running pool whose native filter reported17,831,805 raw
  and14,715,396 unique relations. A full S3 GET, internal manifest and all
  21 listed file hashes verified. This remains an active-run snapshot,
  without a final factorization result or exit status.
- **Derived or proved:** the partial product and known primes are checked;
  the remaining integer is composite. No complete prime factorization of
  this residual is admitted.
- **Missing result:** a proper split followed by prime proofs for every
  resulting cofactor and equality to the original target.
- **What this does not establish:** bounded failure does not prove
  infeasibility, curve weakness, security or a historical selection cause.

## What would unblock it

- [ ] Split the exact composite and prove every terminal prime using the
  independent native verifier, with its full original row20 `n+1` product.
- [ ] Admit the resulting complete record, regenerate all21 complete-factor
  families and replay the unchanged search/calibration without omitted controls.

Those steps complete the requested factorization/comparison computation.
The [historical selection problem](nist-candidate-selection-weakness-hypotheses.md)
and any applicable DLP mechanism remain separate obligations.

## Next discriminating step

Collect the actual native GNFS continuation, retain all failures and progress,
and verify any proposed divisors independently. The declared protocol fixes
local four-core/16-GiB resources and per-attempt watchdogs, not a proof of
infeasibility or a promised finish time. Preserve a closed, hash-verified
checkpoint before any further continuation. No DLP run is requested.
