# PCC attachment type correction — 2026-11-05

The [PCC gate on the original head](https://github.com/isogeny-labs/crypto/actions/runs/41987192419/job/127110608534)
rejected `PCC-20261105-6d30f5` and `PCC-20261105-a3310a`: the producer had
attached `identity.certificate/v1` records as PCC `certificate` proofs, which
require `pcc.solve-certificate/v1`. This was a producer classification error,
and the gate's rejection was correct [pcc:unverified].

The proof-kind contract is [Sabotage resistance §4](../../docs/SABOTAGE_RESISTANCE.md).
The original claim files are preserved byte-for-byte under
[history/pcc-claims/](history/pcc-claims/), and remain reproducibly rejected by
the unchanged native checker. They are historical malformed records, not the
current live claim records [pcc:unverified].

| Historical malformed record | Current native record |
| --- | --- |
| [PCC-20261105-6d30f5](history/pcc-claims/PCC-20261105-6d30f5.json) | [PCC-20261105-ac1e93](../../pcc/claims/PCC-20261105-ac1e93.json) |
| [PCC-20261105-a3310a](history/pcc-claims/PCC-20261105-a3310a.json) | [PCC-20261105-9d5a6d](../../pcc/claims/PCC-20261105-9d5a6d.json) |

Each replacement was created with the native issuer. Its `statement` and
`scope` are identical to the historical record, and its artifact paths and
SHA-256 pins are unchanged. The supported type is now `attestation`, with
canonical artifact subjects and no signature. No independent verifier has
vouched for these bytes. Native verification returns `not_covered` rather
than a passing proof; statement claims can never become verified
[pcc:unverified].

[pcc-type-correction.json](pcc-type-correction.json) pins both historical and
replacement records. [pcc-type-validation.json](pcc-type-validation.json)
records the old exit1 failures and new exit2 classifications. Native PCC
verifier blob `6bb390044068e20e4d87b1563b3bd5d39fb0fe82` is the existing
public version used for the repair checks. The failed original claim files
also remain unchanged at the
[first published commit](https://github.com/isogeny-labs/crypto/commit/ceb109e9ae0a4682f479f014ab6bfb0c261f88f2)
[pcc:unverified].

The live public claim set uses the replacements. Private native claim files
and every existing native ledger row remain unchanged; the new native
records are append-only additions and their ledger publication remains
withheld. No verifier, workflow, gate condition, assertion, polynomial
identity or original replay receipt was edited [pcc:unverified].

Reproduce the distinction using the existing tool; the first command is
expected to return1 and the second2:

```sh
python3 tools/pcc/pcc.py verify research/aps_leads_20261105/history/pcc-claims/PCC-20261105-6d30f5.json
python3 tools/pcc/pcc.py verify PCC-20261105-ac1e93
```

The four APS leads, their pending implementation/performance status and all
earlier source/test boundaries are unchanged. No benchmark ran.


## Public base integration

The observed main tip `d30b9e0db7ec00c55330effef5f64532f97cd160` advances the
original public base `369c1a2ad56cfd597313a5ef6a101616f234e15c` by23 commits
and45 changed paths. None overlaps the APS study or its claim files; the
public ledger and workflows are unchanged between those base tips. A normal
two-parent integration preserves both the original PR head and that observed
main tip. No conflict resolution, rebase or force push is involved. Private
native ledger history is excluded from the public delta and remains
unreconciled [pcc:unverified].
