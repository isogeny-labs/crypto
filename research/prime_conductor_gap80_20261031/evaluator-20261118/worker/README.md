# Standalone native evaluator worker

The [qualification report](QUALIFICATION.md) records the retained loader failure,
corrected non-root fixture replay, image/source/binary manifests and a separately
prepared 47-bit supplied case. The local worker is not a published EKS image.

This packages the existing FLINT/PARI evaluator for a later authenticated
command job. It does not submit a job, configure AWS, or certify a conductor gap.
The runtime uses UID/GID 1000, accepts the existing CLI and reads its supplied
case on stdin. Source, binary and compiler-package manifests are retained in
`/opt/conductor`.

The Debian base is pinned by its public manifest digest. FLINT 3.1.3 and
PARI 2.17.2 match the preceding native checks. Build from the repository root,
using the small native directory as context; no research archives or secrets
enter the build context:

```sh
revision=$(git rev-parse HEAD)
docker build --platform linux/amd64 \
  --build-arg SOURCE_REVISION="$revision" \
  -f research/prime_conductor_gap80_20261031/evaluator-20261118/worker/Dockerfile \
  -t conductor-evaluator:"$revision" \
  research/prime_conductor_gap80_20261031/native
```

Use the managed Docker endpoint documented by the cloud runtime when the
default context points elsewhere. Build only from clean, published inputs.
The resulting local image ID is not a published registry manifest digest.
Before EKS execution, the existing trusted publisher must build/publish this
worker, bind the tested source/binary and OCI digest, and establish the current
budget, reservations, capacity, original deadline and durable output contract.

Development replay, with stdout/stderr captured by the caller:

```sh
docker run --rm --network none --read-only --cap-drop ALL \
  --security-opt no-new-privileges --cpus 2 --memory 1g --pids-limit 64 \
  --tmpfs /tmp:rw,size=64m conductor-evaluator:"$revision" --fixtures
```

Use a bounded client timeout and clean up only that invocation's container
after interruption. These development controls are not performance trials.
The 80-bit quotient/images, matched endpoint measurements, full-window resource
feasibility and production checkpoint/resume remain separate obligations.

## Projective helper build dependency

The Dockerfile now copies and hashes `projective_walk.hpp` alongside the two
existing source files. The header is synchronized with #4703's implementation
checkpoint `bbf30f5b79`; that PR carries the optional bounded-projective evaluator
change and its development controls. Until both changes land together, this
worker branch's older evaluator does not enable the new backend merely by
copying the header.

The retained attempt-A/attempt-B image, source and binary receipts describe
their historical builds. This packaging correction does not rebuild or qualify
an image of the new source. A future image must bind the complete tested source
set, including this header, and retain a new binary/image receipt before a
cloud run. No old receipt or image identifier is reassigned to it.
