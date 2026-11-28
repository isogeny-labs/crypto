# Bounded algebraic adapter qualification, 2026-11-16

This is development correctness qualification, not a performance experiment.
The original depth8, m2..7 matched2x request in ../PROTOCOL.md remains open.
No timing comparison, structured-regularity value or novelty claim is made.

## Sources and scope frozen before the first CAS submission

Resume source head506deadb017935da7b6f01f215c5e7d9485e20b8; canonical main
sync pin a1c98c8e85c32f910b12d5119ca0cd52f15391d6. Read the existing study
protocol, frozen constructor/Magma adapter, PKM reference and current
Magma handbook interfaces. KB unavailable: no exposed KB tools and the
research-kb wrapper exits127; reuse the source-linked review in ../REPORT.md.

The official public calculator documents a REST API:
https://github.com/Magma-Maths/calculator/blob/main/README.md . The deployed
frontend https://magma-maths.org/static/js/calculator.js selects
https://calculator.magma-maths.org/execute . GET /info receipt is retained.
It advertises1GiB guest memory,300-second CPU/wall limits,51200 input bytes,
20480 output bytes, and at most two concurrent jobs per IP. Our client is
stricter: one request at a time,25-second HTTP cap, no automatic retries,
at most nine computation requests under this qualification. Respect rate
limits and stop on an access/rate/capacity rejection rather than bypass it.
Only already-public parameters, code and deterministic synthetic fixtures
are submitted; the service states that code/IP may be retained90 days.

## Steps and correctness controls

1. One field/GB/rational-variety API probe at the exact P-256 prime. Preserve
   request bytes/hash, response JSON, HTTP status and endpoint. A response
   must have success=true, exit_code=0 and truncated=false, without warnings.
2. Qualify the frozen exported depth1, m2 planted instance, four arms:
   dense, tower, paired, oriented. All use exactly FB1hfa6130558c05 and the
   same target. Compare original projected ordered tuples with the complete
   native reference. Expected multiplicity from the existing native fixture:
   two ordered tuples; paired has four auxiliary-sign copies per tuple.
   Retain every original response and replay every reported point using
   exact native GMP/OpenSSL arithmetic. Zero projected coefficient rows at
   this unit target are not usable independent relation rows.
3. Prepare a native affine-linear elimination preconditioner, with explicit
   constant nonzero pivot certificates and exact solution reconstruction.
   It may be qualified on the same four cells, within the remaining four
   requests, only after native positive/negative certificate tests pass.
   Neither fewer variables nor a lower input/output-basis degree is a
   degree-of-regularity or speedup result.

If an exporter syntax or reader defect is found, retain the failed attempt
and correct it with a source-bound amendment before any replacement test.
An infrastructure rejection does not count as a mathematical failure.

## Explicit accounting and limits

The endpoint is a shared public CAS service, not a qualified EKS/L2 host.
Its CPU/memory metadata are retained as returned and are not used to claim
a speedup. No request here substitutes for the original m2..7 depth8
benchmark, paid setup/replay/conversion accounting, independent standing,
regularity proof or original full-depth promotion gates [pcc:unverified].

Record request counts and every failure. A completed CAS marker is a
producer observation; correctness must be checked independently in the
native arithmetic, and completeness is compared only on the complete
small domains specified above. No background jobs or retries are implied.
