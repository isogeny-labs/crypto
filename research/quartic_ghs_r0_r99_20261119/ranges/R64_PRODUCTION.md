# R64 production integration: bare strong-rho accounting path

Status: selectable production path implemented; deterministic parser,
known-answer, trajectory, composition, and optimized-IR counter-elision
controls passed. No wall-time benchmark, modeled-cost result, speedup, or
frontier movement is claimed `[pcc:unverified]`.

## Requirement-to-evidence map

| Requirement | Implementation or gate | Status |
|:--|:--|:--|
| Selectable counter-free production mode | `strong_rho_bare` plus the `bare-b*` arm family | implemented `[pcc:unverified]` |
| Preserve existing arms and defaults | `strong-*` remains fully counted; `fast-*` remains field-counter-free with native diagnostics; parser selftest checks all three authority modes | deterministic control passed `[pcc:unverified]` |
| Preserve known answers and trajectories | paired counted/fast/bare selftest at widths 1, 4, 8, 16, and 32 | passed `[pcc:unverified]` |
| Compose with R17 mul5 and R36 walker-table reuse | bare combined-path selftest at width 8 | passed `[pcc:unverified]` |
| Remove field-counter mutation | existing optimized `FastF3<false>` LLVM-IR gate | zero counter SSA markers in arithmetic and walk `[pcc:unverified]` |
| Remove native-counter mutation | zero-sized `BareNative` sink, type-size guard, and reusable `prototypes/verify_r64_native_ir.sh` gate | no native-counter formal/reference/bump definition in the retained bare path `[pcc:unverified]` |
| Retain an explicit mul6 control | one q=223 CLI correctness invocation of `strong-b8-mul6` and `strong-b8-mul6-walker-table` | both recovered the same scalar/trajectory and emitted `field_kernel=karatsuba6` `[pcc:unverified]` |
| Measure performance or update the frontier | intentionally not run in this pass | blocked by measurement admission; still open |

## Production surface

The new public entry point is:

```rust
pub fn strong_rho_bare(
    inst: &Inst,
    k_true: u64,
    seed: u64,
    opts: StrongOpts,
) -> StrongOut
```

The CLI accepts these forms for any supported width `W`:

- `bare-bW`
- `bare-bW-mul5`
- `bare-bW-mul6`
- `bare-bW-walker-table`
- `bare-bW-mul5-walker-table`
- `bare-bW-mul6-walker-table`
- `bare-bW-legacy-setup`

The default arm behavior is unchanged. `strong-bW` retains both field and
native accounting. `fast-bW` retains native accounting while removing only
field counter updates. The new `bare-bW` path removes both counter families.
Its zero counts are non-authoritative by construction.

Records retain the legacy `count_authoritative` field and add an explicit
split:

| Arm family | `field_count_authoritative` | `native_count_authoritative` | legacy `count_authoritative` |
|:--|:--:|:--:|:--:|
| `strong-*` | true | true | true |
| `fast-*` | false | true | false |
| `bare-*` | false | false | false |

The explicit `-mul6` suffix selects the same Karatsuba control as the
historical unsuffixed arm. `field_kernel` reports the effective kernel rather
than merely echoing the request, so a mul5 request on an unsupported modulus
is labeled `karatsuba6`. Walker-table reuse is accepted only with batched
setup; the core setup routine rejects the invalid combination.

## Mechanism

The mathematical walk remains one statically dispatched generic body.
`CountedNative` owns the historical `NativeCounts` record and performs the
existing increments. `BareNative` is a zero-sized type whose statically
dispatched bump operations are empty. A compile-time size assertion requires
`BareNative` to remain zero bytes. Field accounting is independently selected
through the pre-existing `FastF3<false>` specialization.

This design keeps point transitions, coefficient updates, partitioning,
look-ahead, cycle handling, distinguished-point tests, collision recovery,
RNG calls, and retry branches in the same Rust control flow for all three
accounting modes. The only selected behavior is the accounting sink
`[pcc:unverified]`.

