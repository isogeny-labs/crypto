# Companion PR #4924 CI provenance audit

## Conclusion

The source-commit discrepancy is resolved. Run [45086381347](https://github.com/isogeny-labs/crypto/actions/runs/45086381347), job [137405842381](https://github.com/isogeny-labs/crypto/actions/runs/45086381347/job/137405842381), checked out **`aa27edacb477ddc55bccd23b86d90584ce438cd1`**. The report's `b8a594ea514ff44cadb5342986c68b1eac6e22ed` is the **Runner Image Provisioner commit**, not the crypto repository source commit.

This is a read-only audit of the actual decoded job log, run metadata, and source blobs. No job was rerun; no independent execution or PCC certification was performed; no remote files or PRs were changed.

## Source binding

- Metadata identifies the run as completed/success, workflow run number 6, head `aa27edacb477ddc55bccd23b86d90584ce438cd1`.
- At 00:01:48 UTC on 2026-11-23 the Runner Image Provisioner header prints `Commit: b8a594ea514ff44cadb5342986c68b1eac6e22ed`.
- At 00:01:50 UTC the checkout action fetches `aa27edacb477ddc55bccd23b86d90584ce438cd1`.
- At 00:03:19 UTC `git log -1 --format=%H` prints `aa27edacb477ddc55bccd23b86d90584ce438cd1`.
- The [report inspected at frozen snapshot fb850179…](https://github.com/isogeny-labs/crypto/blob/fb850179c3847fd6950dbd91015c4e3d7c6da62d/research/trace_skew_prime31_20261122/NATIVE_F4_RUST_RUN74_20261123.md) has blob SHA `83f8a71d3ba9345c4c52ac5d9446424f2ce6d547` and mislabels the runner commit as the tested source commit.

### Exact correction to propose to the owning PR

Replace the report's first-paragraph source commit `b8a594ea514ff44cadb5342986c68b1eac6e22ed` with `aa27edacb477ddc55bccd23b86d90584ce438cd1`. Add: “The earlier identifier was the hosted runner provisioner commit. This run binds to the repository checkout recorded by the checkout action and git log.”

## Tests observed in the original Rust job

All below are observations from the existing hosted run; they were not independently rerun.

| Example | Named tests reported passing | Reported summary |
|---|---|---|
| `prime31_trace_skew_pair` | `exhaustive_small_field_scheme_and_jacobian`; `prime31_pair_membership_s3_and_outside_negative` | 2 passed, 0 failed |
| `prime31_trace_moment` | `prime31_m2_to_m4_repeat_root_and_rational_base`; `exhaustive_f17_m2_m3_m4_with_all_negative_assignments` | 2 passed, 0 failed |
| `prime31_monic_s3_pair_oracle` | `quotient_pair_ghost_roots_are_filtered`; `monic_s3_pair_oracle_exact_public_positive_and_negative_controls` | 2 passed, 0 failed |

The log shows `cargo test --locked --release --example ...` and `cargo run --locked --release --example ...` against the original crate. Rust toolchain 1.100.0 is selected. The monic oracle's own output explicitly leaves full signed group checks, authentic right-hand-side rank, cold solving, and asymptotic speedup unestablished. [pcc:unverified]

## Representative numerical claims checked against the log

| Control | Ordered F4 | Reduced symmetric G F4 | Scope |
|---|---:|---:|---|
| m=2, B=32 sweep | 1.881 ms; 145,604 field operations; maximum rows 73, columns 189 | 1.023 ms; 57,147 field operations; maximum rows 498, columns 561 | One planted positive; separately measured maxima |
| m=4, B=4 genuine chained S3 | 8.065 ms; 1,036,961 field operations; maximum rows 686, columns 742 | 6.557 ms; 2,182,404 field operations; maximum rows 294, columns 326 | Planted G+2G+3G+4G=10G; six variables |

These selected figures agree with the report. The m=4 symmetric arm uses about 2.10 times as many reported field operations despite its smaller matrix envelope and lower single observed wall time. These are source-bound control observations, not complete solver benchmarks.

## Later snapshot is not fully covered by this run

Each file was fetched at both the tested commit and the report snapshot. Git blob hashes give the following exact comparison:

| Path | Tested aa27edac… blob | Snapshot fb850179… blob | Equal |
|---|---|---|---|
| `examples/prime31_trace_skew_pair.rs` | `a5050e0cc06feb203066391c7ad70d81d87dd8c9` | `a5050e0cc06feb203066391c7ad70d81d87dd8c9` | yes |
| `examples/prime31_trace_moment.rs` | `7d2c0cc4c50e52f585676349ecf13901b7224615` | `7d2c0cc4c50e52f585676349ecf13901b7224615` | yes |
| `examples/prime31_monic_s3_pair_oracle.rs` | `cc11ef6ee5c6b67bffb07897e7ce24f6921c00c1` | `1ed84edc5c0d70710309004fb661e6ac4b8c03b0` | NO |
| `examples/prime31_trace_f4_pilot.rs` | `bade6712c12fe7b2426a206495ff7bc85befc500` | `bade6712c12fe7b2426a206495ff7bc85befc500` | yes |
| `.github/workflows/prime31-trace-symmetric-rust-gate.yml` | `539779648b58add5e1233b80e7945507f2448f8a` | `6a05ba8d474f3a971ef1e2c2110b29984f984223` | NO |

The skew-pair, moment, and F4-pilot source bytes are identical across these snapshots. The monic oracle and workflow changed. Therefore this particular run must not be cited as validation of the later monic-oracle implementation or the entire later branch. No comparison of all transitive dependency bytes was performed.

## Evidence and remaining limits

The companion `SIBLING_CI_AUDIT.json` retains the exact run/job identifiers, frozen report/source blob hashes, and small selected original log excerpts sufficient to audit the checkout error and the reported outcomes. The original log remains available through the job URL.

**Supported:** existing CI passed the listed original Rust controls at aa27edac…; representative printed numerical observations match the report; the source-commit label is wrong and its cause is identified.

**Not supported:** independent reproduction, complete current-branch validation, blinded unknown-index relation extraction, useful independent relation-rank yield, a cold ECDLP cost advantage, or an asymptotic improvement. [pcc:unverified]

