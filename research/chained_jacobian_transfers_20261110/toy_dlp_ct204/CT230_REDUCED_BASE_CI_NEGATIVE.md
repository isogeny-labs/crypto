# CT230 — CT229 native reduced-base cold-cost adjudication (2026-11-19)

[crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715) · **[pcc:unverified]**

## Result

The fixed native CT229 six-size pilot **finished**, including the previously pending Rust job in [GitHub Actions run 44491343842](https://github.com/isogeny-labs/crypto/actions/runs/44491343842) (native job `135447446971`; field-oracle job `135447447336` also passed). The exact producer source was `b7c4be0e0688977cabd9939d69ebcd2158dd17c5`.

All six precommitted prefix sizes were preserved, including four failures. The native log contains two separate executions on **one** hosted runner: the full CT226 test invocation also executed CT229 once, and then the filtered CT229 command ran it again. The final CI `grep` printed the second panel a third time; it is **not** a third execution or second host.

| Signed base | Smooth | Rank | Trials | ECDLP recovered? | Run A complete elapsed | Run B complete elapsed |
|---:|---:|---:|---:|:---:|---:|---:|
| 12 | 0 | 0/12 | 6540 | No | 177.422 ms | 173.613 ms |
| 18 | 3 | 1/18 | 6540 | No | 178.873 ms | 177.636 ms |
| 24 | 3 | 1/24 | 6540 | No | 184.675 ms | 182.474 ms |
| 36 | 38 | 34/36 | 6540 | No | 191.917 ms | 191.595 ms |
| 48 | 54 | 48/48 | 2213 | Yes, 37 | 68.132 ms | 67.775 ms |
| 54 | 62 | 54/54 | 1421 | Yes, 37 | 44.773 ms | 44.562 ms |

The **only successful** truncated basis (48) requires 1.56 times as many relation candidates as 54 (2213/1421), and complete candidate time is 1.52 times longer in both native executions. Shrinking the matrix by six variables did **not** reduce total solving time.

Measured same-job **candidate/B**SGS ratios among successful arms are 40,755–41,468× slower at 48 and 27,086–27,288× slower at 54. These are measured *toy negative ratios*, not qualified frontiers: BSGS microsecond times are noisy; comparison lacks independent isolated runs, optimized negation-map rho, reference IC, peak RSS, physical transfer/norm cost, blind new targets, uncertainty intervals and signed PCC. **Never calculate a completed-solve speed ratio for the four failed arms.** All six configurations and two executions were retained.

## Scientific decision

The registered lexicographic prefix truncation is negative at this `F101` fixed known-answer cover: 12–36 fail full rank/target within 6,540 candidates; 48 recovers the target but is slower than the 54 control. This does not prove that *all* 36-element bases, non-prefix selection or large-prime combining fail. The 36-arm achieved rank 34 with 38 smooth candidates, so the next diagnostic should measure the **specific unrecovered factor symbols and partial-support relation graph** without post-hoc declaring a DLP success. The next serious candidate is degree-4 partial-support large-prime joining with a preregistered, bounded memory budget, retaining the source-group target and identical complete cold comparators.

Any proposed speed advantage must charge relation collection, all LA, full transfer setup and target recovery including norm-projector operations; the original authenticated 695/701 records and physical `C169`, `C13a`, `C13b`, `D` remain separately gated under S3 authorization/budget STOP. Reuse the existing cryptanalysis norm-reuse and cold-adjudicator branches; do not duplicate them.

## Audit and reproduction

The [frozen two-panel TSV](ct230_native_reduced_base_pairs.tsv) transcribes the **12 unique measured** CT229 JSON records from the GitHub job, with counters, nanoseconds, phase breakdowns, target results and exact head provenance. This is a bounded, provenance-linked extraction, not the entire raw Actions log. The [std-only Rust audit](../../../tests/ct230_reduced_base_cost_audit.rs) validates both unpruned panels, all expected ranks/smooth counts and candidate counts, target scalar, control, phase accounting, counter consistency, exact provenance, and seven failing mutation controls.

```sh
rustc --edition=2021 --test tests/ct230_reduced_base_cost_audit.rs -o /tmp/ct230_audit
/tmp/ct230_audit --nocapture
# Original solver from CT229 (requires the full Rust repository build):
cargo test --locked --release --test ct226_native_genus4_transferred_ic ct229_ -- --nocapture
```

Audit workflow: [CT230 standalone Rust audit](../../../.github/workflows/ct230-reduced-base-audit.yml). The audit verifies the frozen record's **internal fidelity**, not independent regeneration of the native logs, secure attestation, or cryptanalytic cost advantage. All findings remain **[pcc:unverified]**. No paid EKS/Spot run, protected merge or new data-access authorization was made.
