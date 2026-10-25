# Z[tau] multiplier base in the production Koblitz producer (lead A1, wired)

**Date.** 2026-10-23. **Status.** Measured on the `u64` path (n <= 63) and the wide path (71 <= n <= 127 with a u64 subgroup order), `a = 0` only.
Source: workflow `ic-leads` (lead 1 of the ranked shortlist), first step of its
staged experiment.

## What changed

`examples/koblitz_orbit_dlp_fast_online.rs` gains `KIC_NORM=N` for `construct:`
bases. For each seed `R_j` it also takes `alpha(R_j) = [a]R_j + [b]phi(R_j)` for
every `alpha = a + b*tau` with `a^2 - ab + 2b^2 <= N`; each image's signed
Frobenius orbit joins `R_j`'s column with weight `(a + b*lambda) * lambda^t`.
The root index gets one representative per orbit, the rank stage still needs
only K+1 relations (K = seeds). The default (`KIC_NORM` unset or 0) is the
previous base and index, unchanged. With `KIC_NORM > 0` the free-relation
harvest defaults off (23,280 of 23,316 collisions at n=41 were trivial
Z[tau] identities inside one column; explicit `KIC_FREE_RELATIONS=1` still
works). The harvest also stops once the echelon has full rank.

Unit test: `norm_base_tests` (every point is `[coef]` times its column seed).

## Measurements (child CPU seconds, complete process, all targets recovered)

Machine was loaded (load average ~20), so absolute numbers are inflated;
ratios are within one scan. 3 targets, seeds 1..N shown in `scan_logs.txt`.

| n | base | points | child CPU | vs tuned plain |
|---:|---|---:|---:|---:|
| 41 | K=125 plain | 10,250 | 0.478 | 1.0x |
| 41 | K=12 norm 16 | 9,840 | 0.254 | 1.9x |
| 53 | K=280 plain (tuned) | 29,680 | 5.13 | 1.0x |
| 53 | K=28 norm 16 (equal points) | 29,680 | 2.25 | 2.3x |
| 53 | K=20 norm 16 | 21,200 | 1.20 | 4.3x |
| 53 | K=10 norm 32 | 21,200 | 1.36 | 3.8x |
| 61 | K=500 plain (tuned) | 61,000 | 19.0 | 1.0x |
| 61 | K=50 norm 16 (equal points) | 61,000 | 8.0 | 2.4x |
| 61 | K=18 norm 32 | 43,920 | 4.30 | 4.4x |
| 61 | K=28 norm 16 | 34,160 | 4.71 | 4.0x |

Mechanism check: at equal points the root index is the same size and only the
rank stage shrinks (27.3 s -> 3.0 s at n=61). The further 2x comes from
retuning: with K+1 relations needed, the optimum moves to a smaller table.

## Wide path (n = 71, 73), same machine, `construct:` bases, 1 target, seed 1

`KIC_NORM` is implemented in the wide module too (`construct_base128_norm`).
Child CPU is the cleaner number; the machine was loaded (wall is inflated).

| n | base | points | child CPU s | wall s | rank stage s | target recovered |
|---:|---|---:|---:|---:|---:|---|
| 71 | K=600 plain (control, same machine) | 85,200 | 580.8 | 1,113 | 1,043 | yes |
| 71 | K=60 norm 16 (equal points) | 85,200 | 93.5 | 209 | 128 | yes |
| 71 | K=20 norm 32 | 56,800 | 55.4 | 142 | 101 | yes |
| 73 | K=20 norm 32 | 58,400 | 1,119 | 1,333 | 1,131 | yes |

n=71: 6.2x (equal points) and 10.5x (retuned) in child CPU against the plain
control. n=73: no same-machine plain control was run; the recorded plain K=600
rank precompute was about 18,400 s (`RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md`
lineage / n73 ledger), so this is roughly 16x on the rank stage, but that
comparison is across runs and machines. K and norm were not tuned at n=73.
n=83 with `a = 0` is not reachable here: its prime subgroup order does not fit
in `u64` (`to_u64().unwrap()` at construction), independent of `KIC_NORM`.
Only the K=600 sparse/dense n=83 arms from header files exist, and they would
need a header-load path for orbit representatives, which `KIC_DUMP_BASE` does
not record.

## What this is not

* A constant factor against the repo's own K-tuned compact-orbit pipeline, not
  against published index calculus. FGHR / GGMP have no table-based
  counterpart at these (q, n, m), and the closure needs Z[tau], so it is
  Koblitz-only. The exponent is unchanged (CGK bound `S*T^2 >= r/|Aut|`).
* Not measured: n = 83, paired complete-cold child CPU as in
  `research/notes/ecc2k130/disjoint_cold_v2_outcome_20261001/RESULT.md`, and
  the ratio to strong rho (the incumbent was 13-23x at n = 53..61; expect
  about 3-5x of that to be recovered, not more).
* Degeneration: as K' -> 1 this becomes a baby-step/giant-step table over
  Z[tau]-multiples. The gain is the pipeline closing part of its distance to
  that generic bound, not passing it. Prime-field and non-Koblitz binary
  curves have no free endomorphism to use; the prime analogue (integer
  multiples) was priced in the lead workflow at its BSGS optimum (about 0.825x
  rho) and not implemented.
* Free-relation harvest adds almost nothing on this base (rank 5 from 23k
  collisions at n=41).

## Reproduce

    cargo build --release --example koblitz_orbit_dlp_fast_online
    python3 scan.py 53 3 280,0 20,16     # K,norm pairs; edit X path in run.py
