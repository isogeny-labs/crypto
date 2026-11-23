# Prime-field EC arithmetic (`src/ecc`) for the `field_ec` kernels: WALL-only

Slug `crypto-ecbench-dlp-r2`, branch `perf/crypto-ecbench-dlp-r2`, base
`761b5ac3b5` (`claude/performance-optimization-parallel-vgenii`). Rules:
AGENTS.md section 10 and the audits
[`20261110-wall-only-rust-fixes.md`](20261110-wall-only-rust-fixes.md),
[`20261112-rust-deep-audit.md`](20261112-rust-deep-audit.md): every value,
counted unit, RNG draw and output is unchanged. No Python implementation of
any algorithm; no proof-pinned file touched.

All wall figures are **local, unisolated, [pcc:unverified]** (shared 4-vCPU
VM, load average about 1.2-1.9 during the A/B runs, `RAYON_NUM_THREADS=1`).
Callgrind instruction counts (`perfbench run --instr`, `--instr-atstart=no`,
valgrind 3.22) are deterministic but not filed as pcc claims:
**[pcc:unverified]**.

## Scope, and what was out of reach

Files edited: `src/ecc/{point,field,mod,p256_field,secp256k1_field}.rs` and
the new `src/ecc/mont_vt.rs`. `src/cryptanalysis/ecbench/methods.rs` holds
no hot loop (registry and report plumbing); the rho/kangaroo/BSGS inner
loops the audit profiled live in `ic_boundary.rs` (`TunedWalker`,
`add_raw`) and `ecbench/generic.rs`, outside this scope, so they are listed
under "Deferred" below, not touched. The `dlp/*` perfbench kernels call
`cryptanalysis/*` code outside the scope; none of them changed fingerprint
or code path here.

Profiling the in-scope kernels first (base binary, Callgrind): the
`Point::scalar_mul` Jacobian ladder and `FieldElement::{mul,pow,inv}` ran on
`num-bigint` (`div_rem_core`, `mac3`, malloc/free about 75-85% of Ir); the
constant-time `inv`/`pow` was a 3-multiplications-per-bit `BigUint`
ladder; `ct_bignum::Uint::mont_mul` was 82% of the fixed-width ladders.

## Binaries

`cargo build --release --example perfbench -j2` at each commit
(`CARGO_TARGET_DIR` outside the tree), rustc 1.97.0:

| label | tree | sha256 |
|---|---|---|
| base | `761b5ac3b5` | `46d3f1ce4436e4c8cfbd75918d6a94ca0050387090bf566683ca69ebee3f73b0` |
| c1 | commits 1+2 | `41b19fcb4c64cd538ac85c2aaaad2a14f1c62749f84e816ec568093023b0f7c9` |
| c3 | commits 1-3 | `2f95da17ea63b05bf41a52ceecd137f6e2f2ae43064b832ee6c81f558057593a` |
| c4 | commits 1-4 | `e8220cd198593a113f3936fb1f8a4d6b881cd9a5186886c7fe251c095c279289` |

## Commits

1. `Point::scalar_mul` on fixed-width Montgomery words (`mont_vt.rs`): the
   same Jacobian formulas (dbl-2007-bl, madd-2007-bl), branch for branch
   (`Y = 0`, `H = 0`, `R = 0` tests on canonical values), N-word CIOS
   (N = 1, 2, 4, 9 by modulus width, odd modulus 3..2^576), final
   inversion `Z^(p-2)` as `FieldElement::inv` computes it. It declines (and
   the `BigUint` ladder runs, panics included) for an even or tiny
   modulus, an operand that is not a reduced residue of the same modulus,
   or a zero final `Z`. Valid for composite odd moduli too: ring
   operations only.
2. `FieldElement::pow` for an odd modulus of exactly 256 bits and an
   exponent of at most 256 bits (the range `mod_pow_ct` reads): the
   branch-free 4-limb Montgomery ladder, context cached per thread.
