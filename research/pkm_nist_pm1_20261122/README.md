# PKM tower coverage at NIST and 59-bit primes

Date: 2026-11-22. Status: local stage diagnostic; empirical statements here are `[pcc:unverified]`.

The wide PKM pilot now selects the **named NIST curve** with `--curve nist`, using the curve parameters in [`CurveParams`](../../src/ecc/curve_zoo.rs), or its existing random curve mode. The named fields follow [NIST SP 800-186](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf); P-192 is retained as a legacy case. The Kummer factor base uses `2^t | p-1`; the Dickson norm-one torus uses `2^t | p+1`. A quadratic nonresidue defines `F_(p²)` when `-1` is a residue, allowing P-224's depth-1 torus case. When `-1` is a nonresidue, the prior representation is preserved.

## Maps selected and relation to the PKM paper

The [Petit–Kosters–Messeng paper](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf), §§3.1–3.4, uses a high-degree map `L` decomposed into low-degree maps `L_j`. Its two concrete instances are a subgroup of `F_p*` when `p-1` has a suitable smooth divisor, and a coset of a smooth subgroup on an **auxiliary elliptic curve**, mapped through a chain of small-degree isogenies. The latter uses rational x-maps `L_j(x)=ξ_j(x)/ψ_j(x)^2` and a final target `χ` on the image curve.

This run selected these maps:

| Arm | Low-degree step on each tower coordinate | Composed high-degree membership map | PKM paper status |
| --- | --- | --- | --- |
| Kummer (`p-1`) | `y ↦ y²`, starting from `y₀=x/g` | `L(x)=(x/g)^(2^t)-1`, degree `2^t`; roots `g·μ_(2^t)` | An exact small coset instance of the paper's §3.3 subgroup family |
| Dickson torus (`p+1`) | `y ↦ y²-2` | `L(x)=D_(2^t)(x)-c`, degree `2^t`, with `D₂(y)=y²-2` | A separate norm-one torus family; **not** the paper's §3.4 auxiliary-curve isogeny construction |

No auxiliary curve `E′`, smooth subgroup `H⊂E′(F_p)`, isogeny kernel, Vélu x-map, or image target `χ` was selected for these runs. The solver targets were one planted sum and one random point per cell, rather than the paper's relation stream `aP+bQ`; relation collection and final linear algebra were not run. The builder explicitly enumerates all `2^t` base abscissae, so the `t=1,2` solver cells do not test the large high-degree maps needed to obtain the paper's intended relation density. For example, its heuristic condition `deg(L)^m≈m!p` calls for roughly a 96-bit degree at `m=2` over P-192, while the tested torus map has degree 4. All 15 random targets had zero decompositions; all 15 planted targets had two ordered decompositions. This is a check of the small systems' construction and solver verdicts, not an evaluation of the paper's attack.

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
