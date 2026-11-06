# Exact D18 input read for the deployed ephemeral runner

**Status: proposal; neither policy below has been applied.** The source-bound
native D18 pair remains under the frozen [protocol](../PROTOCOL.md) and
[retained-assignment amendment](../ASSIGNMENT_CERTIFICATE_AMENDMENT.md).
These policies only let the deployed runner fetch the two original S3 inputs;
they do not authorize a broader object prefix, a different curve, or an
additional solver arm.

The deployed `ephemeral` configuration is in AWS account `104186811093`,
`us-east-1`, with instance profile `ephemeral-runner`. Its role ARN is
`arn:aws:iam::104186811093:role/ephemeral-runner`. The source bucket
`crypto-autoresearcher` is in `us-west-2`. On 2026-10-26, the
`ecc-challenges-new` profile reached the
[runner configuration](evidence/ephemeral-config.json), while a `HeadObject`
of the original D18 manifest returned `403` ([exit](evidence/runner-manifest-head.exit.txt),
[stderr](evidence/runner-manifest-head.stderr.txt)). The
[runner role](evidence/runner-role.json)'s [only inline policy](evidence/runner-inline-policy-names.json),
[`runs-bucket`](evidence/runner-existing-policy.json), covers its own
`jobs/*` and `cache/*` objects; its [attached policy](evidence/runner-attached-policy-names.json)
is `AmazonSSMManagedInstanceCore`. The source bucket has
[no bucket policy](evidence/source-bucket-policy.stderr.txt) at this check.
These read-only observations identify
two necessary policy sides; they do not prove a submitted job will run.

The runner's deployed [launch template](evidence/launch-template-market.json)
has no market option. This PR adds opt-in `--on-demand` to both runner CLIs,
keeping Spot as the default. The [Docker-free fake-AWS launch test](evidence/market-selection-test.log)
checked the Spot/default and On-Demand request shapes and rejected
conflicting flags. The [standalone Rust CLI test](evidence/ephemeral-cli-test.log)
passed all eight unit tests under Rust 1.99.1; its toolchain emitted a
nonfatal `rust-objcopy` missing-library warning. `bash -n`, focused
`shellcheck`, and `cargo fmt --check` passed. The full Docker-backed local
end-to-end suite was unavailable because this host denies its Docker API
socket. A live `--no-source` On-Demand smoke was attempted, but
[`SpotOnlyEC2Launch` v2](evidence/spot-only-policy-v2.json) explicitly denied
`RunInstances`; the [job status](evidence/on-demand-smoke-status.json) is
`error` and [no instance](evidence/on-demand-smoke-instances.json) was
allocated. The exact S3 policies proposed below would not change that IAM
launch guard. An authorized non-Spot launch path is a separate prerequisite.
Such a lane avoids requiring a checkpoint from the current bounded native
solver; an interrupted or
otherwise invalid wall observation remains a censored attempt under the
frozen protocol. The required root `cargo test --release --lib` attempt on
the merged PR head stopped at the inherited unclosed delimiter in
`src/cryptanalysis/ecbench/methods.rs` ([compiler log](evidence/postmerge-release-lib-test.log));
the merged head has no passing full-library test receipt.

After explicit authorization and immediately before any mutation, recheck
that the source bucket still has no policy and that the role's current
policies have not changed. Apply [the exact source-bucket policy](bucket-exact-read-policy-proposal.json)
and [the exact runner-role inline policy](runner-exact-read-policy-proposal.json)
as `N83D18ExactInputReadEphemeral20261026`. Read back both and compare their
canonical JSON with the reviewed files. If either step fails, roll back any
applied half. Use the policies only for a frozen direct GET of the original
manifest and object, with provider checksums and the full local SHA-256 checks.
The proposal file SHA-256 values, bucket side then role side, are:

```text
a9605e9da5d793f62e86a73dd38281404c7d4bfe1fcb6b43b575a7ea75deeb13
f4a7cd756e55ce400498ef66a640c0f7b23f234f0869d2427701f02970fd269b
```

Remove both policies when the hosted attempt ends or is cancelled, first
checking that their live documents still match these exact proposals; if a
document has changed, reconcile it before deletion. Verify
`NoSuchBucketPolicy` and `NoSuchEntity` respectively. The previously approved
`rho-gh-snapshot-readonly-20261024` user grant is separate; its cleanup must
also be verified after its hosted use ends or is cancelled.

The local Mac's third fresh GET matched producer manifest SHA-256
`1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce`,
custody wrapper SHA-256
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`,
and 160,747,529-byte object SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`.
At the [recorded host preflight](evidence/host-preflight.txt), one-minute
load was 114.84 against the frozen maximum of 24; the local worker could
not launch. This is input and capacity evidence only. No D18 solver job has
been submitted from this proposal.
