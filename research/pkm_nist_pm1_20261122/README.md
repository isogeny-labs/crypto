# PKM tower coverage at NIST and 59-bit primes

Date: 2026-11-22. Status: local stage diagnostic; empirical statements here are `[pcc:unverified]`.

The wide PKM pilot now selects the **named NIST curve** with `--curve nist`, using the curve parameters in [`CurveParams`](../../src/ecc/curve_zoo.rs), or its existing random curve mode. The named fields follow [NIST SP 800-186](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf); P-192 is retained as a legacy case. The Kummer factor base uses `2^t | p-1`; the Dickson norm-one torus uses `2^t | p+1`. A quadratic nonresidue defines `F_(p²)` when `-1` is a residue, allowing P-224's depth-1 torus case. When `-1` is a nonresidue, the prior representation is preserved.

| Field / curve | `v₂(p-1)` | `v₂(p+1)` | Kummer cell | Torus cell |
| --- | ---: | ---: | ---: | ---: |
| NIST P-192 | 1 | 64 | `t=1` | `t=2` |
| NIST P-224 | 96 | 1 | `t=2` | `t=1` |
| NIST P-256 | 1 | 96 | `t=1` | `t=2` |
| NIST P-384 | 1 | 32 | `t=1` | `t=2` |
| NIST P-521 | 1 | 521 | `t=1` | `t=2` |
| 59-bit `288230376154267649` | 16 | 1 | `t=2` | `t=1` |
| 59-bit `288230376151973887` | 1 | 18 | `t=1` | `t=2` |
| Existing 59-bit `288230445959117953` | 7 | 1 | `t=2` | — |

Run with `bash research/pkm_nist_pm1_20261122/run.sh target/release/examples/pkm_tower_pilot` after building the example. [`run.sh`](run.sh) fixes `m=2`, one planted and one random target, the tower control, a 15-second per-solve budget, a one-million-nonzero cap, and `--verify`. Each of the 15 cells has its exact command parameters and two raw rows in [`results/`](results/). The rows record the field prime, curve coefficients, valuations, target, solver cost, and verification checks.

The run completed all 30 rows without a solver stop. All 30 `verify.ok` fields are true; the verifier checked 45 points with `BigUint` evaluation of the input equations and returned basis, compared the solver's inconsistency verdict with direct factor-base enumeration, and checked the staircase bound. Both named-curve and 59-bit controls are included. These are bounded `m=2`, `t≤2` solver checks; the tower construction tests additionally exercise depth 8 at P-192, P-224, and P-256. They do not price a relation pipeline or establish behavior at larger `m` and `t`.

Focused tests: `cargo test --release --locked --example pkm_tower_pilot towers_check` passed two tests. The `wide::tests::nine_limb_field_matches_biguint` test passed separately on P-521. Rust 1.99.1 was used because the default Rust 1.93.1 cannot build the current AWS dependencies. Rust emitted an `objcopy` missing-`libLLVM.dylib` warning after compilation, but both test and runnable binaries executed.
