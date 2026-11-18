# Binary (GF(2^n)) Koblitz fall-degree study

Slug `binary_falldeg_20261118`. The GF(2^n) analogue of the prime-field
last-fall-degree work, extending the binary program
`research/fall_degree_bounds_20261008/` with a **Frobenius-orbit** factor
base. Protocol is frozen in [`PREREGISTRATION.md`](PREREGISTRATION.md);
results and the honest limits are in [`RESULTS.md`](RESULTS.md); the pod
ladder is in [`POD_PLAN.md`](POD_PLAN.md).

All numbers here are `[pcc:unverified]` pilot/calibration data taken on the
local 4-vCPU container, not study rows.

## Question

Does the Frobenius-orbit structure of the summation-system factor base
change the binary fall degree (first-fall `d_ff`, last-fall `d_F`) versus a
random factor base of the same dimension — and versus a structured
non-orbit base?

## Instrument

`examples/binary_falldeg_arms.rs` (built from the repo root):

```
export CARGO_TARGET_DIR=$PWD/target
cargo build --release --example binary_falldeg_arms
cargo test  --release --example binary_falldeg_arms      # gate G1
target/release/examples/binary_falldeg_arms selftest     # gates G2, G4, ordering
target/release/examples/binary_falldeg_arms run \
    --cells 2:13:5,2:15:7,3:15:5 --seed 1262698074 \
    --keep 3 --cmax 4 --ffd-max 6 --dres-max 6 \
    --arms orbit,random,subspace --crosscheck >> raw/pilot.jsonl
```

It reuses, unchanged: `koblitz_groebner::{build_decomposition_system,
first_fall_degree, solving_degree}`,
`koblitz_bench::{random_subspace_basis, chained_solution_count_fast}`, and
the exact HKY mutant closure of `examples/hky_par.rs` (copied verbatim, so
the `random` arm reproduces `hky_par`'s `d_F`).

## Three arms (factor base `V`, matched dimension `ℓ_eff`)

- `orbit`    — squaring-closed (`τ`-invariant) `F_2`-subspace (Frobenius orbit base)
- `random`   — uniform `F_2`-subspace of the same dimension (the null)
- `subspace` — polynomial window `span{1,x,…,x^{ℓ_eff−1}}` (structured, non-orbit)

## Key facts established at registration

- The binary `S_3` here is `x_1²x_2² + x_1²x_3² + x_2²x_3² + x_1x_2x_3 + b`,
  **`a_2`-independent**: Koblitz `a=0` and `a=1` give the identical
  summation system. See PREREGISTRATION §2.
- `d_F` (HKY last fall) is **natively well-defined** over `GF(2^n)` (the Weil
  descent lands in the Boolean ring with a square-free normal form); no PKM
  tower is needed, unlike the prime field. Its **computability** is the
  binding limit (`~C(n_vars, ≤c)` columns). PREREGISTRATION §4, §4a.
- A `τ`-invariant subspace has dimension = a sum of 2-cyclotomic coset sizes
  mod `n`, so the Frobenius-orbit base is **dimension-constrained**;
  the orbit arm fixes `ℓ_eff` and the other arms match it. PREREGISTRATION §3.

## Layout

- `PREREGISTRATION.md` — frozen protocol.
- `RESULTS.md` — pilot readings, the A/A floor, honest limits.
- `POD_PLAN.md` — the cells worth a bigger host and the frozen command.
- `raw/` — JSONL rows and selftest logs.
