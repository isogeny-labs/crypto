# F3 original record custody

Status: exact defective-session disposition approved and active; original 24-record object recovery remains open.

The unchanged smoke-k0n61 session capsule declares 24 records and binds SHA256 `2aa97366f892f7d1a2f4c887e9e5543397ff6d047016048932efaace40e92843`. Its original artifact manifest instead binds the 16-record object `5c937b749f9bb20f6541b19c9ed9e21c16f2392e103ee3b47e0995a272c629fb`. A fresh native audit reports exactly records_hash, record_count, status_counts and last_seq. Each of the 16 retained records passes its per-record checks; the complete capsule fails. [pcc:unverified]

The original producer receipt claiming a 24-record audit is retained as a historical assertion. It does not supply the missing record bytes. No recovered pointer object belongs to this session, and none can replace its records.

Checked recovery locations and exact five-file pins are recorded in [the inactive proposal](../docs/ecbench/defect-proposals/20261116-f3-m83/proposal.json). This search does not establish absence from every possible source.

Exit criteria: recover original bytes matching the capsule digest and replay the integrity audit; alternatively obtain the owner's explicit disposition under [the existing defective-session policy](../docs/ecbench/DEFECTIVE_SESSIONS.md). The inactive proposed row changes no audit behavior. If approved, every audit must continue checking all five exact file hashes and must fail with exactly the four pinned problems; classify it as defective, never integrity-passed. No original artifact is edited, and no missing record is synthesized.

## 2026-11-16 owner disposition

The owner approved the exact proposal in PR #4439. The active catalog now pins the five original files and four expected failed checks under the existing policy; the capsule is classified defective, never integrity-passed. The [decision receipt](../docs/ecbench/defect-proposals/20261116-f3-m83/decision.json) supersedes the pending-disposition status above. Original 24-record recovery remains open; the proposal, controls, producer receipt and frozen artifacts remain unchanged.
