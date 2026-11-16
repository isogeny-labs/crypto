# Validation scope — 2026-11-05

This is a literature/derivation deliverable and a small Rust wrapper around
the existing native identity issuer/checker. The four APS mechanisms remain
unimplemented proposals. No APS solve, exhaustive curve search, performance
experiment, EKS session, timing comparison or full root-library run was
launched [pcc:unverified].

## Exact boundaries

- The proposal assertion is `PCC-20261105-aca921`, kind `statement`.
- `PCC-20261105-ac1e93`, also a statement, records the initial identity
  issuance/replay with unsigned artifact attestations. Native standing is
  **unverified**; native proof checking reports `not_covered`.
- Both records' native ledger additions remain private. The PR includes
  only their scoped claim JSON; publication is not independent verification.
- Main source pins are in SOURCES.md. Content SHA-256 pins of the wrapper,
  manifest/lock, reused modules, certificates and replay receipt are in
  [source-sha256.txt](source-sha256.txt).
- The final wrapper was rebuilt with its final manifest and formatting.
  Its replay output is byte-identical to [identity-receipt.json](identity-receipt.json).

These are producer observations [pcc:unverified].
The final source, document and wrapper-validation assertion is
[PCC-20261105-9d5a6d](../../pcc/claims/PCC-20261105-9d5a6d.json), also an
unverified statement. It does not promote the earlier records.

## Native identity checks

For each of `IDC1h9e131e5cd24a395f` and `IDC1ha14548fc2b5ce5c3`, the native
issuer and checker accept 32 certificate-derived points and 32 additional
producer replay points over the checker's field $2^{61}-1$. For each record,
the issuer refuses an RHS modified by adding one, and the checker refuses a
changed digest. The wrapper also binds the record to its exact expression
tree [pcc:unverified].

The checker is reused, not authored in this study. The producing session
issued and invoked it, so this is **not** an independent signed verdict.
`proof: null` is retained. The records certify neither the chart's entire
conditional lifting argument nor tuple-frontier coverage, and do not turn
finite-field identity evaluations into a proof over every field
[pcc:unverified].

## Reproduction

From the repository root with Rust and the manifest's locked dependencies:

```sh
cargo build --locked --manifest-path research/aps_leads_20261105/Cargo.toml
# Use the built binary's check command on the committed records:
research/aps_leads_20261105/target/debug/aps-lead-identities check research/aps_leads_20261105/certificates
cargo fmt --manifest-path research/aps_leads_20261105/Cargo.toml --check
cargo +1.98.0 clippy --locked --manifest-path research/aps_leads_20261105/Cargo.toml -- -D warnings
cargo +stable clippy --locked --manifest-path research/aps_leads_20261105/Cargo.toml -- -D warnings
sha256sum --check research/aps_leads_20261105/source-sha256.txt
bash research/aps_leads_20261105/render-report.sh
```

The binary path assumes the standalone package's default target directory;
when `CARGO_TARGET_DIR` is set, use its `debug/aps-lead-identities` instead.
Local builds used offline resolution, Rust 1.98.0 and stable Rust 1.99.1.
Final format and strict Clippy checks passed under both toolchains, using
the root crate's existing fixed-size-chunks policy for the imported SHA-256
source [pcc:unverified].

`issue` is only for reproducing records in a **new empty directory**. It
refuses to overwrite an existing record. Normal replay uses `check`.

## Failures and corrections preserved

1. The first standalone build ended 101: the wrapper manifest omitted `hex`,
   used by the unchanged native checker. Adding that dependency fixed it.
2. Initial Rust 1.98 Clippy ended 101 on four fixed-size `chunks_exact`
   warnings in the imported, unchanged SHA-256 module. Standalone packages
   do not inherit the parent package's lints. The new manifest now matches
   the root's existing `chunks_exact_to_as_chunks = "allow"` policy; the
   imported module and root policy were not edited.
3. The first PDF build ended 1 after changing the SVG filename inside
   `includesvg`; LaTeX then looked for an SVG with a PDF suffix. A small
   Pandoc image filter now selects the DOT-rendered PDF before LaTeX output.
4. Initial prose lint found four paragraphs lacking an explicit standing
   tag. Tags were added, without changing the checker.

Original compiler/dependency and document logs stay private. The above is a
normalized account; no prior failure is recast as a pass [pcc:unverified].

The initial whole-worktree policy snapshot could not be completed because
its partial clone needed an unavailable promisor Git object. No repeated
fetch was launched. For bounded changed-file validation, the unmodified
repository size script and artifact policy were applied in a complete local
Git checkout containing the exact proposed additions. Both pass. This checks
the proposed file delta; it does not replace required hosted checks or claim
a full-worktree workflow pass [pcc:unverified].

## Visual and graph review

The final PDF has four pages. Each page was opened and visually inspected
for equations, chart exceptions, certificate IDs, captions, citations and
legible labels. Text extraction retains all four leads, the pending
checklist and primary references. The SVG and embedded PDF derive from the
same DOT source. No measured series exists for a quantitative plot
[pcc:unverified].

The main scoreboard and progress-timeline source files were read. They stay
unchanged because this search adds proposals, not a verified transport edge,
curve or measured cost. No benchmark copy or progress metric is added
[pcc:unverified].

## Pending follow-ups

- Obtain independent proofs/replays for the L1 chart and L3 frontier arguments.
- Implement each lead with exact witness, recovery and refusal checks.
- Complete the separately blocked checkpoint and measurement admission work.
- Freeze held-out protocols before any complete-cost experiment.
- Reconcile and authorize the protected native ledger separately; keep this
  proposal's producer standing unverified and merge automation off meanwhile.

The earlier APS studies and their source/test boundaries are unchanged.
No previous full-suite or component result is represented as a new-head run.


## PCC gate type repair

The original hosted PCC gate failed on the two incorrectly typed attachments;
the separate validation gate, size and artifact checks passed on the original
head. [CORRECTION.md](CORRECTION.md) retains the exact failure, old records and
replacement map. The current native verifier rejects each old record with
exit1 and reports each unsigned-attestation replacement `not_covered`, exit2.
This is a correction to proof attachment types, with identical scientific
statements, scopes and identity artifact bytes [pcc:unverified].

The original validation statements describe the first published source/report
boundary. They are not new-head APS tests, a full library pass, or a new
measurement. The corrected report was rendered and visually inspected
separately; source-sha256.txt and the original identity receipt remain unchanged.


The repair used the unchanged public PCC verifier and its existing tests:
59/59 sabotage-canary classes and 12 native tool tests passed. Scoped
`verify-all` over the three live APS statements returned2 (`not_covered`),
with no failed proof; this is not a full-main claim-set verification or a
signed verdict. Historical malformed records still return1. The corrected
four-page PDF was inspected separately [pcc:unverified].
