# Registration transport and published receipt binding — 2026-11-23

**Unsigned administrative record [pcc:unverified].** The local HTTPS git push failed because no username/credentials were available. The connected GitHub API remained available. This transport correction binds the statement to the actual published receipt commit and does not add a research execution or upgrade standing.

## Preserved local attempt

The original local receipt commit is `90522ccbf6c70e9c0a7a5b51c14b6a7a6c7a6a9f`; its tree is `ce74e476321857abd077244b8b90137707f379af`. The original local registration commit is `485a0bf233954219ab735d2aee0eeaf99143b8e5`; its tree is `ba2797ed96b8925458684308aae611497cda2939`. Its statement `PCC-20261123-c7e76b` binds the local receipt commit. Those commits and the original working tree were preserved without reset or rewrite.

[evidence/unpublished-transport-20261123/](evidence/unpublished-transport-20261123/) retains exact `git cat-file commit` bytes for both commits, the exact `git diff --binary 90522ccb 485a0bf` patch, and the old native registration output. Native SHA-256 values are in its `SHA256SUMS`. These are small historical records of the unpushed attempt; they are not active PCC claims or ledger rows. The old patch must not be applied to the new active worktree or ledger.

## Actual published receipt

The API published receipt commit is **`d532700090da7a3d35e5c1b03f081a0783dbd6ac`**, with the exact same tree `ce74e476321857abd077244b8b90137707f379af`. The branch was advanced by a non-force update from expected source head `b4b87e654950ad57d264e518ec6bb190efd88bba`. The actual API commit was fetched before creating a fresh detached sparse worktree. The already-bound receipt and all CI evidence bytes remain unchanged.

## Fresh unsigned registration

The existing native writer previewed the new claim, its output was inspected, and append used the reviewed canonical claim hash and identical creation time, statement, session and model. The published receipt commit supplied the initial ledger; the old local registration row was not copied.

| Field | Value |
| --- | --- |
| Claim ID | `PCC-20261123-c7e76c` |
| Created at | `2026-11-23T20:42:10Z` |
| Kind / claim type | `statement` / `data` |
| Producer commit | `d532700090da7a3d35e5c1b03f081a0783dbd6ac` |
| Receipt | `research/lp_graph_algorithms_20261123/evidence/native-admission-20261123/README.md` |
| Receipt SHA-256 | `fa889d1bc52561d4155371034401083634f09de1ec57cbc49bbc209510ffe889` |
| Canonical claim SHA-256 | `159e292d4bb26867ecd759e73f0d5240ebdb4fce26cf723ce77564d28f436113` |
| Ledger prefix count | 1,827 entries, preserved byte-exact |
| Previous ledger tail | `8aa9d5f9b3b5607066439e473e6427fe3b1edf25bbe86533e7ef679ce9c61c1c` |
| Appended sequence / final count | 1,827 / 1,828 |
| Appended entry hash | `88f8ab42dd8ca69561ce94f71d3732665cb4c1b3726f0752c5359b56ee8da436` |

The exact native output is [PCC_PRIME31_ADMISSION_20261123.json](PCC_PRIME31_ADMISSION_20261123.json). The actual claim lives at `pcc/claims/PCC-20261123-c7e76c.json`. The original CI-tested source remains `b4b87e654950ad57d264e518ec6bb190efd88bba`; the new `producer.commit` identifies the published receipt revision, not another test execution.

No signature or verdict was created, no keyring was modified, no scientific verifier ran, and the registration writer executed no Python. The metadata publication commit is separate from the already-published receipt commit; its eventual identity does not change the receipt-bound producer identity. Scientific claims, natural capture, full-root qualification, durable full-archive custody and complete cold cost remain outside this unsigned registration. **[pcc:unverified]**
