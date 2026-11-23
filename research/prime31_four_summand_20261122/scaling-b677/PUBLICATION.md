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
