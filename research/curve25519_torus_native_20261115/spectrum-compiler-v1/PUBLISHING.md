# Publication and artifact gates

Source commits44a51fe543 and ee62baf4a9 were published through the maintained
lane helper as cc1acabe36 and7a21a917a2 before the qualified final native call.
Native evidence/claim989b50 was published as412041f073. All code/raw/archive
bytes are available in that immutable Git history; this is not trusted-store
publication or independent scientific validation.

The actual unchanged artifact policy selects these four new frozen subjects:

| Subject | Bytes | SHA256 |
| --- | ---: | --- |
| `source-v1.tar.xz` | 80408 | `073091adec34dad4801539518e5caf14248ddf3269c6e19b206b09fc43082810` |
| `native-v1/controls.jsonl` | 184205 | `81d29842010f77fc4bc7c3a31394b5e35920beee013059a20ad5f9f4021da694` |
| `native-v1/native-run-v1.tar.xz` | 89764 | `c735b9ec5347c8ed39da03588e5809690a7188c3da461f82fd0dd1b80794ad56` |
| `native-v1/original-actual-k20633.smt2.gz` | 143396 | `17504e4aee3db12561eb371d5c031a6d8f62687ca71f27068cca7a3331c7c44b` |

Trusted artifact-autofix job137986888274 on actual head412041f073 failed
2026-11-23T20:22:23Z. Each subject's HeadObject returned403. It produced no
trusted upload/replay receipt, manifest or pointer commit. A403 does not
establish object absence. `ARTIFACT_DIAGNOSIS.json` and the full lossless decoded
job log retain the facts. The diagnostic gzip itself is83001bytes and is also
selected by the same policy; its hash is in that diagnosis.

Publication to the artifact store is **pending canonical access repair**.
Do not retry unchanged credentials, alter policy/helpers/workflows, add an
exemption, export credentials, fabricate a receipt, remove the inspected local
bytes or delete remote objects. Do not rename/split/recompress these selected
published subjects to evade selection. After an authoritative repair, use
maintained migration and require genuine receipts for these exact hashes and
sizes before removing only their tracked copies. Preserve hydrated local bytes.
The PDF is kept by the real policy. No admitted performance row is added.

The previous117571164-byte compiler archive remains independently pending; it
was not retried here. The old incomplete active PCC8d8628 also remains blocked
on its separately documented supported record-correction path. New statement
989b50 has no Lean proof object; it does not withdraw or replace any old record.
All standing remains unsigned/unverified. [pcc:unverified]
