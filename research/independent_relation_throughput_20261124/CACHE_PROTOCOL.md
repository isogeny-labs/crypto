# Parent-residue cache experiment

Frozen before the candidate profile on 2026-11-25. This is a controlled
instruction-cost discriminator, not a wall-time or throughput claim.
[pcc:unverified]

## Hypothesis

`sib_expand` repeatedly reduces the same packed parent seed coefficient for
each minus child. Caching the two residues once per parent and planning each
minus coefficient before the group batch will reduce measured-child solve
instructions without changing the generated points, relation rows, rank, or
recovered scalar.

The change retains the existing bounded integer representation for plus
children. It only replaces repeated evaluations of
`submod(parent_component % r, jump_component % r, r)` by one cached
`parent_component % r` and the already-bounded jump component.

## Frozen inputs and baseline

- Source baseline: commit `ea059ddfbe9feb0ccac68da2f3b1fc3fe95d2a97`.
- Baseline workflow run: `45558180700`; artifact id `14998186273`; artifact
  zip SHA-256 reported by GitHub Actions:
  `e149c6efb27ae907af1bef27d6aecd4282a6fcad439c15063ec1cc60ffdf982b`.
- Frozen spec:
  `research/curve25519_autolab_20261104/siblings/stage1-eks/spec-k28-local.json`.
- Pool 256: sequence 242. Pool 64: sequence 243. Both name workload
  `W636dac909917`, measured round 1, and algorithm seed
  `5769128089488788324`.
- Baseline solve instructions: 7,883,046 (pool 256) and 5,210,850 (pool 64).
  Baseline `generic_ic_precompute`: 6,757,351 and 4,084,179 instructions.

## Controls and decision

The candidate uses the same workflow, stable Rust toolchain, release profile,
Valgrind 3.22.0 settings, spec, and sequence numbers. For each arm, its profiled
and unprofiled measured-child outputs must match on error status, workload and
method identity, recovered scalar, exhaustion, deterministic work, and counters.
Both arms must retain rank 3 from three rows with zero dependent rows.

The micro-hypothesis passes only if both arms reduce solve instructions by at
least 5% and neither arm changes any semantic control. Otherwise the cache is
discarded or revised, with the exact failure preserved. A pass admits—but does
not establish—a separately frozen paired L2 wall/rank experiment. The primary
study metric remains new verified independent rank per charged collection
second; instruction reduction alone cannot promote the implementation.

No Lean work is admitted by this micro-result alone. If the implementation also
wins the paired throughput experiment, Lean may prove that caching residues and
using them in child subtraction preserves the coefficient action in an
order-`r` group.
