# Independent S3 hosted continuation

This narrow infrastructure PR prepares the requested separate-host check
without modifying either divergent PCC ledger. A new path-filtered
research workflow runs on `main` after merge, using already published
frozen source. The earlier failed manual dispatch remains unchanged.
No new hosted run has occurred. The prospective protocol is being frozen
before the ready PR is opened. [pcc:unverified]

The one planned hosted job compiles the original C++ arithmetic through
a fixed S3-only entry and the separate Rust implementation. Both use the
same fixed six-field known-answer population. It logs source/binary/
compiler/host identities and every complete output and exit code, with
no timing claim or outcome-dependent retries. Standard public runners
remain free under the checked official billing policy. No secrets,
paid compute, cache, artifact upload or recurring schedule is introduced.
[pcc:unverified]

See [prospective protocol](PROTOCOL.md), [canonical record](protocol.json),
the new workflow, and [source review](source-review.json). The existing
[audit PR3743](https://github.com/isogeny-labs/crypto/pull/3743) retains
all fourteen requirements and the actual Lean proofs. This PR changes
only the new workflow/continuation files. Main's ledger, old frozen
validator, old protocol/export and original manual workflow are unchanged.
Prepared code is not evidence of remote execution. [pcc:unverified]