3. `ct_mont_mul4`: one interleaved (CIOS) constant-time product with a
   mask-selected final subtraction, equal to `Uint::mont_mul` on every
   pair of residues; used by `p256_field`, `secp256k1_field` and the
   256-bit `pow` ladder (`ct_pow4`).
4. `FieldElement::inv`: Fermat's exponent `p - 2` is public, so a fixed
   4-bit window replaces the always-multiply ladder on 256-bit odd
   moduli (operation sequence follows `p - 2`, not the base). `pow` keeps
   the ladder for caller exponents.

Callgrind Ir (before is the base binary; c1 is commits 1+2 together, they
were not built apart):

| kernel | base | c1 | c3 | c4 (final) | fingerprint base = c4 |
|---|---:|---:|---:|---:|---|
| `point_scalar_mul_p256_x8` | 165,249,830 | 9,481,005 | 9,481,005 | 9,481,005 (-94.3%) | `31a2796c744000cf` |
| `point_scalar_mul_secp256k1_x8` | 159,392,752 | 9,697,781 | 9,697,781 | 9,697,781 (-93.9%) | `d76097d0c514b5f6` |
| `ecdsa_verify_p256_x4` | 546,043,152 | 35,339,857 | 34,567,425 | 34,418,361 (-93.7%) | `bb46bfe51281521b` |
| `ecdsa_verify_secp256k1_x4` | 540,621,581 | 36,363,002 | 35,863,538 | 35,513,650 (-93.4%) | `bb46bfe51281521b` |
| `point_scalar_mul_ct_secp256k1_x1` | 1,377,040,770 | 176,552,030 | 128,479,390 | 94,802,670 (-93.1%) | `2ac5acbe1caf5e14` |
| `ct_scalar_mul_p256_x16` | 103,430,171 | 96,110,587 | 70,337,803 | 70,337,803 (-32.0%) | `9ba17c08fbf800fd` |
| `ct_scalar_mul_secp256k1_x16` | 70,449,387 | n/m | 55,937,979 | 55,937,979 (-20.6%) | `0d6759ad1f0068e6` |

Wall, local, unisolated, [pcc:unverified]: median of 5 samples per run,
three A/B alternations (ms), load average 1.26 before / 1.22 after:

| kernel | base | c4 |
|---|---|---|
| `point_scalar_mul_p256_x8` | 11.5, 12.6, 11.1 | 1.02, 1.17, 1.10 |
| `ecdsa_verify_p256_x4` | 37.9, 36.1, 39.2 | 3.97, 3.87, 3.61 |
| `point_scalar_mul_ct_secp256k1_x1` | 102.5, 107.6, 107.5 | 7.5, 8.4, 10.1 |
| `ct_scalar_mul_p256_x16` | 10.6, 9.4, 8.5 | 5.65, 5.43, 5.15 |

## Identity

- `perfbench run --filter field_ec/,dlp/ --full --samples 1`, base vs c4:
  37 kernels, id and fingerprint lines identical (`diff` empty;
  `ident.sh`, one thread).
- Fingerprints of the seven kernels above equal between base and the
  candidate in every row of the table (the harness fingerprints every
  point computed).
- Unit tests added (all pass): `ecc::point::fixed_width_differential`
  (fast ladder against the `BigUint` ladder over 20 odd moduli of widths
  2 to 577 bits, prime and composite, edge/random off-curve points,
  edge scalars, panic parity; decline cases),
  `ecc::point::pow_256_differential` (`pow` and `inv` against
  `mod_pow_ct`, unreduced bases, over-long exponents, even and 255-bit
  moduli declined), `ecc::mont_vt::tests` (`ct_mont_mul4` against
  `Uint::mont_mul`, `ct_pow4` against `mont_pow_ct`,
  `ct_pow4_public` against `ct_pow4`, the generic product/add/sub against
  `BigUint` for N = 1, 2, 4, 9).

## Tests

