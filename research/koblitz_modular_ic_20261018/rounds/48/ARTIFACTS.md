# R48raw archive and rejected-launch custody

SHA256 `7b5e7a63ea22a63cd76484b1a3885deb8a8b28a5525e892b22a77c94a733f8cb`;
100308744bytes,214regular files,45historical source pins. Scientific source
revision `11a411790222c5f168f87fdb94637d829b5e3321`.
Stored in setec-astronomy-artifacts/store. Six accepted native runs retain
all inputs/classes and1319full-generator quotient certificates. Oneaccepted
Lean proof and oneactual precompilerlaunch failure are separately indexed.
The rejected launch has no fabricated compile/auditreceipt: its actualgate,
oldrunner bytes, exactstderr metadata and historicalGitrevision are retained.

artifact-verification.json records freshGetObject, every memberhash and
byte-identical deterministic repack. publication-validation.json records
historical source blobs, acceptedattempt source hashes, rejectedrunner byte
identity and terminalconsistency. These are byte/receipt checks, not an
independent scientific replay or semantic review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/48
cd research/koblitz_modular_ic_20261018/rounds/48
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read FORMAT.md before interpreting class/equality witnesses. Rebuild with
fresh attempt names and pinned Lean4.34.1/mathlib dependencies; never replay
an accepted producer merely to resume publication.
