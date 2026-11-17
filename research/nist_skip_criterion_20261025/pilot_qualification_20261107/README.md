# Native pilot preparation

The existing native sampler, separate journal reader and recovery controls now
build as static x86-64 development binaries. The corrected build passed 46
native controls and seven CLI controls (one empty-prefix inspection and six
refusals). All six toy recovery states reproduced the uninterrupted journal
bytes. These are local unsigned development observations, not an EKS execution
or signed proof. No real-field curve was sampled. [pcc:unverified]

This preparation follows the user's approval. The missing total dollar cap was
asked separately; it remains unset in this record. No cloud build, publication,
job or checkpoint upload has been performed by this preparation.

## Deliverables and state

| Requirement | State |
| --- | --- |
| Native artifact for the existing command bridge | Local static layer staged, dependencies and files hashed |
| Six-field pilot, ten accepted curves per field | Draft protocol; not launched |
| Published image and EKS qualification | Pending actual image/source and execution receipts |
| Durable checkpoint/recovery and canonical export adapters | Pending implementation and qualification |
| Shared daily/monthly budget and live capacity admission | Pending current readback; no new paid work admitted |
| 60,000 accepted primary curves and ten published percentiles | Unrun; retained by the original protocol and backlog card |
| Every cutoff 100–5000 | Existing study retained; no new sweep here |

`protocol.json` is an `experiment.protocol/v1` **draft**, checked with the
canonical tool at crypto-autoresearcher commit
`5f9d6fb501d408f3e8ad6ee89afbbbdadb8b560e`. Structural validation returned zero
findings. Its image, binary admission, total dollar cap, absolute deadline,
runtime preflight, checkpoint and export bindings are deliberately incomplete.
The planned application checkpoint fields describe an obligation, not an
implemented durable adapter. A draft validation does not permit launch.

## Reproduce the development build

From the repository root, with the recorded installed PARI 2.17.3 prefix:

```sh
bash research/nist_skip_criterion_20261025/pilot_qualification_20261107/prepare-layer.sh /ABSOLUTE/FRESH/OUTPUT
/ABSOLUTE/FRESH/OUTPUT/build/journal_tests research/nist_skip_criterion_20261025/strict_elkies_20261029/inputs.tsv /ABSOLUTE/FRESH/OUTPUT/toy-fixtures
bash research/nist_skip_criterion_20261025/journal_qualification_20261104/test-cli.sh /ABSOLUTE/FRESH/OUTPUT/build
```

The staging script refuses an existing output directory. It only compiles and
copies a layer locally; it never publishes or starts a sampler job. It uses
`pkg-config --static --libs libcrypto gmpxx gmp`, retaining OpenSSL's zlib/zstd
dependencies. `pari-datadir.cpp` reads build metadata without initializing a
curve. The layer includes seadata at PARI's compiled prefix, native source,
the pinned cohort and the three binaries. Each binary has no ELF interpreter
or dynamic-library dependency in the retained `readelf` records. [pcc:unverified]

Static linking still emits glibc warnings about optional dlopen/name-service
paths. Local controls passed, but runtime compatibility and large-field SEA
data sufficiency still require actual image qualification. No speed or
portability-across-all-hosts claim is made. The first failed link and corrected
build stderr are both retained in `review-evidence/`.

The first build failed because static OpenSSL needed zlib and zstd. The corrected
build and controls used unchanged native source. `DEVELOPMENT.json` and
`MANIFEST.sha256` bind this preparation; the November 4 BUILD.json and manifests
remain immutable historical receipts and are not repurposed for these binaries.
The staged binary payload is local, not a published or durable artifact.

## Execution route and admission

`taskq/deploy/eks-karpenter/remote/launch-remote-command` accepts the existing
`isolab.eks.command-request/v1`, whose command can run the native binaries in a
digest-pinned `isolab/tasks` ECR image. It uploads the matching launcher script
and buildspec by hash, so this route does not require a control-plane redeploy.
The generic existing `oci-layer-publish.py --layer-dir ...` can publish such a
layer after admission; its ecbench runtime base must not be mistaken for a
native sampler artifact. No ecbench fixture result qualifies this pilot.

Local read-only STS returned `Unable to locate credentials` through the normal
SDK chain with instance metadata disabled. The documented main-only GitHub
OIDC route remains a possible authenticated path. An Actions-variables read
returned HTTP 403; that does not establish that the documented variables are
absent. A reviewed trusted main workflow may reference them normally; never
extract credentials/variables from logs or grant new access as a workaround.

Before paid work:

1. Obtain the user's total finite dollar cap and authoritative current daily
   and monthly charged, committed and reserved balances. Account for builds,
   status CodeBuilds, idle/queue capacity, failures, storage and transfer.
   Unknown balance blocks spending. The shared USD 200/day and USD 1,000/month
   account policies are not this pilot's per-attempt cap.
2. Read actual NodePools, quota and existing jobs/nodes. `submit-job` requires
   host anti-affinity: a 2-CPU pod occupies a whole worker. Six one-hour
   `c8a.2xlarge` workers can expose 48 default-vCPU hours, not twelve. The
   desired ordinary 16-vCPU ceiling permits at most two such workers, subject
   to other commitments and live readback. Do not raise limits.
3. Publish and review exact source, build, inputs, image and launch command;
   retain `check-experiment-git` and source/image receipts. The preparation
   script and static binaries alone do not satisfy this step.
4. Prepare the checkpoint payload from this isolated study, excluding the
   denied older auxiliary archive. Inspect `checkpoint.sh status`, complete
   the study-specific resume note, upload through the existing authorized
   checkpoint path, and require `checkpoint.sh gate` to pass. The existing
   root `.resume/RESUME.md` belongs to another workstream; do not overwrite it.
5. Implement and qualify durable journal transfer/recovery and canonical
   export/failure flow. Fsync inside temporary emptyDir is not durable Spot
   recovery. Keep original journal prefixes, nonce, index and deadlines;
   retain failures and unstarted fields without invented attempts.
6. Qualify the actual image on known fixtures, record preflight operational
   costs and complete the canonical freeze before drawing the fresh nonce.
   Only then admit the separate six-field pilot, with one attempt per field,
   original per-field deadlines including provisioning/replay and no retries.

The launcher client's bounded build-status loop is not a reason to poll in
the foreground. Submit through a reviewed asynchronous workflow, retain the
original request/build/Job/Pod identities and inspect once per follow-up
cadence. Reconcile ambiguous submission by those IDs before doing anything else.

## Sources checked

KB unavailable: no KB MCP is loaded and `scripts/research-kb status` returned
127, no executable. Conductor also returned 127; no reservation is claimed.
Fallback review used the original October 29 protocol/sampler, November 4
native journal README/source/Makefile, current isolab-eks and report-evidence
skills, command bridge, submit-job, generic OCI publisher, ACCESS.md and
COST_CONTROLS.md, plus the canonical protocol rules, schema, checker and worked
example at the commit above. The
[PARI ellsea documentation](https://pari.math.u-bordeaux.fr/dochtml/html/Elliptic_curves.html#ellsea)
informs the distinction between early-abort disposition and full-order replay.
No new mathematical mechanism or novelty claim is made.

The original exact tail remains 5869345/20037600 = 0.292916567; it supplies no
historical selection cause or weakness proof. This preparation does not alter
that result or the original acceptance/statistic law.
