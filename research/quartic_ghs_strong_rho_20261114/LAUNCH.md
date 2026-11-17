# Round-A launch package

This directory deliberately contains no launch-ready request yet. Requests are
generated only after an operator supplies current, complete preflight and image
receipts. A placeholder with `ok=true` would be dangerous because it could be
mistaken for live admission.

## 1. Read-only preflight

The operator receipt consumed by `make_requests.py` has schema
`ghs.strong-rho.preflight/v1` and records:

- account `104186811093`, region `us-east-2`, and an observation no more than
  30 minutes old;
- both named USD 200 daily readbacks (two controls over the same shared USD
  200 allowance) and the USD 1,000 monthly budget, each read successfully
  within the same 30-minute window; every row records period, limit, actual
  spend, reserved spend, and remaining balance, and the validator recomputes
  at least USD 5 of effective unreserved headroom as their minimum;
- an all-tenant `cpu-spot` inventory proving the next 8-vCPU/8-GiB wave fits,
  namespace quota, and the complete active-Job inventory, each bound by a
  SHA-256 receipt;
- successful checkpoint and Git-custody gates, also bound by SHA-256.

The validator rejects stale, partial, wrong-account, underfunded, or
namespace-only receipts. Run the same read-only preflight again before wave 2.

## 2. Exact image

After preflight, `build_context.sh SOURCE_REVISION NEW_CONTEXT_DIR` clones the
already-pushed revision into a clean sparse checkout, downloads jq and AWS CLI
from their official HTTPS endpoints, and checks their pinned SHA-256 values
before they enter the context. The build is then network-independent:

```sh
DOCKER_BUILDKIT=1 docker build --platform linux/amd64 \
  --build-arg SOURCE_REVISION="$SOURCE_REVISION" \
  --file "$CONTEXT/repo/research/quartic_ghs_strong_rho_20261114/Dockerfile" \
  --tag "$PRIVATE_TAG" "$CONTEXT"
```

The Dockerfile uses a read-only BuildKit context bind, pins the official Rust
base by digest, rechecks jq 1.7.1 and AWS CLI 2.40.3 by SHA-256, preserves the
exact clean Git revision, refuses changed scientific-source hashes, and
compiles/runs the standalone self-test before sealing the image.
Publish only to the private `isolab/tasks`
repository, resolve the pushed digest, and retain a
`ghs.strong-rho.image/v1` JSON object containing `image_uri`,
`source_revision`, `built_at`, and SHA-256 fields for `main.rs`,
`strong_rho.rs`, `pod_run.sh`, `PROTOCOL.md`, `ROUND_A.md`, and `Dockerfile`.

## 3. Requests and waves

From a clean checkout of that same pushed revision:

```sh
python3 research/quartic_ghs_strong_rho_20261114/make_requests.py generate \
  --preflight OPERATOR_PREFLIGHT.json \
  --image IMAGE.json \
  --source-revision "$SOURCE_REVISION" \
  --output research/quartic_ghs_strong_rho_20261114/launch-ROUND-ID
```

Commit and push the generated manifest and four requests, run
`check-experiment-git` on every launch input, checkpoint again, then submit
low+mid. Submit high+top only after both first-wave Jobs are terminal and a
fresh budget/pool preflight passes. `remote/launch-remote-command` is the
reviewed bridge. Do not retry an ambiguous submission; reconcile its original
CodeBuild and Job identities first.

Each request enters `/opt/crypto` with GNU `env --chdir` before invoking the
absolute pod script. The script derives the exact Git root from its working
directory, so this binding is explicit in every hashed request instead of
being hidden in the image's default working directory.

Command Jobs provide standard isolation and counted-work evidence. Their wall
time is diagnostic and cannot be promoted as an L2 speed claim.
