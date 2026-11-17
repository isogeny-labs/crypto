# Strong rho, round 4 (batched inversion): protocol

Parent: `412b60289` (`origin/main`, 2026-10-22). Written on 2026-10-22,
**after** the Callgrind profile and instruction count below and **before** any
native timing.

## Where the instructions are

A Callgrind profile (`--dump-instr=yes`, line-table build) of the parent at
n53, 4 fixtures (`koblitz_rho_fixture 53 0 signed_frobenius 4 strong`),
counts 2,145,944,068 instructions for 3,034,474 walk steps (707 per step):
- `Gf2::batch_inv_clmul` at 13.6% inclusive, 107,497 calls averaging 28
  elements, so about 90 instructions per element. Its single inversion is
  23.7 of those, and the `chunks().zip().zip()` iterator adapters are about
  18 (`core::iter::adapters::zip`, 57.8M).
- Normal-basis conversion in `canonicalize` (`Linear::apply_pair`) at about
  20%. It was already unrolled in round 3 and is not changed here.

## Change (no value changes)

`Gf2::batch_inv_clmul` walks `xs` and `scratch` as `as_chunks::<4>()`
arrays, with the lane as a constant index, and handles the short tail
separately: last in the forward pass, first in the backward pass. The same
products are formed. Lane assignment cannot change a result, because every
inverse is unique.

## Identity gate

- Every output line of `koblitz_rho_fixture n 0 signed_frobenius c strong`
  is identical, timing and RSS aside: n = 41 and 53 with c = 16, n = 61 with
  c = 4.
- The `batch_inv` unit tests pass.

## Measurements

- **Primary: Callgrind instructions per walk step** at n = 41 and 53, with 4
  fixtures. Already measured when this was written:
  - n53: 2,145,944,102 → 2,062,553,397;
  - n41: 244,684,552 → 235,163,781.
- **Native time:** declared now and not yet run. Round 3's `run_time.sh`
  protocol, unchanged: A/A (baseline × 6), then 5 interleaved A/B rounds at
  n = 53 and 61, every process under `tools/isolated_bench.py --cpus 3`.
- A native effect is called established only when it lies outside the A/A
  spread. Otherwise it is reported and not claimed.
- IPC is not measured: the host has no hardware PMU.
