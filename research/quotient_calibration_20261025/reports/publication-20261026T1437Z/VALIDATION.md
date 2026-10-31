# Publication follow-up validation

Checked 2026-10-26 after merging remote head
`9cc1cb489dc209730ebc46a40cc2e9384231ce0a` and main
`f25c4602a` without rebasing or force-pushing.

| Check | Observed outcome |
| --- | --- |
| Artifact upload and fresh-cache readback | Four successful operations; exact 67,540-byte archive and SHA-256 match [STATUS.json](STATUS.json). |
| Artifact-tool suite | `python3 -m unittest tools/artifacts/test_artifacts.py`: 14 tests pass. |
| Native package compilation | `cargo check --release --locked --manifest-path research/quotient_calibration_20261025/native/Cargo.toml`: exit 0. |
| Focused workflow | Actionlint passes; repository workflow generator adds the store pull and is idempotent for this study. Sparse checkout includes the artifact configuration and tooling. |
| Frozen source scope | `git diff --exit-code 055b53c2661f120d20505a1be640a68c8ca1e86f HEAD --` the native package, fixtures, protocol and provenance: exit 0. |
| Markdown links | All 36 local links checked before this validation note was added resolve in the checkout or tracked index. |
| Problem index | All 48 main-branch rows preserved verbatim, plus the two unique study problem rows. No conflict markers remain. |
| Publication data | Manifest, approved local archive and remote-readback receipt agree; no AWS access-key pattern in the publication files. |
| Focused whitespace checks | Pass for study, workflow and problem-record changes. |

## Existing artifact-policy failure

`python3 tools/artifacts/artifacts.py check origin/main HEAD` exits 1 for
nine unrelated large data files, all added by the existing owner-authored
remote artifact-migration commit `9cc1cb489`. These paths are unchanged by
this follow-up:

- `benchmarks/20261018-cofactor-cleared-ic/e6_yield_ratio.png`
- `benchmarks/20261018-cofactor-cleared-ic/e7a_h4_s_by_arm.png`
- `benchmarks/20261018-cofactor-cleared-ic/e7a_h72_s_by_arm.png`
- `benchmarks/20261018-cofactor-cleared-ic/report.pdf`
- `research/isogeny_conductor_gap_round5_20261012/e6/records.jsonl`
- `research/isogeny_conductor_gap_round5_20261012/report.pdf`
- `research/isogeny_conductor_gap_round5_20261012/sessions/e7a_h4/records.jsonl`
- `research/isogeny_conductor_gap_round5_20261012/sessions/e7a_h72/records.jsonl`
- `research/isogeny_conductor_gap_round5_20261012/sessions/e7b_k0/records.jsonl`

The same command against the preexisting remote head reports 13 violations;
the main merge removes four from that difference. The nine remaining files
were preserved rather than changing the owner's unrelated migration. No
additional dataset was uploaded and no artifact-policy exception was added.
This gate is reported as failing.

## Merge and CI scope

Copilot was first asked to merge main in
[comment 6409312203](https://github.com/isogeny-labs/crypto/pull/2569#issuecomment-6409312203)
at 05:58 UTC. After more than eight hours without a response or resolution
commit, the documented fallback was recorded in
[comment 6418763179](https://github.com/isogeny-labs/crypto/pull/2569#issuecomment-6418763179).
The main merge's only conflict was `problems/README.md`; both additions and
their current status were preserved. No frozen evidence conflict occurred.

The existing native source, historical receipts and challenge stream did
not change. The original full-library result remains the pinned historical
run described in [the original validation record](../run-20261025T1845Z/VALIDATION.md);
it is not a passing full-library result for current main. No new experiment
or benchmark ran for archive publication. Hosted CI and review are checked
on the pushed head and recorded on PR #2569. Earlier hosted source-preflight
jobs executed zero steps because of the account billing/spending limit.
