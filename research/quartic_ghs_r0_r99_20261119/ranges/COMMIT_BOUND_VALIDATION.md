# Independent commit-bound q=223 capsule validation

- Date: 2026-11-19 UTC
- Source revision: `95297ce1d098e124e72d45effc07fe8c640de82b`
- Tree: `aaf30505039899dfce2125579cf1256d308ecd1b`
- Claim scope: correctness, deterministic trajectory, and deterministic
  accounting only. Timing fields are retained but excluded. No performance or
  frontier claim is made.

## Outcome

The final stable capsule passes the independent strict validator. The manifest's pinned commit and tree exist; all six source paths resolve to the declared Git blobs; all six blob contents match their declared SHA-256 digests; every retained artifact matches its declared digest and byte count; and the exact eight-arm q=223 record passes the correctness and accounting contract. The in-memory corruption matrix rejected 17 of 17 cases.

The binary rebuild was deliberately kept separate from the record validator. Running the retained rebuild script independently on this snapshot produced two byte-identical binaries from two fresh archive roots, each with SHA-256 `cbd81182cd5bf6d91ebb53e45d3b353644fae48372eaddc856415808124f28b5`, and reproduced the retained selftest streams.

## Requirement-to-evidence status

| Requirement | Status | Evidence |
|---|---|---|
| Strict manifest and record parsing | Verified | Duplicate keys, unknown/missing keys, wrong types, and noncanonical unsigned integers fail closed in `commit_bound_validate.rs`. |
| Commit/tree/source binding | Verified | Git commit type, exact tree, six path-to-blob resolutions, and six independent blob SHA-256 checks passed. |
| Artifact digests | Verified | Record, selftest stdout/stderr, rebuild script, and rebuild receipt hashes and declared byte counts passed. |
| Compiler binding | Verified | Local `rustc -vV` matched SHA-256 `be69adad9137eaa0bdac1dd07e2fe286b1831fc8bad193ea27806ca0c7c8e7c6`. |
| Eight-arm schema and order | Verified | Exactly one instance, one calibration, and eight rho rows in the declared order; exact key sets required. |
| Correct recovered scalar and trajectory | Verified | All eight arms have `k = k_found = 1765657`, `ok=true`, and the pinned q=223 trajectory. |
| Authority/kernel/setup matrix | Verified | Six strong, one fast, and one bare arm match the declared authority conjunctions, kernels, and setup modes. |
| Pinned counters and composition | Verified | 18 field phase vectors and 16 native vectors checked; both mul5 phase relations checked. |
| Alias behavior | Verified | Unsuffixed/mul6 and walker/mul6-walker pairs are identical in all non-clock fields except arm name. |
| Fast/bare controls | Verified | Fast field counters are zero while native counters match the counted combined arm; bare field and native counters are zero. |
| Corruption rejection | Verified | 17/17 representative corruptions rejected. |
| Binary reproducibility | Verified separately | Two fresh archive roots yielded the same 848,736-byte binary and selftest streams. |
| Timing/frontier inference | Not attempted; inadmissible | `wall_s` and calibration clocks were checked only for finite, non-negative syntax. |

## Corruption matrix

The validator rejects modifications to:

1. manifest schema;
2. source commit;
3. Git blob ID;
4. source SHA-256;
5. record SHA-256;
6. arm ordering;
7. recovered scalar;
8. trajectory;
9. authority conjunction;
10. mul6 alias counters;
11. fast field-counter zeros;
12. bare native-counter zeros;
13. pinned field counters;
14. required record keys;
15. record cardinality;
16. duplicate JSON keys; and
17. retained artifact bytes.

## Corrected preliminary defect

Before the final stable snapshot, the first uncommitted build manifest used placeholder paths without a path-remapping build rule. A clean archive rebuild produced SHA-256 `5914a14df6bdbe6abdfd4c14497ba720e072f33b74f670607ff8db1f08a0f7d8`, rather than the then-recorded `d7fbb0d05fadb27f913be75b6fd8baf0c00f69015becd53d9b6b2473325d9766`. Inspection showed absolute temporary source paths embedded in the ELF.

That preliminary artifact was superseded before this validation. The final build adds `-C codegen-units=1 -C strip=debuginfo --remap-path-prefix <archive-root>=/source`; two independently materialized archive roots now produce the same final digest `cbd81182cd5bf6d91ebb53e45d3b353644fae48372eaddc856415808124f28b5`. No result from the preliminary artifact is used as final evidence.

## Remaining provenance improvements

The current capsule passes its exact declared schema. The following additions would make later independent review less dependent on interpreting manifest assertions:

- Define the canonical construction of `source_sha256_manifest`, or retain the exact source-hash manifest it digests. The six per-file source hashes passed the independent check, but the aggregate digest has no declared serialization algorithm `[pcc:unverified]`.
- Replace or supplement `pre_post_source_hashes_equal: true` with explicit `pre_source_sha256_manifest` and `post_source_sha256_manifest` values plus their canonicalization rule.
- Add `run.producer_binary_sha256` and a deterministic normalized replay receipt. The current manifest associates the rebuilt binary with the q=223 record, and the record embeds the full source revision, but this audit did not regenerate the q=223 record from the rebuilt binary. A replay should exclude only `wall_s` and the four calibration clock values, then hash and compare all remaining fields.
- Retain a named zero-byte run-stderr artifact and its path, rather than only its empty-stream digest and byte count.
- After this validator is frozen, add an independent-validation section containing the validator source digest `63554301d69849c7455e2b5029eb54a8c63e12b94bd3e44ff048ea26adc67466` and the validation receipt digest.

These are provenance-hardening follow-ups, not timing gates or performance
evidence `[pcc:unverified]`.

## Reproduction

```text
rustc --edition 2021 -D warnings research/quartic_ghs_r0_r99_20261119/prototypes/commit_bound_validate.rs -o /tmp/commit_bound_validate
/tmp/commit_bound_validate --selftest . research/quartic_ghs_r0_r99_20261119/receipts/commit_bound_manifest.json research/quartic_ghs_r0_r99_20261119/receipts/commit_bound_q223.jsonl research/quartic_ghs_r0_r99_20261119/receipts/commit_bound_selftest.stdout.txt research/quartic_ghs_r0_r99_20261119/receipts/commit_bound_selftest.stderr.txt research/quartic_ghs_r0_r99_20261119/prototypes/rebuild_commit_bound.sh research/quartic_ghs_r0_r99_20261119/receipts/commit_bound_rebuild.txt
bash research/quartic_ghs_r0_r99_20261119/prototypes/rebuild_commit_bound.sh
```

Exact command output and exit statuses are retained in `receipts/commit_bound_validation.txt`.
