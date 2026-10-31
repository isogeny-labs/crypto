# Hosted native Metal CI observation

Source: [crypto job 118955884773](https://github.com/aburan28/crypto/actions/runs/39518361538/job/118955884773),
retrieved through the GitHub connector on 2026-10-21. The gzip file contains the
complete decoded job log, including the cancellation and cleanup. Compression
uses `gzip -n -9`; the decoded text was checked byte for byte against the
connector response before compression.

- PR head: `917e2214b132e76c509575095269652c66ff907c`.
- Tested GitHub merge revision: `8376e7898de541a82647d3978bda1a9ea0107309`.
- Runner: `macos-14`, Darwin 23.6.0 arm64, Xcode 15.4, Rust 1.99.1.
- Native worker identity: `CERTICOM_WORKER 1 metal Apple Paravirtual device`.
- Result: native worker/shader compilation and all six device library tests
  passed. The job was cancelled at its 25-minute limit while building the
  release CLI. No CLI pass or complete hardware qualification is established.
- Scope: arithmetic replay, rho/BSGS/IC fixtures on both exact challenge curves,
  bounded public targets and worker/deadline controls. This is a native Metal
  API observation; it does not identify the physical GPU behind the exposed
  paravirtual device. It is not the host-emulation backend.

SHA-256:

```text
6f137209823bd043abd5c67373eb9c1cee9a23f7d269a633538a50b50981d9af  job-118955884773.log.gz
a60986f2e3748f1bc390e16b0de19ee43484ee297f91e8c143ad656ff0cda9e3  decoded job log
```

To inspect or verify the retained log:

```sh
gzip -cd job-118955884773.log.gz | less -R
gzip -cd job-118955884773.log.gz | sha256sum
```

The [qualification record](../../certicom-130-gpu-hardware-qualification.md)
keeps physical provenance, complete Metal CLI qualification, and NVIDIA
execution open. Constructed fixtures do not establish public-challenge
solutions or throughput.
