# An AVX-512 IFMA native sweep for rootfind-subtract (protocol, frozen before the run)

## Question
`research/ic_native_sweep_20261115` measured `worker=native` (scalar 64-bit Montgomery, one target per
thread) on par with the C++ emulation arm. `simd=ifma` (commit `1deabdf708`) sweeps eight targets per
step in AVX-512 IFMA lanes (Montgomery with R = 2^52), each lane with its own batched inversion, and
probes the same candidates in the same order. Does it speed up the relation phase on the same host?

## Arms (`specs/ifma2.json`, `specs/ifma3.json`)
The workloads of the native-sweep study (`m = 2` on 28/30-bit curves at |F_x| = 4096; `m = 3` on the
34/36-bit prime_explicit curves at |F_x| = 2048; 2 targets, 2 rounds, alternating): `rootfind-subtract`
(one thread), `worker=native` with `threads=1` and with all threads, each without and with `simd=ifma`;
`rho.negation`.

## Predictions
1. Identical trials, relations, logarithm, S and every phase's charge across the IC arms.
2. `m = 2`: the IFMA arm's relation phase is at least 1.5× faster than the scalar native arm's, at one
   thread and at all threads.
3. `m = 3`: smaller or no gain (the probe of the pair table's abscissae, a 64 MiB set, is memory-bound).

## Reading
Local, L0 (`--cpus none --allow-busy`) on this 4-thread VM (AVX-512 IFMA present); wall figures are
indicative. The one-thread pair isolates the arithmetic from the thread pool.
