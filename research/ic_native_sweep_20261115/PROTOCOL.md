# A native multi-threaded CPU sweep for rootfind-subtract (protocol, frozen before the run)

## Question
The multi-threaded CPU arm of the GPU rounds (`backend=emulate`) runs the CUDA kernel's per-thread code
on host threads; round 3's validator found it sweeps 1/7–1/9 as many candidates per core as the
single-thread Rust arm. `worker=native` (new) runs `rootfind-subtract`'s own sweep (Montgomery batched
inversion, two candidates per abscissa, an open-addressing probe set) over each lane batch's targets on
a rayon pool, in process; hits are confirmed and charged by the existing path. How fast is it against
one thread and against the emulation arm, on the same host?

## Arms (`specs/native2.json`, `specs/native3.json`)
`m = 2` on the 28/30-bit `prime_search` curves at |F_x| = 4096 and `m = 3` on the 34/36-bit
prime_explicit curves at |F_x| = 2048, 2 targets, 2 rounds, alternating: `rootfind-subtract` (1 thread),
`worker=native,threads=1` (control: batching without parallelism), `worker=native` (all threads),
`worker=spawn,backend=emulate` (the earlier multi-threaded arm), `rho.negation`.

## Predictions
1. Identical trials, relations, logarithm and S across the IC arms, per workload.
2. `m = 2`: the native arm's relation phase is at least 1.5× faster than the emulation arm's.
3. The native arm with all threads is at least 2× faster than `threads=1` in relation-phase wall.

## Reading
Local, L0 (`--cpus none --allow-busy`) on a 4-thread VM: wall figures are indicative; the
`threads=1` arm is the batching control. Not a GPU comparison (pod unavailable).
