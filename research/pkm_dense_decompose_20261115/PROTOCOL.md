# Does the dense form also win at finding decompositions?

**Protocol, frozen before any study row.** 2026-11-15. Study id `EXP-PKM-DENSE-DEC-001`. Instrument
[`examples/pkm_algebraic_oracle.rs`](../../examples/pkm_algebraic_oracle.rs) (`--membership both|dense`,
`--vkind coset|random`; library `algebraic_decompose_dense`); runner [`run.sh`](run.sh); rows `runs/`. Counted `F_p`
multiplications only; no clock enters a verdict. Context: EXP-PKM-IDEAS-001 idea 1 (`research/pkm_ideas_20261115/`)
found that membership written as one dense polynomial `∏(x − v)` makes F4 *refute* systems far more cheaply than the
degree-2 tower. That was refutation only. This study asks the question that matters for relation collection.

**Question.** On the same factor base `V` and the same targets, is the slides' relation oracle (F4, then the counted
linear stage that reads every solution off the basis, then lifting and sign checks) cheaper when membership is the
dense polynomial than when it is the tower? Targets with a decomposition (planted: a sum of `m` factor-base points)
are the case that matters; random targets (almost always no decomposition) are kept as the refutation control.

**Arms.** `tower`: the degree-2 Kummer tower (slide 17's system, `algebraic_decompose`). `dense-coset`: the same coset
`V`, membership `∏(x − v)` (`algebraic_decompose_dense`; its `field_ops` include expanding the product,
`|V|(|V|+1)/2`). Tower and dense-coset run on *identical* targets in one row (paired). `dense-random`: `|V|` random
elements of `F_p`, its own planted targets (unpaired with the tower; reported against the same `S`). Every arm is
also compared with counted enumeration over `V` (`enumerate_decompositions`) on the same target.

**Ladder.** `p = P1 = 1399680001`, curve and targets from seed `20261114` (as EXP-PKM-ODD-001 part B), 4 planted and 2
random targets per cell. `m = 2`: `S = 2^4 … 2^8`, `both` and `dense-random`. `m = 3`: `S = 2^2 … 2^4`, `both` and
`dense-random`; `S = 2^5` dense arms only (the tower cell costs ≈ 5·10¹² and was censored for memory on this host in
EXP-PKM-ODD-001). Replication at `P2 = 3265920001`: `m = 2`, `S = 2^6` and `m = 3`, `S = 2^3`, `both`. Degree bound
1024 for every arm (the dense system needs degree `> S`; the tower's solving degree stays far below either bound),
budget 1,800 s per system, `RAYON_NUM_THREADS=4`, one binary (SHA-256 in `binary.sha256`).

**Validity (must hold before any verdict).** V1: on every decided row, each arm's relation set equals enumeration's.
V2: every planted target has at least one relation. V3 (A/A against committed data): the tower arm's counts at `P1`
equal EXP-PKM-ODD-001 part B's on the cells both ran (`B-M2-A2-t4/t6/t8`, `B-M3-A2-t2/t3/t4`: same seed, targets and
curve); a difference means the refactor changed the instrument.

**Pre-registered predictions.**

| # | Claim | Prediction | Falsified by |
|:--|:--|:--|:--|
| P1 | dense-coset ÷ tower on planted targets (paired), per completed `S` | median `< 1` at every completed `S` (the dense form also wins at finding) | any completed `S` with median `≥ 1` |
| P2 | the same ratio | `< 0.5` at every completed `S` (a material win) | any completed `S` with median `≥ 0.5` |
| P3 | dense-random ÷ dense-coset on planted targets | within `(0.5, 2)` at every `S` (structure of `V` does not matter) | outside the band at any `S` |
| P4 | dense-coset ÷ enumeration on planted targets | `> 1` at every `S` (the algebraic oracle still loses to enumeration) | any `S` with median `≤ 1` |

**Rules.** A system that times out is censored and reported as a lower bound, never imputed. Every prediction's
verdict is reported. No ladder, budget or band changes after a row is seen without a dated amendment. The instrument
change (schema `pkm-algebraic-oracle.row/v2`: tower and dense results side by side) is in the same commit as this
protocol; the library refactor (`s3_chain_polys`, `algebraic_decompose_system`) passes the existing unit test that the
tower oracle returns exactly the enumerated relations, and a new one for the dense oracle. Nothing here concerns
cryptographic sizes or Pollard rho [pcc:unverified].

**Execution.** Local (4 vCPU, 15 GiB), as the two earlier studies: no EKS route here; checkpoint opt-out
`CHECKPOINT_SKIP=no-s3-credentials` recorded in the pcc ledger. Cells needing more memory are censored and listed.

## Amendments

None at freeze. No row of this study, smoke or otherwise, had been seen at freeze.

### Amendment 1 (2026-11-15, after the frozen run): larger `S` on RunPod

Requested by the user after the frozen run. A separate, labelled extension; the frozen rows and verdicts stand. Rows in
`runs/large/`, never mixed with `runs/`. Same instrument and binary family (SHA-256 in `runs/large/binary.sha256`), seed
`20261114`, degree bound `2^20`, same verdict rules. Host: one RunPod CPU pod (16 vCPU, 128 GB), four lanes of 4 threads,
peak RSS recorded per cell. No EKS route from this session, as before.

Ladder:
- `P1`, `m = 2`, `S = 2^9, 2^10`: `both` (tower and dense on the same targets) and `dense-random`, 4 planted + 2 random,
  budget 7,200 s (the tower at `2^10` is estimated near 10¹² multiplications).
- `P3 = 3221225473` (`= 3·2^30 + 1`, prime; the only change of prime, needed for `2^k | p − 1` beyond `2^10`):
  `m = 2`, `S = 2^8` (`both`, linking the two primes) and `S = 2^11 … 2^15` dense-coset and dense-random, 4 planted + 2
  random, budget 1,800 s.
- `P1`, `m = 3`: `S = 2^6` dense-coset and dense-random, 4 planted + 2 random, budget 7,200 s; `S = 2^7` dense-coset,
  1 planted target, budget 86,400 s.

Predictions, frozen with this amendment before any row:
- L1: dense ÷ tower `< 0.5` at `m = 2`, `S = 2^9, 2^10`.
- L2: dense ÷ enumeration `> 1` at every completed `S`.
- L3: the dense oracle's growth exponent, fitted on planted medians, lies in `[1.8, 2.3]` at `m = 2` (`S = 2^8 → 2^15`,
  `P3`) and in `[4.5, 6.0]` at `m = 3` (`S = 2^5 → 2^7`, the `2^5` point from the frozen run).
- L4: dense-random ÷ dense-coset in `(0.5, 2)` at every `S ≥ 2^6`.
- Validity as before: every decided row agrees with enumeration; every planted target has a relation; the `P3`, `S = 2^8`
  dense count is reported beside `P1`'s.

A cell that hits its budget or the pod's memory is censored and reported as a lower bound.
