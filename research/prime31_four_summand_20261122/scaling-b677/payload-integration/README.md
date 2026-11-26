# RR5 B677 payload integration and development qualification

2026-11-23 Pacific · [PR #5099](https://github.com/isogeny-labs/crypto/pull/5099)

## Delivered component

The fixed RR5 B677 extractor and the separately authored native receipt checker
now have a build, image-publication, command-request, evidence-collection and
replay route in the maintained IsoLab workflow. The implementation is published
at `8aa70e211f9a42e9823e859b1c7197c8677dceca`. Its exact-source local bundle
qualification is retained below; the registered scientific extraction has not
run. All execution observations in this report are unsigned. [pcc:unverified]

The route preserves the frozen [protocol](../PROTOCOL.md): the structured
positive is first, the cofactor positive is compulsory after an ordinary
first-call failure, and H0 then W0 run only after the structured positive
completes and every submitted row passes replay. A failed cofactor call does
not suppress those followups when the structured positive qualified. The
scientific sequence retains one worker, the 180-second outer limit, a 2-GiB
address-space ceiling and the original per-call algebraic limits.

The [payload documentation](../../../../taskq/deploy/eks-karpenter/rr5-b677/README.md)
describes the request and resource contract. The implementation retains failed
and incomplete outputs, checks source/protocol/binary/image/request/Job/Pod
identity, and separates transport checks from row replay. A separate GitHub
job replays the rows without AWS credentials. Its checker authenticates the
submitted public group relations and associated rank/scalar data; extraction
completeness still comes from the source-bound producer traversal.

## Local qualification

The frozen [QUALIFICATION.json](evidence/QUALIFICATION.json) and its raw files
are pinned by `PCC-20261124-21920c`. The command used Rust 1.100.0 and the
existing lockfile to build static `x86_64-unknown-linux-musl` executables with
portable x86-64 flags. It checked the 1,916 inventoried source inputs before
and after the build and test sequence. [pcc:unverified]

| Check | Recorded outcome | Scope |
| --- | --- | --- |
| Native library tests | 32 passed | Small public development controls |
| Independent native receipt-checker tests | 19 passed | Checker behavior, separate from producer completeness |
| Public example tests | 8 passed | Existing example controls |
| Runner contract tests | 57 passed | Mock dispatch, conditional sequence and failures |
| Artifact contract tests | 67 passed | Mock custody, identity and replay failures |
| Static bundle build | Exit 0 | Two executables and fixed runner scripts |
| Workflow lint | Exit 0 | actionlint 1.7.12; ShellCheck was unavailable |
| Existing remote, launcher and checkpoint-source contracts | Exit 0 | Maintained shell contracts |

These are producer-reported development observations, with no signed PCC
verdict. The mocks do not run the registered extractor. The local qualification
does not exercise Docker, ECR publication or the OCI scientific runtime.
[pcc:unverified]

| Built executable | SHA-256 |
| --- | --- |
| `rr5-b677-gate` | `6fc7d212416e4a6c9b775b69ae2c5b039c32838197df6dd1e9a0d034e495ec97` |
| `rr5-b677-receipt-check` | `95d3e9763b463c96410af56d7927bdd39f071377916d3473ad048312d694ef04` |

The checksum inventory, build metadata and source-selection manifest are
retained in `evidence/`. Compiled binaries are not committed. The qualification
receipt keeps its original source revision even though later publication also
integrates newer main bookkeeping.

## Source review and retained failed attempts

[SOURCE_REVIEW.md](SOURCE_REVIEW.md) documents the deliberate refresh of two
whole-file source expectations. Seventeen of eighteen selected ranges are
byte-identical to the prior producer; the remaining range adds only a comment
and two `cfg(test)` accessors. The complete selected diff and every range hash
are retained. No existing selected production function was changed in that
diff, and the builder's exact whole-file and selection guards remain active.
[pcc:unverified]

The initial source-guard failure and missing sparse dependency failure remain
in the evidence directory. They were followed by the source review and exact
tracked-file restoration, rather than changes to the frozen protocol.

The required full-root command was also attempted:

```sh
cargo +1.100.0 test --locked --release --lib -j2
```

| Attempt | Recorded outcome | Disposition |
| --- | --- | --- |
| Root attempt 01 | Exit 101; missing tracked generalized-rootfinder source | Restored the exact tracked source |
| Root attempt 02 | Exit 101; missing embedded sparse inputs and source diagnostics | Retained logs; restored 64 exact tracked inputs |
| Root attempt 03 | Exit 101; 11 compiler errors, no missing-input errors | Retained as the final root-gate failure |

The final errors are the duplicate `round73_mersenne_single_fold` module in
`src/cryptanalysis/prime_radical_root.rs` and missing `powers`, `gcds`,
`previous`, `r` and `g` bindings in `src/cryptanalysis/prime_root_trace.rs`.
Root `src`, `Cargo.toml` and `Cargo.lock` match both the qualified implementation
and incoming main `e1552f1e0942690228900cc86eb6b0c8b4de88c1`. Those unrelated
files were not patched by this task. [pcc:unverified]

The earlier [ROOT_GATE.json](evidence/ROOT_GATE.json), pinned by
`PCC-20261124-daf86c`, is unchanged. The additional
[ROOT_GATE_FINAL.json](evidence/ROOT_GATE_FINAL.json) and final stderr are
pinned separately by `PCC-20261124-eb40d0`. This preserves what was known at
each attempt. The root gate remains failed; the focused native qualification
has the separate scope described above. [pcc:unverified]

## Deployment admission remains blocked

The maintained `scripts/checkpoint.sh gate` was exercised with a completed
resume note. It exited 1 because this runtime has no configured
`ecc-challenges-new` AWS profile, `CHECKPOINT_UPLOAD_TOKEN`, or upload-token
file. No checkpoint was uploaded. The exact command, resume note, diagnostic
and file hashes are retained in [CHECKPOINT_GATE.json](evidence/CHECKPOINT_GATE.json)
and pinned by `PCC-20261124-eb40d0`. [pcc:unverified]

The checked-in GitHub trigger role also lacks ListBucket and the raw-object
GetObject permissions required for complete results collection. The workflow
checks the necessary listing permission before publication/submission and
requires checkpoint success. A successful listing probe does not establish
raw-object read rights. Full readback needs an existing authorized transport;
the implementation neither changes IAM nor puts raw records under a sanitized
receipt filename.

Remaining daily/monthly/lane budget and live all-tenant capacity have not been
established. These are prerequisites the operator must establish before
dispatch, rather than facts certified by the workflow's list probe. No cloud
build, image publication or registered scientific Job was started here.
[pcc:unverified]

The next execution step is the fixed workflow mode on main, with a full
published source revision, only after checkpoint transport, full results
readback, a finite known remaining budget and live capacity are established.
Inspect the original request after an ambiguous submission; a new workflow
attempt is not recovery of the original one. Do not retry the unchanged local
transport failure or alter the frozen targets and limits to obtain admission.

## Claim and ledger custody

The two initially unpublished claim payloads were transferred byte-for-byte
onto the newer main ledger. Their original unpublished registration suffix is
archived in [registration-refresh-v1](evidence/registration-refresh-v1/), and
the maintained PCC append routine registered their unchanged bodies after
main's exact ledger prefix. The registration map links old and new envelopes;
the original claim IDs, proof hashes and producer commit remain intact.
[pcc:unverified]

The refresh receipt records the 1,840-entry checkpoint before the separately
authored final root/checkpoint claim. It is a historical transfer receipt,
not a mutable summary of the eventual ledger length. All three claims remain
`unverified`; no signature or independent execution attestation is supplied.
[pcc:unverified]

## Reproduction and source references

For a clean checkout at the qualified commit, with its full inventoried source
closure and Rust/musl toolchain available:

```sh
taskq/deploy/eks-karpenter/rr5-b677/build-bundle \
  "$PWD" 8aa70e211f9a42e9823e859b1c7197c8677dceca /tmp/rr5-b677-bundle
bash taskq/deploy/eks-karpenter/rr5-b677/test-contract.sh
bash taskq/deploy/eks-karpenter/rr5-b677/test-artifacts.sh
```

The bundle output path must not already exist. These commands reproduce local
development qualification; the cloud admission prerequisites still apply to
any subsequent publication or extraction.

Sources checked: the frozen `scaling-b677/PROTOCOL.md` and `ADMISSION.md`,
the published producer and existing native receipt checker, the maintained
IsoLab workflow, `remote/bridge.yaml`, checkpoint tooling, `HARNESS.md`, and
`COST_CONTROLS.md`. The source-pin review records exact historical Git sources.
KB and Conductor were unavailable in this runtime; the repository protocol,
implementation and prior receipts supplied the review context. This work
adds execution integration and makes no novelty claim.
