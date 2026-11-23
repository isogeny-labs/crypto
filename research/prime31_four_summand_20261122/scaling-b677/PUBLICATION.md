# B677 publication and subsequent CI disposition

The inspected report and evidence were published as commit `98c5d4e93e9122ec1b85c5ed2f7b5e03f8134b87`, tree `4235f5b4b794800bf1c5ca6b9ccfa99c9ab8fa27`, exactly matching local commit `16c3901d455f506625cd491f8940986cccce83f8`. Its parent is scientific core `5e536ed93302def2122968aa880b22c4b675d516`. The report PDF and all frozen scientific/PCC artifacts remain unchanged. [pcc:unverified]

[PR #4974](https://github.com/isogeny-labs/crypto/pull/4974) is the substantive continuation of merged tracking PR #4967 and merged qualification PR #4953. It is draft because the full-root gate and admitted extraction task remain unfinished.

## Terminal hosted CI observation

[Run 45118811731](https://github.com/isogeny-labs/crypto/actions/runs/45118811731), attempt 1, ran the exact scientific core above. It completed with overall conclusion `failure`. [pcc:unverified]

| Job | Observation | Scope |
| --- | --- | --- |
| [137507635035](https://github.com/isogeny-labs/crypto/actions/runs/45118811731/job/137507635035) | Success: all 32 native tests, all eight public-example tests, identity check and artifact upload | Component qualification only |
| [137507635321](https://github.com/isogeny-labs/crypto/actions/runs/45118811731/job/137507635321) | Root library and original-root example calls exited 101 during parsing; subsequent B4 prototype skipped | No full-root tests executed |

These are unsigned observations of the hosted job records. [pcc:unverified]

The failing source is `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs`. Both base `816f9c3a942efed569049618c056d12a0c56d693` and scientific core resolve it to Git blob `f001a1b641abd48bb0130c1a9a246e37c28bcc3a`. The returned source texts are identical. Rust reports mismatched delimiters at 640/656 and an unclosed delimiter at EOF 1233; source inspection finds multiple unfinished declarations and blocks. These errors predate this change. The complete separate triage receipt is in [evidence/ci-triage-01](evidence/ci-triage-01/). [pcc:unverified]

This hosted parser failure is distinct from the local root call retained in `evidence/root-library-01`: the local call reported aws-sdk-s3 metadata allocation failure and then outer timeout, exit 124, before tests. Neither root attempt supplies a passing full-library result. [pcc:unverified]

## Next technical gate

The fixed B677 driver remains compiled but unexecuted. Follow [PROTOCOL.md](PROTOCOL.md): obtain a functioning checkpoint transport and authenticated admitted EKS route, build a digest-pinned image containing the tested payload, then run the fixed structured-positive request and cofactor reference once under the frozen limits. H0/W0 remain conditional on a complete structured positive. Signed row replay, authentic-RHS rank and all cold phase costs remain required before any full-solver comparison.

The repository follow-up is scheduled for 2026-11-22 22:41:16 America/Los_Angeles. It must read the actual current PR state, preserve frozen evidence, address changes caused by this lane, and stop maintaining the PR after merge or closure. It does not authorize bypassing scientific admission or extending a frozen budget.

## Follow-up status check (2026-11-23 06:40 UTC)

The live draft PR #4974 was inspected at head `29919fc78cb2d8af3cf9265d79127d6f4e4215af`, with no review submission, review thread or PR comment. There was no new scientific test, independent verdict or EKS extraction. [pcc:unverified]

[Hosted pcc-gate run 45121025162](https://github.com/isogeny-labs/crypto/actions/runs/45121025162), [gate job 137514587938](https://github.com/isogeny-labs/crypto/actions/runs/45121025162/job/137514587938) failed at the unchanged `tools/pcc/test_pcc.py` unit suite: 49 tests ran, three raised `NameError` (`reasons` twice and `q` once), and one was skipped. Both base and PR head use the exact same test blob `47be6b2e761d0ca77423be7351e94ac8a6aebbbe`. Sabotage canaries, ledger verification, changed-claim validation and strict PR-body lint passed. The separate advisory whole-file lint identified 73 older strong-word occurrences in the shared November 22 session log; its workflow step is continue-on-error. Do not weaken the PCC test gate to convert this into a pass. [pcc:unverified]

The original checkpoint receipt remains exit 1 for unavailable upload credentials/transport; the authenticated EKS caller, supported fixed-command admission, and source+binary+OCI-digest-pinned RR5 image are not present. Read-only review of the repository's current EKS skill/access workflow found a supported manual GitHub/AWS route, but this runtime has no corresponding authorized dispatch or EKS tool. No attempt was resubmitted under a new label. The full-root parser defect and these PCC failures belong to separate pre-existing source maintenance. [pcc:unverified]
