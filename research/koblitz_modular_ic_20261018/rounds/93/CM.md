# R93 rejected principal-open overlap attempt

The single frozen `overlap93-01` native attempt is rejected. The strict
driver recorded native exit 0 but found `?` errors despite the printed
`OVERLAP93_ALL_CHECKS_PASS` marker. The first error was
`lift(number,int) failed` at `poly uinv93=lift(u,1)[1,1]`: in the chosen
F2(u) field, `u` is a coefficient, not a polynomial variable. Subsequent
undefined-symbol errors are consequences. No complete unit-map or gluing
certificate is accepted from this attempt.

Before that error, native output reported all seven named functions as
polynomial members of the rank-32 module, with 32 entries each and full
`B*C=F*W` checks. This is diagnostic evidence only until a clean replay
from corrected source passes. The polynomial identities for h and k were
also evaluated before the error; the strict all-or-nothing acceptance rule
keeps them outside the accepted result for R93.

The raw attempt is retained in `runs/overlap93-01` and deterministic
`checkpoint-overlap93-01.tar.xz`; hashes are in `runs.sha256` and
`checkpoint-overlap93-01.tar.xz.sha256`. The source was pushed first at
`3db2153e6`. The 18-member archive SHA-256 is
`08729f5abdca2c8c72b0c908e5f0a6aa6b865921b2e1fa2b5df4e627debef070`.
Checksum-protected upload to `setec-astronomy-artifacts`, fresh readback,
byte comparison, and all 18 member hashes passed; the receipts are saved
beside this note. The repair is a new frozen round using direct coefficient
inversion `1/u` and the same seven-function scope. It must reject native
`?` errors regardless of a pass marker.

References: [Stacks gluing](https://stacks.math.columbia.edu/tag/01JA),
R50 `RR-ALGEBRA.md`, R88 `INVERSE88.sing`, R91 `CM.md`.
