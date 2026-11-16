# R92 independent reproduction of the R50 rank-32 algebra table

R92's single frozen native attempt `multiply92-01` passed the unit and
all 528 unordered products of the accepted R50 basis in the actual
degree-32 source field. For every pair it reconstructed the reduced
power-basis vector, checked `B*C=F*W`, required all 32 coordinates to
be polynomials, and rejected one corrupted coordinate. The strict
driver reported `PASS`, native exit was zero, and there were no `?`
errors. The source was pushed before execution at `a58703742`.

Two independent scripts ran on a fresh S3 readback. `REPLAY_TABLE92.sing`
checked the unit, the full 32-by-528 product over an **ideal-free**
`F2(u)` field, all 528 column products, 16,896 polynomial entries,
pair indexing, and 528 corrupted-coordinate controls.
`REPLAY_PRODUCTS92.sing` separately recomputed all 528 products in the
source quotient field and matched their exported power vectors.

R50 had already accepted a multiplication table. The post-run
`COMPARE50_92.sing` loaded the R50 `rr-table50.sing` (SHA-256
`e5fe417a6618346fa4e093482bd71a4c91993397125bdc3d9bbab3951f1b0752`,
matching R50's `runs.sha256`) and compared every pair label and all
16,896 coefficients against R92. They agree exactly. R92 therefore
independently reproduces and strengthens the earlier table's checking
path; `SCOPE-ERRATUM.md` corrects the initial novelty framing.

The 8,290,646-byte R92 export has SHA-256
`e7218bfb9e2944a8d9a1b54974dd981c77ae11106ceb8fd5626c86ff13cce395`.
All 18 raw members are in deterministic archive SHA-256
`47a649a68bc6658cc3e4187129ce2aee3bc628a5ce41f55ec9ba2f6176841784`
at `s3://setec-astronomy-artifacts/store/sha256/47/47a649a68bc6658cc3e4187129ce2aee3bc628a5ce41f55ec9ba2f6176841784`.
Checksum-protected PUT, fresh GET, archive digest and all member hashes
passed.

The accepted R50 module is a finite free `F2[u]` algebra inside the
source field: unit membership and all basis products are checked;
associativity and commutativity are inherited from field multiplication.
This algebra closure is not by itself a normality or proper-chart
certificate. The next useful construction is to bind the two R91 chart
maps and R50 multiplication table to an explicit gluing cocycle, then
verify the missing boundary fibers and the modular-factor action.
