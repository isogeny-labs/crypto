# Publication handoff

Regular PR: https://github.com/isogeny-labs/crypto/pull/3422.
First published code/evidence commit:
2432e77f9b4bdc8f5f7565650476a62b5ad733c6; exact local/remote tree:
aabe144ba5de01a1e493066cbe3bd8b681bdec6a.

Current main advanced to 319611cb405c91299a983f743523f9805884a817.
The actual PR response reports mergeable=false, mergeable_state=dirty.
The differences relevant to this component are concurrent additions to
docs/session-logs/2026-11-04.md and pcc/ledger.jsonl. AGENTS.md and the frozen
sampler/cohort are unchanged. Both sides' additions must survive; no side of
the ledger may be selected to discard the other history.
The Git merge preview identifies only pcc/ledger.jsonl as conflicted; the
session log merges automatically.

Follow current AGENTS conflict handling: request Copilot to merge the base into
the PR without rebase/force-push, preserve both histories and all claim records,
and use the ledger tooling. Do not race the pending resolver or edit conflicted
files. Inspect the actual resolution against both parents and the current
main-prefix chain before acceptance. A conflicting frozen claim or ledger
record must be reported, never silently resolved by side selection.

The first publication mistakenly gave the new native research CI workflow a
pull_request trigger. Current AGENTS restrict research workflows to main or
manual dispatch, so this amendment changes that trigger only. The original
MANIFEST.sha256 and BUILD.json remain historical immutable development receipts
for 2432e77; CURRENT-MANIFEST.sha256 binds the amended workflow and unchanged
native sources/raw controls. No C++ behavior or scientific data changed.

Local tests remain producer assertions, PCC-20261104-ab1619 standing unverified.
No current-head CI or independent review completion is inferred from those
receipts. Resolve conflicts, check exact-head gate/size/artifacts and the public
rules, obtain fresh independent Codex review without unresolved findings,
then enable the authorized auto-merge with an expected-head guard when offered.
Do not enable auto-merge while those conditions are pending. [pcc:unverified]

The publication entry was appended to the clean session log before requesting
Copilot. Leave the conflicted ledger untouched while resolution is pending;
append the resolution outcome after the resolver finishes.
PR #2981 is completed and its old watcher remains stopped.
