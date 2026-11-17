# Prime m2 first-fall and strict filtered-witness dataset

`runs.tar.xz` preserves all six immutable inputs, the native executable,
frozen protocol, controls, complete sparse factors, reconstruction circuits,
certificates, producer/fresh replay logs and terminal records. Read
[the byte layout and file map](../PRIME_FIRST_FALL_FORMAT.json) and
[the algebraic argument](../PRIME_FIRST_FALL_DESIGN.md).

Each witness is exactly `A0 tensor B0 + A1 tensor B1 + A2 tensor B2`.
No expanded coefficient is omitted: the factor files and circuit reconstruct
all coefficients using the recorded normalized-variable weights. Highest-form
FFD and strict filtered falls have separate certificates. A nonzero functional
supplies a global LFD lower bound; no global upper or exact LFD is certified.

Hydrate with `python3 tools/artifacts/artifacts.py pull
research/n83_candidate_pdp_20261028/prime-first-fall-archives`, check
`runs.tar.xz.sha256`, and extract into a fresh directory `restored`.
Then run:

```sh
(cd restored && sha256sum -c manifest.sha256)
restored/instrument replay restored restored/results
(cd restored && sha256sum -c manifest.sha256)
```

Replay needs 2 GiB virtual memory and a 300-second limit. It writes no dataset
file. The archive is deterministic GNU tar with sorted names, mtime
2026-10-30 00:00:00 UTC, uid/gid0, modes u+rwX,go+rX,go-w, and xz -0 -T1.
Development failures, protocol drafts, helper scripts and publication records
are retained separately in the support archive; later receipts use a final
publication bundle after support is sealed. Older datasets remain unchanged.
