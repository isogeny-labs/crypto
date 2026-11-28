# Third original-root build blocker

## Finding

The failed Prime31 checkout and the pinned current main contain exactly the same malformed `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs`. The source also matches the base used before the four-line framework restoration. This blocker is inherited and independent of that restoration; it is not an observed rational-chain test failure. The complete direct module fails parsing, while both parents of the merge that introduced the malformed file parse successfully. [pcc:unverified]

No repair, source substitution, new CI run, or cryptographic computation was performed in this audit. A passing parser check is not a type check or an executed test result.

## Exact source identities

| Role | Commit | File Git blob |
| --- | --- | --- |
| Failed dedicated CI checkout | `14a9f021b630fec0a7dd38cb724d71d68c99626b` | `f001a1b641abd48bb0130c1a9a246e37c28bcc3a` |
| Pre-restoration base | `f0ca664eca0081a30adfcc79dcf62a32de40d2af` | `f001a1b641abd48bb0130c1a9a246e37c28bcc3a` |
| Pinned current main | `816f9c3a942efed569049618c056d12a0c56d693` | `f001a1b641abd48bb0130c1a9a246e37c28bcc3a` |
| Introducing merge | `77d388738a48eb5a6e2abd80636c2e31cceab254` | `f001a1b641abd48bb0130c1a9a246e37c28bcc3a` |
| Merge parent 1 | `a9d68fbd9c2dc5b6db267746c483febd22cb5501` | `12758d4edc3f1d5f6c6c66ebb32397d04c991af7` |
| Merge parent 2 | `0d4721abfeb0c402cb7de6227e0f2108101162dc` | `89a190d53bcd20a8141c219db2f0f8fd963b9ce2` |

Each source was fetched at its exact commit through GitHub, materialized without normalization, and checked against its Git blob hash. The first four sources are byte-identical: 46,830 bytes and 1,233 lines, with SHA-256 `94f99234a3952dafac3f2726f3cfe045d1313718ff9b3b74f48dfe3c7ca861f3`. No upstream repair is present at the pinned main. [pcc:unverified]

The bounded path-history query identifies the introducing merge as the newest change to this file. Its metadata dates it to 2026-11-23T02:12:27Z with title “Merge main into prime-ic-mitm-20261118”. This identifies the commit where the file became malformed, not the tool or person responsible for its contents.

## Parser and preservation observations

The parser command was:

```sh
rustfmt --edition 2021 --emit stdout --config skip_children=true <exact-source-file>
```

The executable reported `rustfmt 1.11.0-stable (0ef1bc3860 2026-11-09)`. Child-module traversal was skipped; the entire supplied direct module was parsed.

| Source | Lines | Parser exit | Result |
| --- | ---: | ---: | --- |
| Failed/base/current/introducing merge | 1,233 | 1 | Delimiter errors reproduced |
| Parent 1 | 1,474 | 0 | Empty parser stderr |
| Parent 2 | 1,539 | 0 | Empty parser stderr |

The current-file diagnostic reproduces the mismatch at line 640 and unclosed input at line 1233. It includes the compiler's phrase “another 198 unclosed delimiters”; that phrase is retained as a diagnostic, not an independently enumerated defect count. [pcc:unverified]

The differences are broader than closing braces. The merge loses derives, an `else` branch and substantial test code, while also mixing parent differences. Git's default `diff --no-index --numstat` reports 154 additions/395 deletions relative to parent 1 and 173 additions/479 deletions relative to parent 2. These are the stated diff algorithm's line counts, not a minimal repair size. A separate exact line-subsequence check confirms that the merged file is not merely a pure deletion subsequence of either parent. The parent files each contain six `#[test]` attributes; the malformed merge contains one. This audit does not infer how many tests would execute after a repair. [pcc:unverified]

The parent-comparison API returned merge base `86f17c632710b1cd6c6ed06a0a9dc6952293cca1`; fetching this file there returned 404. No complete intended parent union has been reconstructed, and neither parent is proposed as a blind replacement. An attempted local writer encountered the absent source response before writing a merge-base snapshot; the bounded comparison then proceeded without it.

## Dedicated CI scope

[Run 45113871392](https://github.com/isogeny-labs/crypto/actions/runs/45113871392), job `137492117014`, is bound to head `14a9f021b630fec0a7dd38cb724d71d68c99626b`. The primary job metadata reports failure completed at 2026-11-23T04:56:20Z. Source capture and evidence upload succeeded. Each of the five independent command steps failed: spec regression, malformed-plugin regression, rational-chain tests, deterministic receipt generation, and native identity issuance/replay. [pcc:unverified]

The enclosing compile errors occurred before the new tests or issuer could execute. The final step contains issuance followed by replay under `set -e`; failure of the issuance build prevents the replay command. Nothing here establishes a test assertion failure, a new native identity, or a passing whole-root build. The parent coordinator retains the full workflow logs and artifact separately; this package is the source-provenance and parser audit. [pcc:unverified]

## Integration boundary and next action

Retain the compiled-input freeze and all prior failure receipts. Record this as the remaining original-root build blocker. Any future repair needs a separately reviewed reconstruction preserving both parents' relevant implementations and tests, followed by normal compilation and correctness checks. This audit does not broaden the Prime31 source patch, remove the binary-hyperelliptic module, disable tests, or substitute a smaller crate while calling it original-root validation.

The compact integration files are `AUDIT.md`, `receipt.json`, `metadata.json`, `source-checks.json`, `current.parser.log`, the two empty parent parser logs, `merge-stats.json`, and `subsequence-check.json`. Local source snapshots and full text diffs are reproducible audit inputs identified by their Git blobs; they need not be duplicated in the final research record.