## Deterministic controls

At widths 1, 4, 8, 16, and 32, the selftest paired the counted, field-only
counter-free, and bare implementations using the same options and seeds. The
planted scalars were 17, 118, 219, 320, and 421. For every width it required
the bare result to match the counted result in:

- success and recovered scalar;
- steps and distinguished-point table size;
- restarts and look-ahead retries;
- detected cycles and cycle-escape additions; and
- reciprocal setup behavior.

It additionally required all bare setup, walk, and verification field counts
and both native-count phases to equal their all-zero defaults. The counted
control still had to match its frozen per-width field/native oracle. The
selftest returned `SELFTEST OK` `[pcc:unverified]`.

The composition control selected `use_mul5=true` and
`reuse_setup_table_for_walkers=true` together at width 8 with planted scalar
119 and seed `0xC01D`. The bare result matched the counted combined arm on the
same trajectory fields and returned zero field/native counts
`[pcc:unverified]`.

The post-remediation CLI correctness check ran the explicit mul6 controls on
q=223, seed 1, target index 0. Both recovered scalar 1,765,657 and reported
the identical trajectory `(steps=1487, dps=76, restarts=0, lookahead=53,
cycles=2, cycle_adds=10)`. Both records labeled the effective kernel
`karatsuba6`; their setup-mode labels distinguished ordinary batched setup
from walker-table reuse `[pcc:unverified]`. The command emitted incidental
calibration and `wall_s` values because that is the production CLI schema;
those values were excluded from the validator and are not measurements or
comparisons in this report.

## Optimized-IR gates

The existing field-counter gate reported:

```text
counted_arithmetic_counter_ssa_markers=271
uncounted_arithmetic_counter_ssa_markers=0
counted_walk_counter_ssa_markers=6
uncounted_walk_counter_ssa_markers=0
PASS: optimized COUNT=false removes FastF3 counter read/modify/write hooks
```

The reusable native-counter gate is
`prototypes/verify_r64_native_ir.sh`. It uses `set -euo pipefail`, a private
`mktemp` directory with an exit trap, and no fixed mutable output. It compiles
the current source with LLVM IR at optimization level 3, extracts the complete
retained `strong_rho_bare` definition and every retained function definition
explicitly specialized on `BareNative`, and fails closed if its counted
positive control or any bare-path premise is absent. The post-remediation
result was:

```text
bare_specialized_defs=1
counted_specialized_defs=4
bare_native_counter_formals=0
counted_native_counter_formals=4
bare_wrapper_defs=1
bare_wrapper_lines=3385
bare_wrapper_counter_references=0
bare_bump_definitions=0
bare_path_references=67
PASS: optimized bare strong-rho path retains no NativeCounts mutation hook
```

The counted specializations are the positive control: all four retained
definitions carry a named native-counter formal, while the retained bare
specialization carries none. The complete optimized bare wrapper contains no
`CountedNative`, `NativeCounts`, native-counter formal, or bump-hook reference;
67 `BareNative` path references confirm that the scanner examined the live
bare walk rather than an unused specialization `[pcc:unverified]`.

Exact commands, source and binary hashes, toolchain, outputs, and limitations
are retained in `receipts/r64_validation.txt`.

## Remaining gates

- The new mode has no admitted wall-time, instruction, cache, memory, or E2E
  measurement. The incidental timing fields from the CLI correctness check
  are excluded. No performance conclusion follows from counter elision.
- The deterministic controls cover the frozen development instance and
  options above; they are not a statistical rho-constant panel or a curve
  sweep.
- The legacy analyzer tests still pass, but this pass did not add a native
  analyzer contract for split field/native authority.
- An admitted source-bound A/A and interleaved A/B panel, with complete setup,
  walk, verification, and host metadata, remains required before promotion or
  leaderboard use.
