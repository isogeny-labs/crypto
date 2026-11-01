# First device run of the ECCp-131 CUDA rho walk

The [CUDA port](../../research/certicom131_cuda_port_20261023/README.md) compiles
and passes every CPU gate, but no CUDA kernel in it has ever executed. This
card is the one run that changes that: a single launch on one GPU, its parity
gate, and the first measured walk rate. It is a device-correctness run, not a
campaign, and it claims no exponent and no solve.

## Question and decision value

Does the device end state equal the CPU end state word for word on ECCp-131
(and on the ECC2-131 arm that shares the same thread function), and what is
the sustained rate in walk steps per second?

Both answers decide funding. A measured rate turns the 2^65.3 group-operation
estimate for ECCp-131 (parallel rho, sqrt(pi n / 4)) into an extrapolation
from hardware rather than an assumption, which is what a campaign proposal
needs. A nonzero word difference means the kernels are wrong even though the
CPU emulation agrees with itself, and the port stops until the arithmetic is
fixed. Nothing else on the shelf can answer either question.

## What exists, and what is not known

`research/certicom131_cuda_port_20261023/` holds the per-thread walks
(`gpu_walk.cuh`), the kernels and device gate (`walk_cuda.cu`), the host
emulator (`host_emul.cpp`) and two frozen outputs, `results.json` and
`report.pdf`. What is known, all CPU-side:

- The host emulator passes the field oracle and the one-step oracle on every
  lane, with zero off-curve end points and zero exceptional lanes at 12/6 and
  200/40 iterations; the end state is independent of the thread/batch split
  and folds to `a4a414fc843c9989` on ECCp-131.
- `ptxas` 12.9 assembles the ECCp-131 kernel at 64 registers, 1,992 B stack and
  no spills, and the ECC2-131 kernel at 255 registers with some spills, for
  sm_80/86/89/90 and, by retargeted PTX, sm_120.

What is not known: whether any of that runs on a device without losing a word,
and the device rate. The port's README says so in its first line.

## The frozen run

```sh
git checkout 37d76b61ad4399c376dc5ac97321ee9e49e66747
make -C research/certicom131_cuda_port_20261023 build/cudashim.h cuda-run SMARCH=sm_120
```

The first Make target creates `build/` on a clean checkout before `cuda-run`
writes `build/walk_cuda`; `cuda-run` does not depend on that directory-creating
target itself.

`cuda-run` builds with `nvcc -O3 -arch=sm_120` and then runs
`build/walk_cuda --threads 65536 --steps 8`. Inside that process, in this
order and with no dependence on the flags:

1. **ECCp-131 parity (fixed):** 64 threads x 8 lanes, 12 steps, seed `0x1318`.
   The host computes the reference in the same binary, the kernel runs on the
   device, and the two states are compared word by word.
2. **ECC2-131 parity (fixed):** 8 threads x 4 slots x 32 lanes, 6 steps.
3. **ECCp-131 rate:** a warm-up launch, then one timed launch of
   `65536 x 8 = 524288` lanes for `--steps` steps, reported in `steps_per_s`.
4. **ECC2-131 rate:** the same shape for the bitsliced arm.

The JSON lines are `{"kind":"device_parity",...}` and
`{"kind":"device_rate",...,"note":"single timed launch, not a campaign rate"}`,
and the process ends with `{"kind":"summary",...,"pass":true|false}`. Exit
status is nonzero on any parity difference or exceptional lane.

## Success, failure, stop

- **Success:** both `device_parity` lines carry `words_differing = 0` and
  `exceptional = 0`, `summary.pass` is `true`, exit status is 0, and both
  `device_rate` lines carry a positive `steps_per_s`. ECCp-131 is the claim;
  ECC2-131 is the second arm and a failure there is reported, not tuned away.
- **Failure:** any nonzero `words_differing` or `exceptional`, a failed launch,
  or a missing rate line. Report the line and stop; do not edit the walk in
  the same run, because a fixed kernel would need its own card and its own CPU
  gates.
- **Stop:** one launch of each arm. If the run has to repeat, the seed and the
  command are unchanged, and both attempts are reported.

## Cost

One GPU for under an hour, and the number is a bound rather than a
measurement:

| input | value | source |
|:--|:--|:--|
| device | 1 x RTX PRO 6000, sm_120 | the sm_120 target the port was retargeted for |
| timed work | 524288 lanes x 8 steps, twice | `Makefile` `cuda-run`; `walk_cuda.cu` `run_prime` |
| parity work | 512 lanes x 12 steps, 256 lanes x 6 steps | `walk_cuda.cu` `run_prime`, `run_bin` |
| build | one `nvcc -O3` translation unit | `Makefile` |
| cap | 1800 s per shard, wall <= 1 h | `run.timeout_per_shard_s` |

No core-hour estimate is given: no device rate exists yet, and inventing one
is the error this card exists to remove. The cap is one GPU, one container and
one working tree; nothing here parallelises.

## Completion

The result lands as a PR that updates
`research/certicom131_cuda_port_20261023/` — `results.json` gains the four
JSON lines and the exit status, the report gains a device-parity section and a
rate panel, and the README's "Not run on a GPU" paragraph is replaced with the
device, driver and date. The same result PR also adds the benchmark record:
`benchmarks/<YYYYMMDD>-certicom131-cuda-device-run/` contains the run and
results PDFs plus SVG and 150-dpi PNG copies of every graph, and
`benchmarks/README.md` gets a row for the measured device rate. Label the rate
as a single-launch diagnostic, not a campaign rate. Include the replay
certificate, validator report and matched rho reference required by the
benchmark validation gate, and generate the study and browsing copies from the
same source. Verification is `make host-test`, which re-derives the CPU
reference the device matched and must still fold to `a4a414fc843c9989`.

## Why this is blocked here

The lanes that wrote and reviewed the port have no GPU (the authoring machine
is a macOS host) and no RunPod API key in their session environment, so they
cannot launch it. The command is frozen, the commit is merged, and the gates
are written; what is missing is the device.
