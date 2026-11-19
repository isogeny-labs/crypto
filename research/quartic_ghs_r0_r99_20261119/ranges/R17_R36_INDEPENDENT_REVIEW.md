# Independent review of the composed R17/R36 q=223 record

Status: deterministic record-content validation passed in a separately
authored native validator `[pcc:unverified]`. Source binding, independent
execution replay, isolation, timing, validation-gate admission, and PCC
standing remain open.

Evidence:

- raw producer record:
  [`../receipts/r17_r36_q223.jsonl`](../receipts/r17_r36_q223.jsonl), SHA-256
  `243eff9e1eea808d483c317f450ec884343a6472b8a8b0c84fc69f74fd57ef06`;
- independent validator:
  [`../prototypes/r17_r36_validate.rs`](../prototypes/r17_r36_validate.rs),
  SHA-256
  `7e0d3676352c6458c26283680f206aecd7a588f02673650493a766dbac456392`;
- exact command/output receipt:
  [`../receipts/r17_r36_independent_validation.txt`](../receipts/r17_r36_independent_validation.txt).

## Review requirements and disposition

| Requirement | Independent check | Disposition |
|:--|:--|:--|
| JSON shape and arm inventory | Required seven records and the exact five-arm set; rejected duplicate keys, duplicate/missing/extra arms, and missing required fields | passed as a producer-side record check `[pcc:unverified]` |
| Correct recovery | Required `ok=true` and `k_found=k=1765657` in every arm | five of five records met the predicate `[pcc:unverified]` |
| Trajectory invariance | Compared batch, instance, target, steps, DPs, restarts, lookahead, cycles, cycle additions, and reciprocal setup count against `strong-b8` | four candidate tuples equalled the baseline `[pcc:unverified]` |
| Native-count invariance | Compared every emitted setup/walk native counter against `strong-b8` | four candidate native tuples equalled the baseline `[pcc:unverified]` |
| R17 phase identity | Independently recomputed each mul5 `fq_mul` and `fq_small_div` relation in setup, walk, and verify for standalone and composed candidates | six phase relations matched `[pcc:unverified]` |
| R36 exact setup delta | Bound both nine-element setup vectors to the values stated in `R36_PRODUCTION.md`; required walk/verify equality | both vectors and all non-setup counters matched `[pcc:unverified]` |
| Complete totals | Recomputed four complete counters from the phase records | stated baseline, mul5, and combined totals matched `[pcc:unverified]` |
| Counter-free authority | Required false authority, 27 zero field/group counters, and counted/fast trajectory/native equality | predicate held `[pcc:unverified]` |
| Fail-closed behavior | Injected eight independent corruptions | eight of eight were rejected `[pcc:unverified]` |
| Timing claim | Validator never parses or compares `wall_s` or calibration clock values | no timing disposition |
| Source binding | Inspected the instance token and current Git/source state | not satisfied; rerun required |

## Independently recomputed identities

For each affected phase, the validator checked:

```text
fq_mul(reference) - fq_mul(mul5) = f3_mul(reference)
fq_small_div(reference) = 0
fq_small_div(mul5) = 4 * f3_mul(reference)
```

It also required unchanged `fq_inv`, `f3_mul`, `f3_sq`, `f3_inv`,
`batch_inv`, and `group_add`. The same identities were checked once for the
standalone mul5 arm and once after composing walker-table reuse.

The recomputed complete totals were:

| Arm | `fq_mul` | `fq_inv` | `fq_add` | `fq_small_div` |
|:--|--:|--:|--:|--:|
| `strong-b8` | 110,435 | 782 | 400,276 | 0 |
| `strong-b8-mul5` | 96,884 | 782 | 670,668 | 54,204 |
| `strong-b8-mul5-walker-table` | 89,423 | 559 | 633,370 | 51,844 |

These are deterministic implementation counters from the supplied record,
not wall-time or modeled-cost results `[pcc:unverified]`.

## Tamper corpus

The independent self-test changed one condition at a time and required the
entire input to fail:

1. `k_found` changed by one;
2. one candidate's `steps` changed by one;
3. combined setup `fq_mul` changed by one;
4. fast walk `fq_mul` changed from zero to one;
5. `walk_native.hash` removed;
6. an unknown arm added;
7. the source-revision token changed; and
8. one JSON object supplied the `rec` key twice.

All eight corruptions were rejected `[pcc:unverified]`.

## Open source-binding obligation

The instance binds only the label `worktree-r17-r36`. At review time the
checked-out commit was `67e251c8c60bf8e3a9d5bea5da3fc6c480a16bd8`, while
`main.rs` and `strong_rho.rs` were modified relative to that commit. The
JSONL carries neither source digest nor binary digest. Consequently, the
content checks above cannot establish which executable produced the record.

The next admissible deterministic record must be rerun from a pushed commit
and include:

- full Git commit;
- SHA-256 of `main.rs` and `strong_rho.rs`;
- SHA-256 of the executed binary;
- exact build and run commands; and
- the same independent validator result over the replacement JSONL.

That rerun can establish a source-bound deterministic control. It still will
not establish isolated performance or move the frontier without the frozen
measurement, replay, validation, and PCC gates.
