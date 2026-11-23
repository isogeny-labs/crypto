# Bounded complete-system CAS development qualification

Supplement to PROTOCOL.md; freeze before any new CAS submission. This is
exact algebra/correctness development, not a time or parameter-search
experiment. The full m=2–7 goal remains open beyond these two small cells.

Use the existing, unchanged native affine-linear preprocessor on **both**
dense and tower arms. Generate original and residual exact systems at the
already-defined depths2 and4, m=2, deterministic planted targets. Require
each target to be finite. Homogenize the residual **generators**, not an
already-computed Gröbner basis, using a common new H. Certify coefficientwise
that substituting H=1 recovers every residual equation. Export full source,
residual/reconstruction certificates, program bytes and hashes.

Submit at most **two** sequential requests to the already-qualified public
Magma REST endpoint https://calculator.magma-maths.org/execute :

1. A known homogeneous CI of degrees2,3 over the exact P-256 field, with
   quotient regularity3/ideal regularity4, followed by both depth2 arms.
   Check Regularity(QuotientModule(J)) and print its Betti table. Compute
   the affine rational variety and reconstruct every target point using
   the existing exporter. Replay using unchanged GMP/OpenSSL adapter and
   compare with complete native references. Depth2's one ordered tuple
   is only an API/certificate qualification, excluded from goal promotion.
2. Only if request1 passes transport/admission, CI check, all finite-point
   replays and complete references: the two depth4 arms, same operations.
   Check measured regularities, with reg(J)=reg(S/J)+1. Retain both arms
   even if the candidate worsens. A timeout/failure in either arm is not
   a value or a negative mathematical result.

Use SetQuitOnError(true), no SetNthreads call, maximum20-second Magma
alarm and25-second HTTP deadline, one request at a time, zero retries.
Stop after any access/rate/capacity rejection or ambiguous transport.
Keep the advertised1GiB/51200-byte input/20480-byte output limits and
inspect returned truncation/warnings/success flags. No paid/cloud resource
change. The prior continuation's nine-request allowance is not reused.
Return all raw requests, responses, failures and exact replay outcomes.

The public service does not provide independently pinned host/image/CAS
binary identities. Returned CPU metadata are excluded from performance
claims. CAS-reported regularity and Betti tables remain unsigned producer
evidence until independently checked. A passing unit does not establish
affine F4 solving degree, full m=2–7 coverage, runtime or novelty.

The exporter uses a byte-identical prefix of the frozen linear_reduce.cpp
up to (excluding) its CLI main, included by a new C++ translation unit.
The assembler only selects source text, performs no algebra, and asserts
exactly one matching CLI boundary. The original file is never edited.
