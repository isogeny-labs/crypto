# Reviewed GHS strong-rho engineering composition

Date: 2026-11-18
Code revision bound into the control binary: `d85752a9a0`
Scope: deterministic `q=223` correctness, trajectory, operation-count, and
optimized-IR controls only

## Result

The reviewed hot-loop, reciprocal reducer, direct-adjugate inverse, corrected
jump-only setup batching, and field-counter-free specializations compose in one
source. The release selftest recovered every planted log at widths
1/4/8/16/32; the source-bound `q=223` run recovered scalar `1765657` in all
12 counted/control arms. Counted and counter-free counterparts had identical
answers, trajectories, setup-native counts, and walk-native counts. Every fast
arm emitted zero field counts with `count_authoritative=false`, and the
analyzer rejected false, null, zero, and malformed authority values
`[pcc:unverified]`.

This is a producer-side deterministic control. It is not an isolated timing,
profile, matched grid, end-to-end comparison, or frontier measurement.
Development clocks remain in the raw JSONL for audit and are excluded from
every conclusion.

## Composition disposition

| Lane | Reviewed head | Integrated disposition |
| --- | --- | --- |
| Hot loop | `b5bd269619fa` | reusable batch scratch, lazy/reused x-hash, partition mask with modulo fallback, reserved DP table |
| Corrected setup | `281afe5342f5` | batch the 32 jump simultaneous multiplications; keep every walker on charged `new_walker` retries |
| Direct inversion | `5307cc2fc65c` | direct cubic adjugate; retain Frobenius/product inverse as a selftest oracle |
| Reciprocal reduction | `e59eb29bae64` | high-product reciprocal reducer; expose one cold `setup.reciprocal_div` diagnostic |
| Counter-free field arithmetic | `a40cd497befd` | const-specialized `fast-b*` arms and fail-closed analyzer authority gate |
| Retry2 slab | `b63283b85588` | evidence only; production source excluded because the exact count comparison was negative |

The source composition order followed the independent integration review:
hot-loop spine, reciprocal reducer, direct inversion, corrected setup, then
counter-free genericization. `main.rs` uses one parsed arm structure for
`strong-b<W>`, `fast-b<W>`, and the optional `-legacy-setup` suffix;
leftover suffixes are rejected.

## Combined b8 operation counts

The paired `strong-b8` and `strong-b8-legacy-setup` records used the same
target and had identical walk, verify, native, and trajectory fields. Only
setup field work differs:

| Phase/mode | `fq_mul` | `fq_inv` | `fq_add` | `f3_mul` | `f3_sq` | `f3_inv` | `batch_inv` | `group_add` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| setup, legacy | 85,131 | 2,348 | 271,096 | 6,245 | 3,897 | 2,348 | 0 | 2,429 |
| setup, jump-batched | 56,484 | 511 | 204,142 | 6,437 | 2,346 | 511 | 33 | 1,455 |
| walk, either | 52,861 | 241 | 192,668 | 7,034 | 1,553 | 241 | 231 | 1,551 |
| verify, either | 1,090 | 30 | 3,466 | 80 | 50 | 30 | 0 | 31 |
| total, legacy | 139,082 | 2,619 | 467,230 | 13,359 | 5,500 | 2,619 | 231 | 4,011 |
| total, jump-batched | 110,435 | 782 | 400,276 | 13,551 | 3,949 | 782 | 264 | 3,037 |

Thus the combined jump-only setup control changes the complete counted b8
record by `-28,647 fq_mul`, `-1,837 fq_inv`, `-66,954 fq_add`,
`+192 f3_mul`, `-1,551 f3_sq`, `-1,837 f3_inv`, `+33 batch_inv`, and
`-974 group_add`, while preserving the recorded walk and answer
`[pcc:unverified]`. These are exact implementation counters, not elapsed-time
or end-to-end speedups.

Standalone lane deltas were not added together. Setup batching removes many
inversions that the direct inverse would otherwise optimize, so only the
recomposed binary's counts appear above.

## Archived Round B to composed default

The archived
[`round-b/q223-round-b.jsonl`](../round-b/q223-round-b.jsonl)
(SHA-256
`6b5e6715d244a783900f1bc0d4dc70ea4e65a83000351fba187dd7abcd4386d7`)
and the composed `strong-b8` record use the same q=223 scalar and preserve
the same 1,487 steps, 76 distinguished points, zero restarts, 53 lookahead
retries, two detected cycles, ten cycle additions, and recovered scalar.

| Complete counted record | `fq_mul` | `fq_inv` | `fq_add` | `f3_mul` | `f3_sq` | `f3_inv` | `batch_inv` | `group_add` | walk hashes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| archived Round B | 149,558 | 2,619 | 498,658 | 15,978 | 5,500 | 2,619 | 231 | 4,011 | 4,534 |
| composed default | 110,435 | 782 | 400,276 | 13,551 | 3,949 | 782 | 264 | 3,037 | 1,567 |
| composed minus Round B | -39,123 | -1,837 | -98,382 | -2,427 | -1,551 | -1,837 | +33 | -974 | -2,967 |

