# Combined signed/Kummer v3 final binding

This directory holds additive coordinator bindings for one prospective
combined-v3 invocation. The source packet, locks, prior terminal evidence,
campaign authorization, and cache snapshot remain immutable at their original
paths.

The normalized receipts map existing evidence into the exact field shapes
required by `signed-kummer-combined-runner-v3-20261111/durable_io.py`. They do
not add authorization. `admission-candidate.json` and `current-ledger.json`
reserve one 2,400-second attempt without executing it. The candidate is not
runtime-visible until an identical tracked `runner-v3/admission.json` is
published and read back. The fresh cache receipt blobs are artifact-managed;
the 2,618,767,578-byte cache body remains local and is copied into a private
workspace only by the admitted materializer.
