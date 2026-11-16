# AWS handoff: primary N83 fresh-trial pattern panel

This directory is the executable handoff for the exact primary `a=0`
Koblitz GF(2^83) curve in [`protocol.json`](protocol.json). The feature
branch is `codex/n83-aws-handoff-20261025`, PR
[#2557](https://github.com/aburan28/crypto/pull/2557), stacked on PR #2551.
The frozen hosted workflow is
[`n83-natural-yield-pattern-panel.yml`](../../../../.github/workflows/n83-natural-yield-pattern-panel.yml)
at commit `fd9121107c40b159b13497c475f848ea9a21aa96`; it builds the
mathematical worker from `83393dd87ef48f11963c582cd79b89296c4a1bea`.
Use those source identities when replaying run
[`40185426122`, attempt 1](https://github.com/aburan28/crypto/actions/runs/40185426122).

The eleven immutable inputs are listed, with byte sizes and SHA-256 digests,
in [`../linux-xor-complete-inputs-20261025/manifest.json`](../linux-xor-complete-inputs-20261025/manifest.json).
Their bucket/prefix is
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/linux-xor-panel-20261025/inputs/`.
The workflow fresh-fetches all eleven objects on each Linux ARM stratum host.
The signed-orbit archives, XCNFs, and export metadata must match the manifest
before a search is admitted. The pinned solver is the qualified Linux ARM
CryptoMiniSat binary from preflight artifact `12593635829`, ZIP SHA-256
`00713618c7c09188a9f75fef6ce6323af376428cc48d537491394c91429d1237`.

The frozen panel has strata `t1_m0`, `t1_m1`, `t2_m0`, and `t2_m1`.
Each runs six ordered, bounded arms after an exact four-formula export.
The worker-build artifact is `12597612377`, ZIP SHA-256
`b530157c3040e7586ef5d1a8a9ec40a7c5bfd5586650c0c6733ece8c5e3042da`;
its Linux ARM worker SHA-256 is
`d7e97aaac59ab37a454d73ee0782871b6f7f266222bfe167ff9b28ec2b461aeb`.
Keep this build ZIP byte-for-byte. GitHub Actions artifacts have finite
retention; the provider ZIP, its API metadata, and the run's jobs JSON are
required inputs to the independent auditor.

All four strata completed. Their exact artifact IDs and ZIP hashes, 24-arm
bounded verdict, process charges, and scientific limits are in
[`RESULT.md`](RESULT.md). The four saved provider-audit JSONs and a compact
retained receipt set are in [`run-40185426122-attempt-1/`](run-40185426122-attempt-1/).
Run [`verify_retained.py`](verify_retained.py) to check every committed
receipt; the saved replay is
[`retained-replay.json`](run-40185426122-attempt-1/retained-replay.json).

For each completed stratum, download its artifact without repacking it,
record its SHA-256, and save the API metadata for both artifact IDs plus
`repos/aburan28/crypto/actions/runs/40185426122/jobs`. Then run
[`audit_artifact.py`](audit_artifact.py) with `--stratum`,
`--artifact-zip`, `--expected-zip-sha256`, `--metadata`,
`--build-artifact-zip`, `--expected-build-zip-sha256`,
`--build-metadata`, and `--jobs`, as shown in [`AUDIT.md`](AUDIT.md).
The auditor binds both ZIPs and provider jobs to the frozen run/head, checks
the eleven fresh S3 replays, exact formula bytes, raw solver streams, process
accounting, and any first-stage group-verifier receipt. Record its JSON and
the input ZIP hashes in the PR. If a candidate passes the first group check,
retain `PENDING_INDEPENDENT_SAT_GROUP_REPLAY` until a second independent
source build, fresh archive fetch, exact model replay, and curve-group check.

The repeated local [192-formula identity-mask census](../full-identity-mask-census-20261025/RESULT.md)
is a separate necessary-condition screen, not solver yield. It leaves masks
0, 1, and 2 without a contradiction from its three checks. The earlier
[single-pattern hosted panel](../linux-xor-natural-panel-20261025/RESULT.md)
completed six capped arms without a natural relation. Neither panel selects
a factor base by complete one-target cold index-calculus runtime. That
selection still requires independently replayed natural relations, graph
closure and rank, verified factor-base logs and unseen-target descent,
complete build/import/search/linear-algebra accounting, and matched
same-target rho on this exact curve. Keep `selected_best_total_runtime=null`
until those gates are met.
