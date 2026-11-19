# R17 production integration: five-evaluation cubic multiplication

Status: selectable production candidate implemented; deterministic algebra,
trajectory, and counter controls passed; isolated timing and modeled-cost
promotion remain unmeasured `[pcc:unverified]`.

## Change

The composed strong-rho source now accepts the diagnostic suffixes `-mul5`
and `-mul6` and their `-walker-table` compositions. The current six-product
Karatsuba kernel remains the default and explicitly named differential
control.

The candidate evaluates at `0`, `1`, `-1`, `2`, and infinity, interpolates
with exact canonical divisions, and reduces once modulo `z^3-c`. A new
`fq_small_div` authority field counts each logical `/2` and `/3`. Moduli not
coprime to six fall back to the six-product kernel.

## Deterministic controls

The independent standalone R17–R33 oracle passed 768,394 exhaustive pairs and
180,000 deterministic larger-field pairs. The production self-test then
compared 20,000 additional pairs against both production Karatsuba and the
frozen field implementation. Every production mul5 call used exactly five
`fq_mul` and four `fq_small_div` operations `[pcc:unverified]`.

The q=223 complete-path control ran:

```text
/tmp/ghs-r0-r99-composed run 223 1 0 1 ic-repeat strong-b8,strong-b8-mul5,strong-b8-walker-table,strong-b8-mul5-walker-table,fast-b8-mul5-walker-table
```

All five arms recovered scalar `1765657` and retained 1,487 steps, 76
distinguished points, zero restarts, 53 lookahead retries, two cycles, ten
cycle additions, and identical native records `[pcc:unverified]`.

For every phase, mul5 reduced `fq_mul` by exactly that phase's `f3_mul` count
and introduced exactly four `fq_small_div` per `f3_mul`. Against the unchanged
`strong-b8` control, the standalone mul5 arm changed the complete record by:

| Counter | Karatsuba | Mul5 | Mul5 − Karatsuba |
|:--|--:|--:|--:|
| `fq_mul` | 110,435 | 96,884 | -13,551 |
| `fq_inv` | 782 | 782 | 0 |
| `fq_add` | 400,276 | 670,668 | +270,392 |
| `fq_small_div` | 0 | 54,204 | +54,204 |

The joint-table-plus-mul5 arm recorded 89,423 `fq_mul`, 559 `fq_inv`, 633,370
`fq_add`, and 51,844 `fq_small_div` over setup, walk, and verification
`[pcc:unverified]`. These competing operation movements require an admitted
wall and cost-sensitivity measurement; no speedup or frontier change is
claimed.

Raw output: [`receipts/r17_r36_q223.jsonl`](receipts/r17_r36_q223.jsonl).

## Retained failed attempt

The first production build after adding the `use_mul5` field failed with Rust
error E0063 because a self-test-only `FastF3` literal lacked the new field.
The multi-line shell did not have `set -e`, so it subsequently ran the stale
pre-change binary. That output is excluded. The literal was fixed and every
admitted build/run command was repeated with `set -e`; the later receipt is
the only q=223 input used above.

## Analyzer and CLI remediation

- [`ranges/R17_NATIVE_ANALYZER.md`](ranges/R17_NATIVE_ANALYZER.md) records a
  dependency-free native checker. It rejects missing/malformed authority or
  phase counters, enforces the kernel identities, and requires an explicit
  rational small-division sensitivity rather than silently pricing the work
  at zero.
- The CLI now accepts explicit `-mul6` and `-mul6-walker-table` controls and
  emits the effective kernel after the modulus-domain fallback.

## Remaining gates

- Integrate the native authority/model contract into the canonical publication
  analyzer and require the split field/native authority introduced by R64.
- Freeze the treatment of exact small divisions in the full cost-sensitivity
  band; the native checker deliberately does not rank arms.
- Run source-bound isolated A/A and ABAB measurements after execution
  admission is restored.
- Obtain independent replay, validator, matched reference, and PCC standing
  before any leaderboard update.
