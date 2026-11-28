# Claim records and standing

These are unsigned producer data attestations. Each record binds the exact raw
files it names; none is a signed execution proof or independent verification.
The repository verifier reports `not_covered`: the intentionally descriptive
`statement` claim kind has no proof kind and cannot be verified. A future proof
claim requires a separate eligible evidence record.
The append-only ledger check succeeds; it does not upgrade their standing.

| Record | Scope |
| --- | --- |
| [PCC-20261123-7d7792](../../pcc/claims/PCC-20261123-7d7792.json) | Historical 14-test and 16-test observations. |
| [PCC-20261123-3048b0](../../pcc/claims/PCC-20261123-3048b0.json) | Final 17 plus two native tests, and eight public-example tests. |
| [PCC-20261123-08018e](../../pcc/claims/PCC-20261123-08018e.json) | Retained expanded B8 failures and completed compact B8 controls under unchanged limits. |
| [PCC-20261123-c0200b](../../pcc/claims/PCC-20261123-c0200b.json) | Evaluated encoded norm identity and rejected false-expression control. |

Raw verifier outcomes and the ledger-prefix check are in
[evidence/pcc-checks](evidence/pcc-checks/). The reviewed source closure is
identified by its actual-source inventory and build manifests, rather than
assuming that an earlier Git HEAD already contained the then-uncommitted files.

After tracking PR #4949 merged, the final publication preserves main's complete
ledger prefix and appends new registration envelopes for these unchanged claim
files. The [registration mapping](evidence/pcc-final-main-registration/registration-map.json)
pins the original envelopes in their immutable archive. The
[final prefix check](evidence/pcc-final-main-registration/ledger-verify.stdout.txt)
reports 1,557 entries and a valid chain; the older verification receipt remains
intact. Re-registration does not change evidence standing.
