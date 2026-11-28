Post-publication CI triage: RR5 B677

Run: https://github.com/isogeny-labs/crypto/actions/runs/45118811731
Scientific core: 5e536ed93302def2122968aa880b22c4b675d516
Completed conclusion: failure; updated 2026-11-23T05:47:35Z.

Observed outcomes

1. rr5-component-qualification (job 137507635035) succeeded.
   All 32 native tests and all 8 public-example tests passed against the
   declared production source closure. The identity/false-expression step
   and artifact upload succeeded. Compilation preceded the logged per-run
   2 GiB / 180-second limits. These results do not establish a completed
   B677 relation extraction or full root-library qualification.

2. rr5-development (job 137507635321) failed.
   Both root cargo test commands exited 101 at Rust parsing errors in
   src/cryptanalysis/binary_hyperelliptic_index_calculus.rs.
   The errors cite 640:94 (mismatched delimiter, related close at 656) and
   1233:2 (unclosed delimiters at EOF). No root tests or original-root
   example tests ran; the following B4 prototype step was skipped.
   This hosted failure is separate from the earlier local root-library-01
   dependency allocation error and timeout exit 124.

3. The failed source was unchanged by this PR relative to the stated base.
   Core 5e536ed93302def2122968aa880b22c4b675d516
   Base 816f9c3a942efed569049618c056d12a0c56d693
   Both name Git blob f001a1b641abd48bb0130c1a9a246e37c28bcc3a.
   Blob size: 46830 bytes.
   SHA256: 94f99234a3952dafac3f2726f3cfe045d1313718ff9b3b74f48dfe3c7ca861f3
   The two connector-returned full texts also agree exactly. Missing struct,
   method, branch and loop delimiters are visible at the cited source
   locations. This is clear pre-existing malformed source.

Receipt layout

- triage.json: observations, provenance, exact source comparison, log ranges.
- run.json / jobs.json: observed workflow metadata and step conclusions.
- component-log-excerpts.txt / root-log-excerpts.txt: selected complete log
  lines, with original timestamps and ANSI escapes preserved. Each file
  concatenates the non-overlapping source ranges declared in triage.json;
  no gap markers were inserted into the raw excerpt files.
- source-excerpts.json: exact source slices and the fetched immutable URLs.
- SHA256SUMS: hashes of every receipt file listed above, plus this README.

This is a separate administrative receipt created after the report/PCC
artifacts were frozen. No scientific run, CI retry, source repair, report
edit, claim change or ledger append was performed for this triage.
