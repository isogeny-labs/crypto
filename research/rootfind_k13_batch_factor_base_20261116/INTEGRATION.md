# Integration against current main

The candidate history through `187ac97b8f7` was merged into a sparse branch
at main commit `672067a75abcb2ad3f6b78cc44b9e36ec514d2da` after PR #4433
merged. The Koblitz source merged automatically. This merge adds no measured
timing or end-to-end solver result. [pcc:unverified]

The sole content conflict was `pcc/ledger.jsonl`. Main's 1,215 ledger entries
were retained byte-for-byte, and the existing `PCC-20261116-7748c7` claim
file was appended at sequence 1,215 with the same claim digest. The resulting
entry hash is `242e98ad23570695bb37984d395974c61873babf1310147b6980bdc4b5ac2dee`.
The chain and base-prefix checks both returned no errors; the earlier candidate
ledger entry remains visible in the merge parent's committed history.
[pcc:unverified]

The full `python3 tools/pcc/pcc.py ledger verify --base isogeny/main` command
exited 1. Its [stdout](integration-ledger.stdout),
[stderr](integration-ledger.stderr), and [exit](integration-ledger.exit) are
retained. It reported 125 claims without the required `claim_type`. The
[base check](integration-ledger-base-check.json) confirms that every named
claim exists in this main commit and lacks that field there. This is a
mainline PCC gate impediment; the candidate claim is not one of the 125.
No pre-existing claim was rewritten. [pcc:unverified]

`rustfmt --edition 2021 --check` on the auto-merged Koblitz source and
`pcc lint-prose` on the source-bound report passed. The source-bound test
receipts in [CHECKS.json](CHECKS.json) predate this merge; the newly combined
source has not completed a local Rust suite. The next verification gate is
the merged source's focused batch tests and a passing mainline PCC validator
after the upstream claim-type issue is resolved. [pcc:unverified]
