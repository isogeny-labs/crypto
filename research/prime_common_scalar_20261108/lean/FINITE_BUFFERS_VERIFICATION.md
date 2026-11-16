# Producer verification for finite coefficient buffers

Pinned Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Source07cfb5586 contains
FiniteBuffers, ResidueEncoding and both default targets.

Clean default build: buffer-default-build-001.log. Actual expanded repository
source audit:96 declarations,0 failures,0 extra trusted; only propext,
Classical.choice and Quot.sound. Receipts in finite-buffers-gate/.
Official import-inclusive ResidueEncoding fresh replay completed exit0;
log residue-fresh-001.log and status finite-buffers-fresh-status.json.
These remain unsigned producer checks, not trusted execution attestation.
[pcc:unverified]

From repository root with pinned toolchain bin directory on PATH:

```sh
lake -d research/prime_common_scalar_20261108/lean build
lake -d research/prime_common_scalar_20261108/lean env leanchecker --fresh --verbose ResidueEncoding
bash tools/lean/proof-gate.sh --out /tmp/finite-buffer-gate research/prime_common_scalar_20261108/lean
```

No checker, proof gate, Lean trust manifest, kernel or CI policy changed.
No inferred sorryAx from failed elaborations was accepted. All failed builds
are retained (including explicit casts/constant-unit and finite-index type
annotations), while final accepted sources contain no proof hole.

The PCC CI failure was metadata-only: unsigned records lacked decl_sha256.
Original versions are archived verbatim, canonical pins were generated with
the maintained metadata routine, and corrected claim registrations were
appended. Native JSON Schema checks enforce required pin fields. There is no
producer-signed verdict or claim of independent verification.
