# R91: all 32 normalized RR generators transport in both directions

The frozen R91 source was pushed at `7ff259920` before its sole native
attempt, `transport91-01`. It evaluated all 32 accepted R50 PID generators
under the R85/R86 forward and R88 inverse diamond substitutions. The run
emitted 32 column lines in each direction, each after power-basis
reconstruction, a full `B*C=F*W` product, polynomial-coordinate checks,
and a corrupted-coordinate rejection. The native process exited zero,
emitted no `?` errors, and printed the exact completion marker; its strict
driver returned `PASS`.

The pole-clearing powers lie between 44 and 61 in both directions. Each
direction has the same per-column sequence and histogram (CSV in
`publication/power-histogram.csv`), with one generator attaining the
derived bound 61. The sum of the 32 powers is 1782 per direction. These
are exact integer outputs from the first-power search, not timing data.

The exported certificate `transport91.sing` is 1,104,793 bytes, SHA-256
`6cd9d9265ff1f54d40bd343994e1122f95d2e1d031a05a9b2d8acdf3c579d5b2`.
The independent `REPLAY_CERT91.sing` loaded that export from a **fresh S3
readback** in an ideal-free `F2(u)` coefficient-field ring. It verified
both 32-by-32 products, all 64 column products, all 2,048 polynomial
module coordinates, the power ranges, and 64 corrupted-coordinate
rejections. Its log has the exact marker and no native errors.

The first replay attempt used the polynomial ring `F2[u]`; it rejected
the first full product. That ring cannot faithfully load the exported
power vectors because some entries are rational functions such as a
nonzero numerator divided by `u+1`. A notation-normalized copy failed
for the same reason. Both diagnostic logs are retained. Replaying in
the ideal-free coefficient field checks the intended equality without
using the relative curve ideal; the original quotient-field reload was
also checked. The raw certificate was not edited.

The 18 raw run members are in deterministic archive SHA-256
`fdfeaef4c2d514523fde7f82a7622783c26883995381b1dcd0bcf5de1f1f0656`
at `s3://setec-astronomy-artifacts/store/sha256/fd/fdfeaef4c2d514523fde7f82a7622783c26883995381b1dcd0bcf5de1f1f0656`.
Checksum-protected PUT, fresh GET, archive digest, and all member hashes
passed. The Lean `Bound91` module proves the integer inequalities behind
the 61 and 17 pole ceilings under their explicit degree and ramification
assumptions; it does not formalize the cusp valuations or CAS transport.

This establishes a two-way map of the localized rank-32 modules for the
accepted generators. Proper projective boundary behavior and an actual
nonzero modular-factor transfer still require separate construction.
The identical power sequences suggest checking whether a simple
automorphism explains the symmetry, but no such explanation is claimed.
