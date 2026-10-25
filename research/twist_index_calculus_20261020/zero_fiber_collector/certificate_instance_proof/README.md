# Literal certificate instance verification

The [protocol](PROTOCOL.md) verifies the first registered raw-degree-12
square certificate from the separate degree/closure diagnostic. The
candidate and workload hashes are fixed before export. No F4, relation
collector, point census or DLP is repeated.

The [native exporter](native/src/main.rs) uses fresh dense integer coefficient
arrays, rather than the producer's sparse verifier, and constructs the exact
integer lift `sum(h_i*f_i) = q^2 + 37*R`. It emits literal Lean polynomials.
The intended checked conclusions are that integer identity, its
characteristic-37 consequence and the repeated-abscissa conclusion under
the explicit no-zero-divisor hypothesis. The original generic proof is
copied verbatim as a prefix. No certificate identity is assumed by the new
instance theorem.

Native gate `../runs/certificate-instance-native-gate-v1/` passes **13 tests**
(four instance controls and nine unchanged standard IDC tests), with zero
failures. The format check, release build and source seals pass. Verifier
SHA-256: `2702267a3d45a00d26d276999c734e193021ab41566c2e3eebb8e3e12e9e03e8`.
The actual export, Lean compilation and IDC have not yet run at this source
freeze; those outcomes require their own receipts.

Raw execution is reserved as `../runs/certificate-instance-proof-v1/`.
Inspect the managed unit `twist-certificate-instance-proof-v1.service` and
its receipts before any launch or resubmission. Use one worker, the shared
busy reservation, 8 GiB and 1,200 seconds per proof/export/IDC phase.
An observation timeout is not a restart authorization.

This closes neither a curve/decoder/group bridge nor a parametric-family,
full-panel, complete-cost or asymptotic claim. All three existing panel
owners retain their workloads and deadlines. The raw16 and closure8 first
system results are separately audited/repeated by that registered owner;
they are not inferred from this instance proof. A final report, diagram and
inspected PDF remain required after the proof attempt is terminal.
