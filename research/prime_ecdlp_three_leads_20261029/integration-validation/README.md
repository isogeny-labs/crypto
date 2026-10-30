# PR2868 integration validation

This is software/schema/proof validation of the main integration, not a
scientific run. The integration commit is
`497698e279eee351b775d8dad6696f35cbef468a`, with parents
`9a29c1d6cbebe1db669be59119adc4ce69bc8a97` (authored work) and
`78f84234e18facfc46be52741d7fa6acadc92b88` (main).

Copilot had not responded or changed the branch after about an hour. The
AGENTS.md reasonable-wait fallback was explained in PR comment6504324054.
The only conflict was the daily session log. Every paragraph from each parent
was verified present, and the prepared PKM_GOALS_SESSION.md handoff was appended.
Against main, the integrated tree has exactly the original61 authored paths;
all60 non-log blobs match the authored parent byte-for-byte. Main's Rust
library and dependency changes are retained. No lockfile was manually edited.

Fresh commands and outcomes are in receipt.json. The native backlog validator
is the previously built executable (SHA256
`4c5256a9e6f66e5b02e47a5650c1e0e553dea2b0f226a304dcc77fa1341a2e15`);
its `src/bin/backlog.rs` and `src/bin/backlog/` sources are unchanged by main.
It validates the card without posting, leasing or launching any work. Old validation/ and
planning-validation/ receipts remain immutable and retain their original
revision and scope. The prior full-library run remains capped incomplete
(5,232 passed,0 failed,123 ignored,one unchanged test unfinished); it is not
a full-library pass on this integration. Both large raw logs are still local
with their original hashes, NOT_UPLOADED. No scientific panel has run, no
benchmark/ledger value is promoted, and the goal card remains blocked/unfunded.

The first rustfmt invocation omitted the session's RUSTUP_HOME selector and
failed before formatting. The retained initial log records this setup failure;
the correctly selected toolchain passes. Empty successful-command logs are
represented by exit status and SHA256 in the receipt. Local policy checks do
not replace GitHub's required gate,size,artifacts checks on the final PR head.
