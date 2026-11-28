# p31 Mersenne calibration row (scaffold)

Frozen ordinary cofactor-one curve for the Fp IC exponent-lead gate. Source
of truth: external workdir `prime-fp-subexp-ic-p31mersenne-20261121/CURVE.json`;
committed excerpt: [`CURVE.json`](CURVE.json).

## Table identity

| Field | Value |
|:--|:--|
| Calibration key | `prime/generated-31bit-2147483647` |
| ICV1 slug | `icv1-fp31-tm3989-ae9db972` |
| Legacy alias | `generated-31bit-2147483647` |
| `ic bench` | `--bits 31 --family p31-subexp` |

Boundary native units in [`calibration.json`](../../calibration.json) for this
row are **scaffold copies** of `prime/generated-24bit-10935329` until isolab
re-pin via `ic boundary` + `boundary_pin_calibration.py`. Do not use this row for
`fits.exponent_moved` claims until boundary and solver-unit keys are pinned.

## Pin sequence (isolab-eks, Intel Xeon class)

1. **Boundary ratios** — `ic boundary` on the p31 instance; merge with
   `docs/ic/tools/boundary_pin_calibration.py`.
2. **`ns_per_fp_mul`** — `cargo run --release --example fp_mul_calibration_microbench -- 31`
   (or roster bits); 400 000 dependent `mulm` per rep, seeds `1..5`, median ratio.
3. **`ns_per_hashed_tensor_step`** — after contraction timer lands in CI (#4903),
   extend `docs/ic/tools/run_interleaved_solver_pin.sh` with:

   ```bash
   ic bench --json --bits 31 --family p31-subexp \
     --factor-base hashed-affine --oracle hashed-tensor \
     --targets walk --max-trials 50000 --repeats 1 --seed "$seed"
   ```

   then `solver_unit_pin_calibration.py --key ns_per_hashed_tensor_step \
     --instance prime/generated-31bit-2147483647`.

4. **Classical arms (constants only until (2–3) land)** — same workloads as
   smaller primes: `m4gb-f2`, `inherited-f4`, `sat-cdcl` on matched cells.

## p31 gate

Run priced `ic.tensor_contraction_v1` ecbench only after step (3). Require
`fits.exponent_moved == true` with no unpriced contraction counters.