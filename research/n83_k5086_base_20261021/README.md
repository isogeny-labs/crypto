# N83 K5086 exact factor base

The deterministic N83 orbit constructor produced **844,276 distinct subgroup-usable points** in 5,086 signed Frobenius columns. Independent replay checked the curve and subgroup parameters, every representative's subgroup membership, all orbit coordinates and labels, closure, and global distinctness. The [prefix receipt](prefix_check_k5500.json) confirms that every coordinate, label, and representative is the exact ordered prefix of the independently replayed K5500 base.

| Field | Recorded value |
| --- | --- |
| Curve | `EC1N83Ckb1a1haacb69ee9f37`; binary `y² + xy = x³ + x² + 1`, subgroup order `8569786107849059`, cofactor `1128547018` |
| Actual usable points / folded columns | `844276` / `5086` |
| Scanned abscissae | `10227` |
| Producer base hash, BLAKE3 | `78f6b15d96f67fed5854e28b6737a95d16a74874da8e7520a3ca19483834b22d` |
| Ordered dump SHA-256, before compression | `4d185d734df771376c6822332db9f021ded0994a5bef6e5e0a0e19261d21d0b6` |
| Archive SHA-256 | `6b854238918a56d67a4a1b10144d33629ad2fb47382df2888afb0b456f2ad1d9` |
| Source / executable SHA-256 | `8c4f90114ed00c7c188f50908b177063fa03faefa646f7c0dfe08e87bb39c015` / `5c7d4c3b23989d0ca1d3bf5a4426c12c9560bba44e69699af07f45e410e43487` |
| Bounded process | `PASS_PROCESS`, 9.571 s wall, 444,923,904 bytes sampled peak RSS, 600 s and 8 GiB caps |
| Group and prefix replay | `PASS_INDEPENDENT_BASE_REPLAY` / `PASS_EXACT_PREFIX` |

The process used the frozen [runner](runner.py), [input](input.json), [source snapshot](source_snapshot.rs), and [executable snapshot](executable_snapshot), with raw [stdout](stdout.jsonl), [process receipt](process_result.json), [base archive](base_dump.jsonl.gz), and [verification](verification.json). The source and executable links point to byte-identical snapshots in the K5500 run. The archive was checked with `gzip -t`; independent replay was repeated from the archived copy using [`verify_base_stage.py`](../n83_two_tier_root_20261021/verify_base_stage.py), followed by [`verify_base_prefix.py`](../n83_two_tier_root_20261021/verify_base_prefix.py). The [machine-readable base record](base_record.json) keeps the candidate ID and unmeasured end-to-end costs null.

The producer's `base_construction_ms` is 5,462.290 ms. Its timer starts before construction and is read after serializing and writing the base dump; the 9.571 s child wall additionally includes process startup. These shared-host intervals are construction diagnostics. Root-index memory, relation rank, natural yield, final linear algebra, one-target cold total, and same-point online IC/rho remain unmeasured for this exact candidate.

The sealed [K5086 comparison](../n83_two_tier_root_20261021/MAXK_PROTOCOL.md) uses an earlier source digest. This current-source base replay establishes exact point membership and prefix structure; the sealed comparison still requires its own source-bound stage receipt and complete Tier 1 runs before it can select a factor base.
