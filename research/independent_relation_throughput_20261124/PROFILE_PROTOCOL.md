# Profile protocol: sibling collector wall regression

Frozen before profile collection on 2026-11-24. A profile explains cost and does
not establish a speedup. [pcc:unverified]

## Question

Why does `wwalk:...,sib=8,pool=256` use fewer counted operations than the plain
two-seed walk yet take longer end to end in the existing local panel?

## Arms and fixed inputs

- Baseline: `sib=8,pool=64`, the current ratcheted sibling configuration.
- Diagnostic candidate: the same oracle and target stream with `pool=256`.
- Same explicit k = 28 prime curve, public target seed, measurement seed,
  factor base, `sib=8`, lazy verification, one thread, and release binary.
- Profile the setup/collection path only after checking the run's relation rows,
  rank, recovered scalar, and deterministic counters against an unprofiled run.

The first profile is CPU sampling with DWARF call graphs. If the host cannot
sample, use Callgrind. If a single solve yields fewer than 1,000 samples, loop a
fixed list of identical profile jobs inside one profiled command; do not change
the curve, target, or arm between repetitions.

## Predictions and falsification

P1: subset selection, queue/map/reference bookkeeping, or sibling point lifting
contains a sibling-specific hotspot with at least 20% self or inclusive share.
Falsified if no named sibling-specific path reaches 20% within two standard
errors in a profile with at least 1,000 samples.

P2: pool 256 spends a larger share than pool 64 in product-pool setup or memory
bookkeeping, rather than in field inversion/multiplication. Falsified if the
profiles order the arithmetic path above the bookkeeping path by more than two
standard errors and the Callgrind instruction totals agree with that ordering.

## Controls and outputs

- `tools/profile.sh probe` records host capabilities.
- Both arms use the same profiler mode, frequency, fixed job loop, and binary.
- The unprofiled output digest and every deterministic relation/rank counter must
  agree with the corresponding profiled arm. A mismatch invalidates the profile.
- Preserve unsupported profiler attempts and command failures as results.
- Profile outputs live under `profiles/<arm>-<mode>/`; raw profiler data is not
  committed loose.

The next implementation hypothesis and the paired measurement protocol are
written only after reading these profiles.

