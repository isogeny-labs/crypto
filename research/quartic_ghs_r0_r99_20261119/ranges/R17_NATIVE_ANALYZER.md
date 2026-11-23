# R17 native small-division authority analyzer

Status: implemented as a standalone native fail-closed checker and exercised
against the retained composed q=223 JSONL. It is not yet wired into the
canonical frontier analyzer or publication gate. All execution observations
are producer-side `[pcc:unverified]`.

Source: [`r17_native_analyzer.rs`](../prototypes/r17_native_analyzer.rs)
Receipt: [`r17_native_analyzer.txt`](../receipts/r17_native_analyzer.txt)

## Requirement status

| Requirement | Status | Evidence |
|:--|:--|:--|
| Native implementation without Python or external crates | implemented | one standalone Rust source, compiled directly with `rustc -D warnings` |
| Parse flat JSONL fail-closed | implemented | malformed values, duplicate keys, nested JSON, trailing data, missing keys, and wrong types return exit 2 |
| Require `field_kernel` on every rho row | implemented | missing and unknown-kernel mutations rejected |
| Require phase-local `fq_small_div` | implemented | `setup`, `walk`, and `verify` parsed independently; missing, malformed, and cross-phase substitutions rejected |
| Enforce kernel identities | implemented | `karatsuba6` requires zero; `evaluation5` requires exactly `4*f3_mul` in each phase |
| Enforce counted/fast authority | implemented | `strong-*` requires authoritative counts; `fast-*` and `bare-*` require non-authoritative all-zero field components |
| Avoid silent zero pricing | implemented | `check` requires an explicit nonnegative `NUM/DEN` small-division weight; omission is an error |
| Test retained composed records | complete at producer scope | all five rho records in `r17_r36_q223.jsonl` accepted |
| Test corruptions | complete at producer scope | twelve record corruptions and seven malformed weights rejected |
| Establish a speed or ranking result | not attempted | the tool intentionally ignores wall time and does not rank arms |

## Interface

```sh
rustc --edition 2021 -D warnings -O \
  prototypes/r17_native_analyzer.rs \
  -o /tmp/r17-native-analyzer

/tmp/r17-native-analyzer check 1/2 receipts/r17_r36_q223.jsonl
/tmp/r17-native-analyzer --selftest receipts/r17_r36_q223.jsonl
```

The first command requires an explicit small-division sensitivity. `1/2`
means one `fq_small_div` is displayed as one half of a multiplication in the
narrow diagnostic quantity:

```text
mul_plus_small_div = fq_mul + (NUM / DEN) * fq_small_div
```

The program prints the exact numerator and denominator rather than rounding.
An explicitly supplied zero such as `0/7` is accepted and canonicalized to
`0/1`; omitting the weight is rejected. Thus zero remains an inspectable
sensitivity choice rather than an implicit model assumption
`[pcc:unverified]`.

This diagnostic quantity is not complete cost. The output retains `fq_inv`,
`fq_add`, `f3_mul`, `f3_sq`, `f3_inv`, `batch_inv`, and `group_add` beside it,
but does not invent weights for them. It also excludes native work and elapsed
time. Consumers must use the frozen complete cost model before making a ratio
or ranking statement.

## Fail-closed record contract

Every nonblank line must be one flat JSON object, including non-rho metadata
rows. Duplicate keys, nested arrays or objects, unsupported raw non-ASCII
strings, malformed numbers, and trailing bytes are rejected. A row with
`rec:"rho"` additionally requires:

- a recognized arm class: `strong-*`, `fast-*`, or `bare-*`;
- Boolean `count_authoritative` consistent with the arm class;
- `field_count_authoritative`, when present, consistent with the same class;
- `field_kernel` equal to `karatsuba6` or `evaluation5`; and
- all nine field/count components for each of `setup`, `walk`, and `verify`.

For `karatsuba6`, each phase must have `fq_small_div=0`. For `evaluation5`,
each phase must satisfy `fq_small_div=4*f3_mul`. A non-authoritative arm must
have every parsed field component equal to zero, so an apparently correct
small-division field cannot hide other counted work `[pcc:unverified]`.

The three phases are never merged for validation. The output emits one line
per phase per arm. The selftest's phase-cross-substitution mutation replaces
the evaluation5 setup count with the valid walk count; it is rejected because
the setup identity no longer holds.

## Retained-record result

The checker accepted five rho rows from the retained composed q=223 record:

- `strong-b8` / `karatsuba6`;
- `strong-b8-mul5` / `evaluation5`;
- `strong-b8-walker-table` / `karatsuba6`;
- `strong-b8-mul5-walker-table` / `evaluation5`; and
- `fast-b8-mul5-walker-table` / `evaluation5`.

It emitted 15 phase rows under the explicit weight `1/2`. This establishes
only that those stored fields satisfy this native authority schema
`[pcc:unverified]`.

## Corruption matrix

The built-in selftest derives corruptions in memory from the retained input;
it does not overwrite evidence.

| Corruption | Expected result | Observed producer result |
|:--|:--|:--|
| missing `field_kernel` | reject | rejected |
| missing `setup.fq_small_div` | reject | rejected |
| string instead of integer small-div count | reject | rejected |
| unknown kernel | reject | rejected |
| nonzero Karatsuba small-div count | reject | rejected |
| evaluation5 count not equal to `4*f3_mul` | reject | rejected |
| counted arm marked non-authoritative | reject | rejected |
| fast arm marked authoritative | reject | rejected |
| fast arm with another nonzero field count | reject | rejected |
| unknown arm class | reject | rejected |
| valid count copied from the wrong phase | reject | rejected |
| duplicate `field_kernel` key | reject | rejected |

Weights `""`, `1`, `1/0`, `-1/2`, `1/-2`, `1.5/2`, and `1/2/3` were also
rejected. The final selftest line was `SELFTEST OK` `[pcc:unverified]`.

## Integration boundary

This closes the missing native replacement path at prototype scope. It does
not by itself repair `analyze.py`, change the canonical schema, validate the
stored measurements independently, or authorize publication. A production
integration should reuse this contract in the native canonical analyzer,
bind its source and input hashes, add the corruption matrix to CI, and then
retire—not silently bypass—the legacy path.
