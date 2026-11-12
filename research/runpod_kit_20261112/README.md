# RunPod kit (2026-11-12)

A minimal way to run research binaries on RunPod CPU pods from a Claude Code cloud
session. It needs no S3, no registry and no repository access on the pod.

## How it works

- **API key.** The RunPod key is a network secret on this session's environment. The
  proxy injects it into requests to `rest.runpod.io`; scripts send the placeholder
  `Authorization: Bearer proxy-injected`.
- **The pod.** `rp.sh create` starts a `runpod/base:1.0.2-ubuntu2404` CPU pod (glibc
  2.39, the same as this container). The pod runs `podserver.py`, which is passed in
  base64 through `env`.
- **Transfer.** Everything goes over the pod's RunPod HTTPS proxy URL
  (`https://<id>-8000.proxy.runpod.net/<token>/…`), behind a random 128-bit token.
  - `put` uploads in 2 MB chunks, because the proxy rejects bodies of a few MB, and
    checks the SHA-256 of the reassembled file.
  - `exec` runs a detached command and writes `exec_N.log` and `exec_N.rc`.
  - `get` and `ls` read results back.
- **State.** Pod ids and tokens are kept in `state/`, which is git-ignored.
- **Cleanup.** Always end with `rp.sh delete NAME`. Name pods `bdr-*`, so leftovers
  can be found with `curl -sS https://rest.runpod.io/v1/pods -H "Authorization: Bearer proxy-injected"`.

## Smoke test, 2026-11-12

| step | result |
|:--|:--|
| pod ready | 12 s |
| 7 MB `jv_cover` upload | SHA-256 matched |
| `--exp dlp --sizes 101` | solved and correct in 29 s |
| pod | deleted |