This is an exact deterministic archived-source count comparison
`[pcc:unverified]`. It is not an additive estimate: the composed binary was
run directly. Walk hashes are an unpriced native diagnostic, and the table
does not establish an elapsed-time or frontier change.

## Width and counter-elision controls

The counted q=223 arms produced:

| Arm | steps | DPs | lookahead | cycles | total `fq_mul` | total `fq_inv` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| strong-b1 | 1,615 | 107 | 62 | 0 | 92,215 | 1,833 |
| strong-b4 | 2,481 | 155 | 92 | 1 | 135,737 | 1,030 |
| strong-b8 | 1,487 | 76 | 53 | 2 | 110,435 | 782 |
| strong-b16 | 1,008 | 57 | 34 | 0 | 111,109 | 1,117 |
| strong-b32 | 2,000 | 114 | 68 | 0 | 181,502 | 2,069 |

These records expose width sensitivity but do not select a throughput winner.
The corresponding `fast-b*` records preserved every listed trajectory field
and all native counters. The optimized LLVM-IR gate found 11 counted and 11
uncounted arithmetic definitions, with counter SSA markers changing from
262 to 0 in arithmetic and from 6 to 0 in the walk wrapper
`[pcc:unverified]`. Native diagnostic counters intentionally remain active.

## Controls and commands

From `research/quartic_ghs_strong_rho_20261114`:

```sh
GHS_SOURCE_REVISION=d85752a9a0 rustc --edition 2021 -O main.rs \
  -o /dev/shm/ghs-leads/integration/ghs-strong-rho-integrated-bound
/dev/shm/ghs-leads/integration/ghs-strong-rho-integrated-bound selftest
/dev/shm/ghs-leads/integration/ghs-strong-rho-integrated-bound run \
  223 1 0 1 ic-repeat \
  strong-b1,strong-b4,strong-b8,strong-b16,strong-b32,fast-b1,fast-b4,fast-b8,fast-b16,fast-b32,strong-b8-legacy-setup,fast-b8-legacy-setup
jq -e -s -f round-c-integration/check_q223.jq \
  round-c-integration/q223.jsonl
python3 -m unittest -v test_analyze.py
bash round-c-counterfree/verify_counterfree_ir.sh
```

Artifacts:

- [`q223.jsonl`](q223.jsonl) — source-bound raw deterministic control.
- [`check_q223.jq`](check_q223.jq) and
  [`validation.txt`](validation.txt) — recomputation and result.
- [`selftest.txt`](selftest.txt) — algebra, reducer, setup, trajectory, and
  counter-elision selftest output.
- [`analyzer-test.txt`](analyzer-test.txt) — ten analyzer tests.
- [`ir-gate.txt`](ir-gate.txt) — optimized-IR counter-elision gate.
- [`round-b-comparison.txt`](round-b-comparison.txt) — archived Round B and
  composed-default exact count delta.
- [`full-test-status.txt`](full-test-status.txt) — required repository-wide
  release-test attempt and its ten unrelated compile blockers.
- [`metadata.txt`](metadata.txt) — source, binary, compiler, and host hashes.
- [`SHA256SUMS`](SHA256SUMS) — artifact digests.

The bound binary SHA-256 is
`8ce42ed8f774b9f1bd16ce455cf3eef2f6b6de2cf84b12d92b1fd92d117f3344`;
`strong_rho.rs` is
`724f1c433b465818da6b0ee6d0ad7deb01434c6bb114e87dae4e304fce4bd340`.

## Retained failures and boundary

- The first setup lane rolled counters and RNG state back after a rare
  infinity fallback. Independent review rejected that accounting. Its evidence
  remains under `round-c-setup/`; the corrected jump-only implementation and
  evidence are under `round-c1-setup/`.
- Retry2 preserved the tested trajectory but exchanged 43 batch inversions for
  50,381 additional base-field multiplications and 1,437 group additions in
  its branch-local control. Its code is not in the combined source.
- One integration-time `rustfmt` command used a repository-relative path
  while already inside the study directory and returned file-not-found. It
  changed no file; the corrected formatting command and subsequent build
  passed.
- The first required `cargo test --release --lib` attempt reached the crypto
  test crate but did not run tests: nine sparse included artifacts and one
  duplicate unrelated test function produced ten compile errors. After the
  tracked artifacts were materialized, the exact rerun failed only on that
  duplicate function, which is also duplicated in `origin/main`. Both
  attempts and exit statuses are retained in
  [`full-test-status.txt`](full-test-status.txt).
- `conductor` was not installed, and `scripts/research-kb status` found no
  executable. The lane protocols retain the repository-local sources and
  prior-art receipts used by their authors.
- Checkpoint credentials/admission were unavailable. No profile, isolated
  timing, matched full grid, leaderboard update, or frontier conclusion was
  produced. The unresolved measurement requirement remains in
  [`problems/ghs-strong-rho-measurement-admission.md`](../../../problems/ghs-strong-rho-measurement-admission.md).
