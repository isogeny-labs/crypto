# Pre-test protocol: block Lanczos vs coordinate Wiedemann on the genus-4 x* core (lead #2)

Frozen before any matrix is exported or any solver is run. Lead and referee pre-test:
`research/hec_breakthrough_search_20261120/REPORT.md`, rank #2. Every number this study produces
is `[pcc:unverified]`.

## 1. Question

Does replacing the coordinate Wiedemann solve of `genus_ladder` arm x\* (SGE, then
`wiedemann_coord(square = true)` on a random square row subset of the reduced core,
`src/cryptanalysis/chained_transfer/ic_control.rs`) with block Lanczos over F_r cut the counted
linear-algebra work by at least 20%?

## 2. Unit (both arms identical)

The lab's unit for linear algebra is the counted mod-r operation: `Nat { mul, add }` of
`ic_control.rs`, summed into `la_mul` / `la_add`; the w = 1 cost charges each as 1.

- Sparse product, both arms, exactly as `mat_vec` / `mat_t_vec` count it: per stored non-zero one
  `add`, plus one `mul` when the entry is not `+-1`. Diagonal scaling `D`: one `mul` per entry.
- **Lane-SpMV**: one product of the sparse matrix (or its transpose) with **one** vector. A block of
  `b` vectors is `b` lane-SpMVs, because over F_r (r about 2^56) each lane costs its own mod-r
  operations per non-zero. A "block pass" (one sweep over the matrix for a whole block) is reported
  for information only; it measures memory traffic and parallelism, not counted work.
- Dense block work of block Lanczos is charged in the same unit: every `b x b` Gram product over
  `N` rows `b^2 N` mul + `b^2 N` add; every block axpy with a `b x b` coefficient `b^2 N` mul +
  `b^2 N` add; small `b x b` products `b^3` mul + `b^3` add; a `b x b` inverse by Gauss-Jordan,
  each elimination multiply-subtract one mul + one add, each pivot inverse `pivot_inversion_steps(r)`
  muls (the binary's charge for an inversion). Only the target coordinate of the solution is formed
  (as `wiedemann_coord` does), charged `b` mul + `b` add per iteration.
- Wiedemann arm: the binary's own accounting, re-implemented line for line (`mat_vec` per step,
  `berlekamp_massey`, the final coordinate sum and inversion), and checked against the binary's
  recorded `la_wied_mul` / `la_wied_add` for the same exported run.

## 3. Arms

- **W (baseline)**: coordinate Wiedemann, square subset, sequence length `2N + 2`, as in the binary.
- **L_b**: block Lanczos / block conjugate-direction on `A'^T D A'` (the same square subset `A'`,
  random diagonal `D`), the recurrence of the lab's `koblitz_sparse_la::block_lanczos_solve`
  (first lane the right-hand side, other lanes random), for `b in {1, 2, 4, 8}`. A second variant
  `L_b^rect` on the whole reduced (rectangular) core `A^T D A` is reported for information.

## 4. Instances

- **Main**: one p = 16411 genus-4 instance at the x\* configuration (cc 1.4, SGE 4,1.5,
  `--threads 1`) from curve seed 373 of `research/genus4_prime_order_20261118` (a confirmation
  curve), its reduced core exported by a flag-gated (environment variable) dump added to
  `genus_ladder`, binary built to the scratchpad. If the export fails, a synthetic matrix with the
  recorded dimension, density and row-weight distribution is used instead and the result says so.
- **Toy (planted-log check)**: 3 smaller certified `genus_ladder` instances (genus 3 or 4, small p),
  exported the same way; the planted logarithm is the binary's certified answer `t` (`[t]G = T`
  checked in the group). Each arm must return `x[ct] = t` (after the unit-T scaling the binary
  applies, which the export records). The main instance is also checked.

## 5. Metric and kill rule (frozen, from the referee)

- Primary: `rho_spmv = (lane-SpMVs of L_b) / (lane-SpMVs of W)`, for the best `b`.
- Charged: `rho_ops = (mul + add of L_b) / (mul + add of W)`, all counted work of the solve.
- **KILL** if `rho_spmv >= 0.80` for every `b` tested (less than a 20% mat-vec reduction), **or** if
  block Lanczos fails to recover the planted logarithm on all 3 of 3 toy instances. A `b` that
  passes the SpMV gate but has `rho_ops >= 0.80` is reported as failing on charged cost.
- **Pass** otherwise; the implied change to the x\* row is then a projection, computed as
  `new/old = 1 - s_LA (1 - rho_ops)` with `s_LA` the recorded LA share of the w = 1 cost
  (0.56-0.60 at p = 16411), and labelled a projection, not a measurement. The same projection is
  reported on a KILL for completeness.

## 6. Prediction (written before data)

Over a large prime field a block of `b` vectors costs `b` times the counted work of one vector, so
block Lanczos needs about `N/b` iterations x `b` lanes x 2 products (`A'`, `A'^T`) = `2N` lane-SpMVs,
the same as the `2N + 2` of coordinate Wiedemann. Expected `rho_spmv` near 1.0 and `rho_ops` above 1
(dense block work exceeds Berlekamp-Massey's `~4N^2`). Expected verdict: KILL.

## 7. Limits

Single thread, under about 60 minutes of compute, pure Python solvers (exact counts, not timings).
No timing claim. Does not touch `research/genus_ladder_scaling_20261119`.