`cargo test --release --lib ecc::` on the final tree: 335 passed, 0
failed, 2 ignored (the filter also matches `binary_ecc::`). Other
modules that call `Point`/`FieldElement`, by test-name filter on the same
binary: `ecdlp_variants` 15, `pollard` 66, `schnorr` 15, `elgamal` 18,
`aut_folded` 11, `gaudry` 58, `kangaroo` 7, `prime_ecc` 13: all pass. The
full `--lib` run (about 6,400 tests) was stopped after about 2,300 passed;
the 24 failures seen are in `certicom_dlp::device_tests` (needs `gpu/` and a
C++ compiler on the tracked source), `ecbench::{band,bounds,challenge,claim,
compare,exact_scalar,koblitz_descent}` (read committed `research/`
session files): files this sparse worktree does not check out, not the code
under change (checked: the panics are `must be materialized` and
`read_records(...).unwrap()` on missing directories). `rustc` warnings in the new code:
none after the `t_n1` fix. Not run: `cargo clippy`, `cargo test --test`
(disk and time), the CI debug-profile gate.

## Reverted ideas

None reverted; one thing was not built: a windowed/NAF variable-time
ladder (see Deferred).

## Deferred method changes (would change the operation sequence or leave scope)

- **Windowed / wNAF `scalar_mul`**: fewer additions, but the
  exceptional-case sequence (`H = 0` inside the ladder) differs for
  off-curve, small-order or composite inputs, where the `BigUint` ladder's
  output is part of the existing tests; needs its own id or a restriction
  to on-curve prime-order inputs.
- **Dedicated Montgomery squaring** (about 10 of 16 limb products) and a
  special-prime reduction for secp256k1: the field representation is
  Montgomery form in public constants; changing it is not WALL-neutral for
  anything that reads `SecpFieldElement`'s internals.
- **`ic_boundary.rs` (`TunedWalker`, `PrimeCurve::add_raw`, batched
  inversion), `ecbench/generic.rs` (`kangaroo_with`, BSGS), `prime_fast`
  `rho_parallel` DP store**: the audit's open threads
  (`rho.negation_lanes_v1` lane bookkeeping and Montgomery-form lanes) are
  in files outside this task's scope.
- `Point::add_vartime`/`double_vartime` above one word (`BigUint::modinv`
  per step) could use `Mont<N>` plus a batched inversion; the walks that
  call them live outside the scope.

## Reproduce

```bash
cargo build --release --example perfbench -j2        # at each commit
valgrind --tool=callgrind --instr-atstart=no --callgrind-out-file=/dev/null \
  perfbench run --filter field_ec/point_scalar_mul_p256_x8 --exact --instr --samples 1
perfbench run --filter field_ec/,dlp/ --full --samples 1     # fingerprints
```

## Independent review (skeptical re-check, 2026-11-13)

[pcc:unverified] Rebuilt base `761b5ac3b5` and the branch head from clean
checkouts (`cargo build --release --example perfbench -j2`, separate
`CARGO_TARGET_DIR`): `perfbench run --filter field_ec/,dlp/ --full
--samples 1` gives 37 id/fingerprint lines, `diff` empty.
`cargo test --release --lib ecc::` on the head: 335 passed, 0 failed, 2
ignored. Callgrind `field_ec/point_scalar_mul_p256_x8`: base 165,249,830 Ir
(equal to the table), head 9,480,975 Ir (table: 9,481,005; the 30 Ir
difference is build noise from the rustfmt commit).

Constant-time reading of the diff: the only variable-time code
(`mont_vt::Mont`, `Point::scalar_mul`'s fixed-width ladder) is reached only
from `Point::scalar_mul`, which was already documented and implemented as
variable time (branches on the bits of `k`) for public inputs;
`scalar_mul_ct` calls it only on the public point and `2^scalar_bits`.
`ct_mont_mul4` and `ct_pow4` have fixed loop bounds and mask selects, as
`ct_bignum` did. `FieldElement::inv` now follows the bits of `p - 2` (the
fixed 4-bit window), so its operation sequence depends on the modulus, never
on the base: valid only because a `FieldElement` modulus is public (every
curve and group modulus in this tree is). A caller that built a
`FieldElement` over a secret modulus would leak that modulus through `inv`;
none does. The per-thread Montgomery cache holds only the public modulus and
its constants and is keyed on full modulus equality.
