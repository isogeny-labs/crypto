# 14-bit single-curve degree-32 component panel — 2026-11-25

The 14-bit roster curve is fixed: `icv1-fp14-tm43-158b6914` for every
`--seed`. The primary degree-32 map, `prime-isogeny:t=5,seed=5,aux=ordinary`,
has 21 abscissae on it. No power-of-two interval `prime-interval:t=5` or
`t=6` with a shift in `[0,64]` gives 21, so the primary map has no exact-size
control at 14 bits.

A size-only scan of ordinary t5 auxiliary seeds `0..31` found seed 7 at 15
abscissae, matched exactly by `prime-interval:t=5,lo=1` (also 15). This
protocol therefore tests the degree-32 construction at 14 bits with auxiliary
seed 7 at 15/15. It is a different auxiliary seed from the 13-bit seed-5
panel, and all target seeds share one curve; it is not three independent
instances.

## Binary

Binary SHA-256
`1ae5170f9b3362f75a636909e1f5d01fb0ead0fd4886627d6012e2b9de9d8d08`,
built from the worktree with the `prime-interval` factor-base registration in
`src/bin/ic/bench.rs`. The three PKM workload sources keep the hashes in
`SOURCE_BUILD_RECEIPT_20261124.md`.

## Frozen run

For each target seed in `20261301`, `20261302`, `20261303`, use 16 repeats,
`--bits 14`, `--family generic`, `--targets walk`, one rho context run, 1,000
target trials, a five-second solver-call budget and a 240-second whole-call
cap, in the order interval, candidate, candidate, interval:

- interval: `prime-interval:t=5,lo=1` with oracle
  `pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=1,mode=chain`
- candidate: `prime-isogeny:t=5,seed=7,aux=ordinary` with oracle
  `pkm-semaev:m=2,engine=f4,degree=8,mode=chain`

Preflights must reproduce the same curve and 15 abscissae in both arms.
Preserve every command, full JSON, external wall and child CPU in an
append-only store.

## Gate

The component gate passes only if all four invocations exit successfully, both
arms stay at 15 abscissae, all 16 recovered scalars agree across the four runs,
candidate/interval charged GAE and wall and child CPU are below one in both
mirrored pairs, and combined charged GAE is below 0.9.

This is a 14-bit component signal on one curve with three target replicates.
It is not a three-curve replication, not a rho crossover and not transfer to
s59, P-192 or NIST.
