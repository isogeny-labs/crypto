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

Native gate `../runs/certificate-instance-native-gate-v2/` passes **13 tests**
(four instance controls and nine unchanged standard IDC tests), with zero
failures. The format check, release build and source seals pass. Verifier
SHA-256: `ea6e2710044facaa8dff429041a11bcc260c0d45a982b357ad7819031c1aac6f`.
The first export passes all 2,197 dense coefficient checks and retains 454
nonzero integer-lift remainder coefficients. Its first Lean attempt fails
with an out-of-memory panic under the unchanged 8 GiB address-space limit.
No theorem or IDC is accepted from that attempt. Both native gate versions,
the original exported module and the failed run are preserved.

Version 2 balances sum syntax and proves each of the four multiplier products
separately before combining them. All ten theorem axiom sets must pass.
The same three final statements, literal coefficients and one aggregate
1,200-second/8 GiB Lean phase remain required. The changed proof encoding
has not yet consumed the real candidate at this second source freeze.

The original execution `../runs/certificate-instance-proof-v1/` and owner
`twist-certificate-instance-proof-v1.service` are terminal, exit 1. The
second execution is reserved as `../runs/certificate-instance-proof-v2/`.
Inspect the managed unit `twist-certificate-instance-proof-v2.service` and
its receipts before any launch or resubmission. Use one worker, the shared
busy reservation, 8 GiB and 1,200 seconds per proof/export/IDC phase.
An observation timeout is not a restart authorization.

This closes neither a curve/decoder/group bridge nor a parametric-family,
full-panel, complete-cost or asymptotic claim. All three existing panel
owners retain their workloads and deadlines. The raw16 and closure8 first
system results are separately audited/repeated by that registered owner;
they are not inferred from this instance proof. A final report, diagram and
inspected PDF remain required after the proof attempt is terminal. A repeated
factor excludes the current distinct-factor family; whether signed multisets
produce nonzero folded rows is a separate unresolved relation-yield question.
