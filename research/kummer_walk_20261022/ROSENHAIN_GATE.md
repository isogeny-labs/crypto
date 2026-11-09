# Rosenhain setup gate: separate addendum

Run ID `ROSENHAIN-SETUP-20261022`, governed by the separate
[frozen protocol](ROSENHAIN_PROTOCOL.md). The [PDF](ROSENHAIN_GATE.pdf) and
[graph](rosenhain-setup.svg) plot the raw model values.

| Observation | Value |
| --- | ---: |
| Independently factored Scholten quintics checked | 21 / 21 |
| Ordered finite-root pairs per model | 20 |
| Pairs with square scale per model | 10 / 20, on all 21 models |
| Selected `Fp` Rosenhain maps found | 21 / 21 |
| Exact transformed/reconstructed coefficient identities | 21 / 21 |
| Theta constants, Kummer coordinates or differential formulas verified | 0 |
| Kummer collector DLP targets solved in this addendum | 0 / 1,260 |
| Candidate/rho cost ratio | null |

For `f(x)=c5∏(x-rj)`, choose roots `r0,r1`, put
`delta=r1-r0`, `t=(x-r0)/delta`, and `Y=y/sqrt(c5*delta^5)`.
The remaining transformed roots are `lambda,mu,nu`. The native gate
independently expanded `f(r0+delta*t)/(c5*delta^5)` and the product
`t(t-1)(t-lambda)(t-mu)(t-nu)` and compared all six coefficients exactly.
It recorded every model, 20 pairs tried, square-pair count, chosen map,
and both coefficient vectors in
[rosenhain-models.jsonl](results/rosenhain-models.jsonl).

On the first model `p=739`, the selected map has `r0=246`, `r1=369`,
`delta=123`, `scale=179`, `sqrt_scale=87`, and
`(lambda,mu,nu)=(296,16,10)`. All 21 rows have `all_correct=true`.
The native [validator](src/bin/validate_rosenhain.rs) checked distinct
normalized roots, the square root, exact coefficient vectors and unique
model primes. A separate execution of the released producer yielded a
byte-identical JSONL file. The [summary](results/rosenhain-summary.json),
[manifest](results/rosenhain-manifest.json), [dictionary](ROSENHAIN_DATA.md),
and [digest file](ROSENHAIN_SHA256SUMS) preserve review metadata.

This establishes an `Fp` Rosenhain equation for each quintic. It does not
establish that the theta constants needed for Gaudry's fast Kummer model lie
in `Fp`, nor any Jacobian-to-Kummer projection, fast xDBL/xADD, exception
rule, or signed walk recurrence. Those are the next separate implementation
and proof gates. The original 1,260-target full-collector cost requirement
remains open in the [problem record](../../problems/genus2-kummer-relation-labels.md).

The relevant published fast Kummer construction is
[Gaudry, *Fast genus 2 arithmetic based on Theta functions*](https://eprint.iacr.org/2005/314/).
The published signed-recovery framework is
[Chung, Costello and Smith, *Fast, uniform scalar multiplication for genus 2 Jacobians with fast Kummers*](https://eprint.iacr.org/2016/777/).
