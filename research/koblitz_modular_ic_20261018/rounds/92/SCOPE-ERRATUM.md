# Prior-work correction for R92

R92's frozen protocol described the 528-product table as the next algebra
check. R50 `RR-ALGEBRA.md` and the accepted `rr-mult50-02` plus fresh
`rr-table-reload50-01` had already completed that table. The R50
`NEXT_SOURCE_NORMALIZATION.md` sentence about remaining multiplication
checks is stale relative to those accepted R50 records.

R92 is therefore an independent reconstruction and exact comparison,
not the first closure proof. Its different computation works directly
with normalized generators in the quotient field and solves coordinates
using R50's saved `S`, while R50 multiplied numerator polynomials and
used a PID lift. The post-run `COMPARE50_92.sing` checks all 16,896
coefficients and 528 pair labels against the retained R50 table; they
agree exactly. This scope correction does not alter R92's frozen source
or raw attempt.
