# Cold X1(2,8) equal-size PKM validation cohort

Frozen before selecting the held-out curves or running their map and relation
arms. The six earlier `t4_c*` and `t5_c*` curves were used to develop the
selector and are construction controls, not the promotion panel. This cohort
does not change their workload files or earlier search receipts.

## Held-out workload selection

Use `freeze_z2xz8_holdout.py` with no map-size or solver feedback. At each
field-size rung `bits=11,14`, take the first three primes `p >= 2^bits`
with `p ≡ 1 (mod 32)`, ordered by p. For each p, scan nonzero `(a,b)` in
lexicographic order and choose the first nonsingular curve
`y²=x³+ax+b` with prime group order `r != p`. Exhaustively count points,
take the lexicographically first affine point as generator, and verify its
order. No curve may be replaced after a map failure or slow relation arm.
For `j=0..15`, derive `q=1+(int_be(SHA-256(input)) mod (r-1))` from
`input=pkm-z2xz8-holdout-v1|bits|curve_index|j|retry`, starting retry at zero
and incrementing only when `q` repeats an earlier scalar;
store its exact point `[scalar]G` and hash. Derive the relation seed from
SHA-256 of `pkm-z2xz8-relations-v1|bits|curve_index|j`. Freeze and hash
the six workload files and manifest before screening any selected map.
An independent verifier must replay primality, group order, generator,
target points and all hashes. These are public synthetic benchmark instances.

## Source and map selection

Candidate source is `a0642f42e205aba4951dcf9732721d62211d7c33`,
with Rust 1.99.1 and `prime-isogeny:t=5,seed=0,aux=ordinary,family=z2xz8,selection=equal-size`.
Build `ic` from a clean full checkout. Record binary, source and build-log
hashes in a build receipt before any measured arm; every run verifies them.
The binary must carry the native rejected-attempt trace from draft PR #5382.
If source changes, create a new cohort amendment and new store; never mix
binary versions in one panel.

The candidate scans `m=1..64`, then ascending `R.x`, at most 64 distinct
cosets per nonsingular auxiliary. It finds the lexicographically first Q32
by native rational halving from all three rational 2-torsion points. The
first coset with the same actual target-curve lifting-abscissa count as
`x=0..31` is selected. The only target-curve datum used for selection is
the public curve equation. No target point or scalar enters map selection.
Keep every singular draw, no-Q draw, failed coset, cap and censored result.
An unavailable exact-size map is a failed row, not a substituted curve.

## Cold target arms and complete accounting

For each of 16 frozen targets on each of six curves, launch a fresh process
per arm. The primary candidate repeats the entire auxiliary and coset
selection for that target; charge all rejected work and the selected Vélu
chain. Compare with equal-size `prime-abscissa:below=32` on the identical
target and relation seed, interleaved interval/candidate/candidate/interval.
Run the m=2 F4 chain oracle and separately a certified MITM join on the
selected candidate map. Run the existing `prime-kummer` m=3 F4 control on
the identical target, reporting its actual base size. All arms use checked
full-rank incremental Gaussian elimination and independently verified
output scalars. Run matched negation-map rho on each identical target.
Use the existing explicit degree-32 panel's 20,000 relation-trial and
five-second solver-call caps for m=2, and its 2,000,000-trial cap for the
Kummer control. Cap each cold process at 120 seconds and 2 GiB. Record
process wall, child CPU, peak RSS, all setup, every failed relation and
solver call, joins, rank, linear algebra, verification and rho. A timeout,
unavailable map, or incomplete rank remains censored in the denominator.

Keep operation counts and measured time separate. Price native square-root
modular multiplications, coefficient arithmetic, Jacobi calls, Vélu
evaluations, group operations, memory and all other positive uncharged
native work before reporting a complete GAE total. A missing field price
or positive uncharged counter makes the row incomplete, not zero-cost.
Measure field prices on the same quiet host as the cold panel. Do not pool
hosts. Report any setup reuse across 16 targets only as a separately marked
amortized scenario.

## Promotion

Require source-pinned independent replay of both the native selection
trace and each recovered scalar. Compare all three curves at each size,
with all 16 preselected targets and failed/censored arms retained. Promote
only if complete cold candidate cost beats the equal-size interval and
existing PKM controls on both rungs and the paired cost ratio improves at
the larger rung. Repeat close calls under the same protocol. State a
matched-rho advantage only where the measured complete cold cost actually
crosses same-target rho. Diagnostic construction, a solver-call gain, or
an extrapolation alone cannot pass this gate.
