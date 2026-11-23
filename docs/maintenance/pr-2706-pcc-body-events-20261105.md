# PR #2706 PCC metadata events

The failed gate (run 41978809708, job 127083857039) reported seven uncited strong-claim paragraphs in the pull-request body. This maintenance explicitly marks producer assertions unverified without changing frozen evidence. [pcc:unverified]

The default pull_request workflow events omit edited. Added edited and ready_for_review alongside opened/synchronize/reopened so body lint responds to real metadata corrections. Every verifier, ledger-prefix check, canary, command and permission is preserved. This is the existing PCC gate; no post-main workflow is restored to PRs. Actionlint accepts the workflow. [pcc:unverified]

No new scientific trial, hash-chain rewrite, claim promotion, review override or check bypass is introduced. All required and applicable checks on the exact new head remain required. [pcc:unverified]
