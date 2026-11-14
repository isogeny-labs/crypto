# Amendment 1 (before the recorded run; PROTOCOL.md is not edited)

PROTOCOL.md is frozen by `frozen.sha256`. This amendment is hashed in the same
file, after the smoke test and before the recorded run. Everything here is
`[pcc:unverified]`.

## What the smoke test showed (scratch output, not recorded)

A first build of `instrument_rerun v3` run into a scratch directory (not the
record, not the output directory) gave, before this amendment:

1. The gate passed: candidate, rho on E1 (428, 357, 370, 847), IC group ops,
   trials and LA mulmods equal the v3 record.
2. The recorded LA counter is `(fb + 2) x rows + 1` in all four rows (the
   PROTOCOL's reading, with the final inversion product). The counted
   elimination of the same rows (arm L) costs 1.02 to 1.03 times as many
   multiplications, so frozen prediction P1 (a factor 2 to 60) is already
   contradicted; it is scored refuted in RESULTS.md.
3. The tuned negation walk (`rho_reference_walk`) has a set-up of about 50
   operations where the recorded `rho_on_ec` spent about 400. The matched
   rho total at p = 211 to 1619 is 63 to 92 operations, not 357 to 847. Every
   recorded distance was therefore divided by a rho that was 5 to 9 times too
   expensive. This is not a prediction in the protocol (P5 said the negation
   total would be within 0.7 to 1.3 of the plain total, which refers to the
   tuned plain walk and is unaffected; the comparison to the recorded old rho
   is a separate quantity) and the recorded-rho column is kept in the tables.
4. `ic_control::run_ic` restarts its walk from a random point every 4096
   steps. In a subgroup of order `r` of a few hundred elements the walk repeats
   after about `sqrt(r)` steps, so 99% of the trials were repeats (C at p = 419
   spent 1.7e5 group operations).

## The change

`run_ic_with(inst, params, restart)` is added to `ic_control.rs`
(`run_ic` calls it with the old 4096, so the positive control is unchanged).
`instrument::ic_on_rung` passes `restart = clamp(ceil(sqrt(r)), 4, 4096)`.
Nothing else in PROTOCOL.md changes: arms, factor-base rule, seeds, primes,
pricing, decision rule and predictions stand as frozen. The recorded run is the
first run after this amendment with the output directory as target.
