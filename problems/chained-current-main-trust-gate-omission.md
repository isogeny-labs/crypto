# Canonical trust and artifact gates missing from the fetched main tree

- **Status:** open
- **Kind:** missing repository dependencies; existing access/tool failures; ledger divergence
- **Affected work:** chart10/audit11 source publication and canonical PCC intake
- **Next action owner:** research lane for scoped evidence; repository maintenance for canonical trust/gate repair
- **Last updated:** 2026-11-17

## Problem

Canonical PCC admission and the original merge set cannot be established against
fetched main d06c4a82808701ede1aca1a64ebb7f861b0eade5 because its keyring,
PCC/artifact tooling and several required workflow files are absent. The ledger
remains and diverges from this session's source ledger after1216 entries. This
does not block making native sources and unsigned evidence reviewable. [pcc:unverified]

## Evidence and uncertainty

- Observed: preflight bd5fdc retains the entire13157-path tree list and exact tree0b135d7c119895e62a73a6ac2b858343c7284feb. Fifteen workflows remain; root AGENTS, keyring, PCC tool, artifact config/tool and PCC/artifacts/large-files workflows are absent. [pcc:unverified]
- Observed: PR4455 original0729 and final7014 have eight37/66 theorem outputs0 failing0 trusted; required CI fails. Complete logs, four provider ZIPs and five ledger histories are retained under chart10's `evidence/pr4455-whole-CI-and-fork-04/`. Original migration/validation HeadObject403 is not an authenticated upload/replay. [pcc:unverified]
- Observed: an earlier local prefix command returned0 while printing that the unmaterialized base ledger was absent. After the exact promisor blob was materialized the unchanged tool returned1 at1216. That earlier0 is rejected as prefix validation. [pcc:unverified]
- No mathematical obstruction or attribution for external main changes is established. Archived original tools are custody data, not replacement trust roots. [pcc:unverified]

## What would unblock it

- [ ] Repository-maintained canonical keyring, unchanged trusted verifiers and original artifact policy/tooling are available on the actual intended base.
- [ ] Original ledger/keyring checks compare actual hydrated bytes; any fork reconciliation is explicitly accepted without side choosing, dropped records or rechain.
- [ ] The original whole-object store upload/authenticated replay and required gate/size/artifacts set pass on the final source head, with blocking review resolved.

These conditions enable canonical admission and normal merge evaluation. They
do not supply Jacobian maps, dual orientation or transfer cost results.

## Next discriminating step

Read the exact new main tree when substantive repository maintenance lands and
compare whole trust/policy/ledger bytes and original gate outputs. Do not retry
unchanged403 calls or use other credentials/providers. Meanwhile implement the
unblocked balanced-divisor/Richelot chart component under frozen protocols.
Paid STOP, locked vault and original full LP workload requirements persist.

## Resolution and history

Unresolved. The source successor preserves all original records in a whole
archive and appends no record to the canonical ledger. A source PR or merge
does not establish canonical PCC or store admission. [pcc:unverified]
