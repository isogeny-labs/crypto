# Frozen Voronoi boundary EC candidate selection control, proposal 14

Frozen 2026-11-24 (America/Los_Angeles) before this producer run, draft PR #5165. Proposal #14 prioritizes Voronoi boundary vertices as a relation sieve. Here the candidate domain is the exact ordered triple grid of the completed [S4 V2 run](S4_MATCHED_REPORT.md), SHA-256 of its stdout `3accec218c07a2acd2fb433270fa8724db6aee57fef85d0e29e3eb3de16c1f7f`. That run used `E/F_1009:y²=x³+x+14`, a fixed 16-point factor base, on-curve target abscissae 0 and 320, and checked every algebraic S4 zero against signed EC point sums. Its producer labels remain `[pcc:unverified]` pending independent replay.

Map each candidate triple of factor-base indices `(i,j,k)∈{1,…,16}³` to the integer cube. Choose two fixed Euclidean Voronoi sites `(1,1,1)` and `(16,16,16)`. The exact equal-distance plane has index sum 25.5 and no lattice point; define a fixed one-cell-thick boundary slab by `24≤i+j+k≤27`. Score all 4096 candidates with two squared-distance calculations, represented by six coordinate squared-difference proxies per candidate. A matched-size control takes the first `N` grid indices under SplitMix64 hash of zero-based row-major triple ID xor fixed seed `0x1420261124`, where `N` is the boundary count. Sort by unsigned hash then ID. The hash arm receives the same number of exact S4 evaluations as the boundary arm. This baseline is chosen without relation labels. Record exact signed hit triples and rank of selected 17-column rows over `F_1013` for each target, and retain the full-grid rank. The implementation parses the original witness rows, checks target sizes 24/33, exact index/sign-to-row consistency, and uniqueness. Rank is computed by modular Gaussian elimination.

This is a **partial EC candidate-selection adaptation** of Voronoi prioritization. Factor-base-index Euclidean proximity has no established connection to EC relation density, and selecting witness rows from frozen full-grid labels does not measure S4 evaluation wall time or an online solver. It does test the original geometric boundary selection rule against a same-count candidate baseline, keeping target identity and rank visible.

The checkpoint gate still fails (`.resume/RESUME.md` missing; earlier S3 `InvalidAccessKeyId`); the user's renewed instruction and explicit local bypass admit this public local analysis only. No external target or cloud cost. Stop on input hash mismatch, compiler/run error, failed assertion, missing sentinel, or stderr; preserve all streams and exit. No silent retry. Frozen native Rust source SHA-256: `217ebb17a1fa1b02431c8f243e247b4269633d6a0a6390da999f601a4c76553f`. The input SHA was rechecked before compile and remained `3accec218c07a2acd2fb433270fa8724db6aee57fef85d0e29e3eb3de16c1f7f`. Invoke once:

```sh
rustup run 1.99.1 rustc -O --edition 2021 research/prime_field_100_ideas_20261123/EC_VORONOI_014.rs -o /tmp/ec_voronoi_014.bin
/tmp/ec_voronoi_014.bin research/prime_field_100_ideas_20261123/S4_MATCHED_V2_STDOUT.txt > research/prime_field_100_ideas_20261123/EC_VORONOI_014.stdout.txt 2> research/prime_field_100_ideas_20261123/EC_VORONOI_014.stderr.txt
```

The binary is local-only and must not be staged. Save compiler and run statuses in `EC_VORONOI_014.exit.txt`; report source/binary/raw hashes and findings below. No result is admitted at protocol freeze.

## One-shot local result

Rust 1.99.1 compiled the frozen source once; compile and run statuses were both `0`, stderr was empty, and the completion sentinel appeared. The two-site slab selected 764 of 4096 ordered triples; the fixed unsigned-hash arm selected 764. The parser checked all 57 signed witness rows against their factor-base index/sign row encoding and the expected target counts. Results are secondary producer analyses of the frozen S4 labels `[pcc:unverified]`.

| Target abscissa | Arm | Candidates | Exact signed witness triples | Rank over `F_1013` |
|---:|---|---:|---:|---:|
| 0 | Voronoi boundary | 764 | 0 | 0 |
| 0 | Matched hash | 764 | 2 | 2 |
| 0 | Full grid | 4096 | 24 | 4 |
| 320 | Voronoi boundary | 764 | 12 | 2 |
| 320 | Matched hash | 764 | 3 | 2 |
| 320 | Full grid | 4096 | 33 | 6 |

The boundary slab had no hits for one target and more hits than the fixed hash sample for the other, with equal selected rank on the latter. These two target outcomes do not support a stable boundary advantage. The boundary test charges six squared-distance coordinate proxies on each of 4096 candidates before 764 S4 checks, whereas the hash control needs one hash per candidate plus selection sorting; neither cost is a measured GP decomposition runtime. The full-grid rank remains a ceiling for these subsets. Input S4 labels, including their independent correctness review, remain a separate evidence gate.

| Preserved artifact | SHA-256 |
|---|---|
| [Source](EC_VORONOI_014.rs) | `217ebb17a1fa1b02431c8f243e247b4269633d6a0a6390da999f601a4c76553f` |
| [Stdout](EC_VORONOI_014.stdout.txt) | `7e1e22a4fa54f55e69b97e9bdc086dc6637c4fa79537e41a68ef0b8a5cb621e0` |
| [Stderr](EC_VORONOI_014.stderr.txt) (empty) | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| [Exit record](EC_VORONOI_014.exit.txt) | `6ae53545e4b9562543543c003678fd596b448a77bc6940dfe87010b843763960` |
| `/tmp/ec_voronoi_014.bin` (local only) | `cc6dadbbd8b3e13422fa06e843a450240f720b5fa26666061af68bd6d3129ecb` |
