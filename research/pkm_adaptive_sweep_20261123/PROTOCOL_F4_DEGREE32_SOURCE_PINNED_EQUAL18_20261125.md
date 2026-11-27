# Source-pinned degree-32 equal-18 component panel — 2026-11-25

The source-pinned replay of the original 16-abscissa workload failed the size
gate: binary `c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`
(commit `a5503f4bb66ac3aa4eac3125fba01534d2574096`) produced 18 lifting
abscissae for `prime-isogeny:t=5,seed=5,aux=ordinary` on
`icv1-fp13-t19-0d05ae31` while `prime-abscissa:below=32` produced 16. The
original degree-32 result is therefore not source-reproducible at its recorded
size.

This protocol starts a new, source-pinned equal-size test. It does not claim to
replay the original 16-abscissa run.

## Selection

A size-only scan of 13-bit CLI seeds `20261181..20261220` compared the
abscissa count of `prime-isogeny:t=5,seed=5,aux=ordinary` with
`prime-abscissa:below=32`. Exactly one seed matched at 18 abscissae:

- seed `20261185`
- instance `icv1-fp13-tm13-9e2d11f6`
- both arms 18 lifting abscissae

The scan was frozen before any timed call. This is a selected instance; the
selection effect must be reported with the result.

## Frozen run

Binary SHA-256:
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`.

For seed `20261185`, use exactly 16 repeats, `--bits 13`, `--family generic`,
`--targets walk`, one rho context run, 1,000 target trials, a five-second
solver-call budget and a 240-second whole-call cap, in the order
interval, candidate, candidate, interval:

- interval: `prime-abscissa:below=32` with oracle
  `pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain`
- candidate: `prime-isogeny:t=5,seed=5,aux=ordinary` with oracle
  `pkm-semaev:m=2,engine=f4,degree=8,mode=chain`

Preflights must reproduce the same instance and 18 abscissae in both arms
before any timed call. Preserve every command, full JSON, external wall and
child CPU in an append-only store.

## Gate

The component gate passes only if all four invocations exit successfully, both
arms stay at 18 abscissae, all 16 recovered scalars agree across the four
runs, candidate/interval charged GAE and wall and child CPU are below one in
both mirrored pairs, and combined charged GAE is below 0.9.

A pass establishes a source-pinned small-instance relation-extraction
component lead on one selected 13-bit instance. It does not establish a
same-target rho crossover, a source-exact replay of the original 16-abscissa
result, asymptotic behavior, or transfer to s59, P-192 or the NIST curves.
