# Multiprecision auxiliary setup prototype

Native C++17 GMP/FLINT point halving and degree-two tower setup now builds,
with a separate OpenSSL BN-polynomial/EC_GROUP replayer. Four supporting
ring/field identities have a passing local canonical Lean receipt. Numerical
qualification is pending the required S3 checkpoint upload access.
[pcc:unverified]

Read [SOURCE-AUDIT.md](SOURCE-AUDIT.md) for actual contracts/status, the
[frozen protocol](PROTOCOL.md) for the one planned qualification, and
[PUBLICATION.md](PUBLICATION.md) for source/dependency history. This prototype
does not include relation generation, a DLP target or a scalar-recovery claim.

The [two-page report](report.pdf) and editable [diagram](pipeline.dot) explain
the arithmetic contracts and pending checks. Start on page1. Current data
claim PCC-20261115-261591 remains unverified.

Build without running numerical cases:

    bash research/big_prime_halving_20261115/build.sh PREFIX NEW_BUILD

The source was frozen at f5b318b482 before numerical calls. Original build001
diagnostics and passing build002 bindings are retained in checks/. The old
word-prime sampler and historical raw rounds remain unchanged. Do not reuse a
build, wrapper, raw round or manifest path for another attempt.

Before the pending qualification, restore this branch's saved RESUME.md to
.resume/RESUME.md, pass scripts/checkpoint.sh gate using the restored upload
access, then run the exact copied wrapper and arguments in that resume note.
The repository's tracked historical resume note is preserved on this branch.
