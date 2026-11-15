# RunPod execution package

User-selected provider: RunPod. Allocation is pending authenticated access.
GET `https://rest.runpod.io/v1/pods` returned HTTP401 at 06:19Z on 2026-11-15
using the documented `proxy-injected` placeholder. No pod was allocated.
The session has no RUNPOD_API_KEY and no browser connector. Configure access in
the environment settings; never commit a key or include it in artifacts.

## Ready job and resource envelope

The first job is **correctness replay**, not a wall-time comparison. It runs
the frozen public toy panel and actual-field transition trace twice, refuses
an existing output directory, checks the binary digest and retains failures.
Each child has a 120-second cap. The later complete-cost comparison remains
a separate preregistered experiment, requiring source/A/A, destination generic
and folded controls, accounting and proven measurement admission.

Use one CPU pod, at most 4 vCPUs and 30GB disk; no GPU. Maximum allocation:
30 minutes and USD1 including storage. Obtain an authenticated provider quote
and check its rate before creating the pod. Refuse a quote above the cap.
Use `runpod/base:1.0.2-ubuntu2404` as the compatibility image and retain its
resolved digest when available; record its absence otherwise. Do not claim L2
from the provider name, an affinity setting or exclusive container use.

Build before allocation:

```sh
study=research/p256_genus_dlp_20261115/folded_walk
cargo build --release --locked --manifest-path "$study/Cargo.toml"
sha256sum "$study/target/release/p256-folded-walk"
```

`bash "$study/runpod/package.sh" FRESH_OUTPUT_DIRECTORY` additionally creates
the upload binary, frozen expected outputs, source archive, toolchain identity
and SHA256 manifest. It records both HEAD and uncommitted state: HEAD alone
does not identify a development package. The package command never contacts
RunPod or starts a pod.

Use the existing `research/runpod_kit_20261112` upload/exec/fetch route, with
an environment key if available (its existing placeholder is runtime-specific).
Upload the exact binary and `pod-run.sh`; compare their SHA256 values after
upload. On the pod, supply that recorded digest and a new output directory:

```sh
bash pod-run.sh ./p256-folded-walk RECORDED_SHA256 /workspace/p256-folded-replay-UNIQUE_ID
```

Retain provider allocation/rate and deletion receipts, host/cgroup metadata,
job outputs and exit codes. Compare toy.json and trace.json byte-for-byte
against this directory's frozen results. Fetch before deletion. Install an
allocation-specific deadline guard before starting; delete this identified
pod on success, failure or cap expiry. Never delete somebody else's pod.

No benchmark is pending in a running process. A missing credential is an access
dependency, not a measured negative for the transfer construction.
