# Chain-free Koblitz index calculus with a half-dimension Frobenius-invariant base (L2), `icv1-f2m31-tm90707-c95f16f5` (K_0 over F_2^31)

Lead L2 of [`../ecdlp_exponent_assessment_20261018/LEADS.md`](../ecdlp_exponent_assessment_20261018/LEADS.md).
The scaling note (`research/notes/ecc2k130/RESEARCH_KOBLITZ_SCALING_TARGET.md`,
"The chaining term is avoidable") observes that a Frobenius-invariant
factor base of dimension near `n/2` makes `m = 2` summands enough, so the
`(m − 2)·n` chaining unknowns disappear and the binding constraint moves
to materialising `2^{dim}` points. This session measures that trade on
``icv1-f2m31-tm90707-c95f16f5` (K_0 over F_2^31)` (`icv1-f2m31-tm90707-c95f16f5`, `r = 1 439 393 ≈ 2^{20.46}`,
`A = 62`) with every phase charged, against the strong signed-Frobenius
rho reference and its A/A control. Counted group operations; a measurement,
not a performance claim; no `S` on any leaderboard changes.

## Spec and sessions

- [`spec.json`](spec.json): arms `rho-strong` (reference), `rho-strong-aa`
  (control), `ic-m2-d{10,12,14}` (`koblitz-standard-subspace:dimension=D`,
  oracle `pdp3-koblitz:m=2,engine=rootfind-negfold`, the folded pair
  table, `max_trials` 2 000 000), `ic-m3-d10-f4` (the one-link chain:
  `m = 3`, `pdp3-koblitz:m=3,engine=inherited-f4,degree=3,node_budget=8192`).
  4 targets, 2 rounds, 1 warm-up, alternating, 2 400 s per run, L0.
- [`sessions/n31-interrupted-d16/`](sessions/n31-interrupted-d16/): the
  first attempt also carried `ic-m2-d16`. Its warm-up run **crashed at
  the factor-base phase with `memory allocation of 36507222032 bytes
  failed`** (signal 6; `exec/000005.stderr`): the folded pair table at
  dimension 16 asks for 36.5 GB on a 15 GB host. The session was then
  interrupted by hand (the `ic-m3-d10-f4` warm-up record in that directory
  is the interruption, signal 15/9, not a failure of the arm) and is kept
  as evidence of the materialisation wall. The spec's own description
  says "timed out"; this README is the correction: it was the allocation.
- [`sessions/n31/`](sessions/n31/): the rerun without dimension 16, same
  spec otherwise.

## Warm-up readings (both attempts agree to the digit; counts, not wall)

| arm | `S = gae / √r` | note |
|:--|--:|:--|
| `rho-strong` | 1.296 | floor `√(π/124) = 0.159`; the strong reference at this size is start-up dominated (REPORT §5) |
| `ic-m2-d10` | 354.1 | pair table of `2^{10}` points |
| `ic-m2-d12` | 3 772.7 | |
| `ic-m2-d14` | 56 799.7 | ×16 per +2 in dimension: the table, not the search, is the cost |
| `ic-m2-d16` | crashed | 36.5 GB allocation |

The measured rounds, the replay audit, the `ecbench report` output and
the reading are filled in below when the session completes.

## Reading (to be completed)

The `×16` per two dimensions is the pair table's `|F|²` growth; with
`|F| = 2^{dim}` and the folded table of `≈ |F|²/(2·2n)` sums the set-up
alone is `≈ 2^{2·dim}/(4n)` group operations against `√r ≈ 2^{10.2}`
for the whole rho solve. At `dim ≈ n/2` that is `≈ r/(4n)`, which is
below `√r` only while `r < 16 n²`, i.e. never at a size that matters.
This is the LEADS.md "Against" paragraph made concrete: the chain-free
`m = 2` route trades the chaining unknowns for a table that is itself
`r/n`-sized, so it measures the orbit saving without being a candidate.
The `m = 3` one-link arm is the first chained case and is the number this
session was run for.
