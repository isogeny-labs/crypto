# Published PCC Lean record has incomplete declaration bindings

- **Status:** open
- **Kind:** producer metadata defect; supported correction-path gap
- **Affected work:** PR5071, [erratum](../research/curve25519_torus_native_20261115/coset-spectrum-v1/record-correction-v2/ERRATA.md)
- **Next action owner:** producer for complete companion evidence; canonical PCC maintainers for a supported immutable-record correction
- **Last updated:** 2026-11-23

## Problem

Published record8d8628 omits decl_sha256 in three lean proof objects. The
current changed-claim CI correctly refuses it. A correctly declaration-bound
companion is complete, but the maintained tool has no supported way to
withdraw/supersede the original active malformed record while keeping its
published bytes/history immutable. [pcc:unverified]

## Evidence and uncertainty

- **Observed:** exact1ef-head job137952785754 reports the missing required
  fields on8d8628. Full decoded59687-byte log and source identity are retained.
- **Prepared correction:** cd1904 pins normalized statements/hashes and the
  unchanged source. Maintained local verification reports three lean passes
  and overall pass. Its verdict is unsigned; standing remains unverified.
- **Retained:** old8d8628 JSON and all published ledger prefixes. No change to
  source proof, frozen report/claims, verifier, keyring, policy or checks.
- **Missing:** an authoritative data-preserving active-record correction or
  withdrawal procedure accepted by the maintained verifier/workflow.
- **Not inferred:** a failed Lean theorem, a failed spectral equivalence,
  signed validation, permission to skip the failed check or broader ECDLP
  impossibility. This is the producer's introduced metadata failure.
[pcc:unverified]

## What would unblock it

- [x] Produce the complete new binding record and validate it with the
  maintained verifier against the unchanged theorem source.
- [ ] Apply an authoritative supported correction/withdrawal procedure that
  preserves the original published record/history and does not alter trust
  or bypass a required check.
- [ ] Read actual new-head CI/reviews and verify that the gate accepts the
  correction. A companion local pass alone does not make the old check pass.

## Next discriminating step

Canonical maintainers must identify the supported record-management action;
the CLI inspected here has no such command. The producer has provided the
exact old record, new record, declaration hashes, source/gate receipts and
reproduced failure. Do not repeat unchanged downloads, rewrite the old record,
add a schema exemption or patch around the verifier. Recheck on a supported
canonical remedy or a genuinely changed head. No experiment/paid job is
needed to establish this metadata dependency.

## Resolution and history

Unresolved. Correct companion cd1904 advances the record correction but does
not withdraw8d8628. Publishing this problem or passing the source kernel gate
does not resolve the active failed record. Broader coefficient-root discovery
and growing-family cost work remain separate, active scientific obligations.
[pcc:unverified]
