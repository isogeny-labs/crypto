# Full-tree curve registry rebuild, 2026-10-23

This branch rebuilds the live ICV1 registry from a complete checkout after
merging main at `d7daaa6b42e7b5209e78c2407b2eaf35da25fbe4` and was
subsequently reconciled with main
`5cb553810e5bbfbd8691ec614ffcaacefc625757`. The source-bound delta checker uses the earlier
`a53a7e2bfa20080a424b9a2484a16932b6e3c116` baseline.
The builder now reads the exact `*curve_ids.json` EC1 records, including the
conductor-gap prepared lists that name their subgroup order `r`. It excludes
its own generated registry and alias index from text-name harvesting, so a
second build is byte-identical. No frozen run, measurement, or source map is
rewritten.

The catalog grows from **2,572 to 15,791 models**. The 13,219 added models
are prime-field models. The [checker](check_registry_transfer.py) binds each
one to a unique pointer in one of 152 unchanged tracked JSON source files,
recomputes its ICV1 identity and group-order certificate with independent
[arithmetic](order_arithmetic.py), and compares the exact baseline and live
alias/EC1 inventories. It rejects a deliberately wrong order. The source
[bindings](candidate-source-bindings.jsonl) and order
[certificates](candidate-order-certificates.jsonl) were first produced in
[PR #2304's source audit](https://github.com/aburan28/crypto/pull/2304) and
are included here so this branch can replay them against its exact registry.
They are stored as plain JSON Lines for review and Git delta compression.

The first-source inventory of the new models is 7,760 from the volcano
cost-v2 study, 4,583 from the volcano census, 870 from the earlier volcano
cost study, three from isogeny-volcano rho, and three from the weak-curve
leaderboard. Those are source counts, not new performance results.
Among the 2,572 preexisting models, 236 entries gain or change metadata:
216 source lists, 57 representation lists, one unresolved-EC1 status, and one
parameter record (categories overlap). No prior identity, alias mapping, or
EC1 UID is removed. The one parameter edit changes only the n73 modulus-rule
provenance text; its polynomial and curve model stay identical.
The builder now retains every distinct model and representation source rather
than truncating those lists. A merge check confirms that all 15,791 models,
their aliases, EC1 UIDs, and source references from both parents survive.

The order proof uses a point of certified order plus Hasse-interval uniqueness
for 13,216 models, and exhaustive field counts for three small models. The
[Lean theorem](../../../research/sea_point_count_20261016/lean/RegistryOrderUniqueness.lean)
checks the interval-uniqueness implication from explicit premises. Lean does
not check Python point arithmetic, source extraction, or primality of the
conductor-gap representation fields. The 298 conductor-gap generators were
separately checked on their nonsingular curves with `[r]G = O`; their source
studies retain their original primality checks. These are distinct trust
boundaries.

| Generated artifact | SHA-256 | Scope |
| --- | --- | --- |
| `registry.json` | `e39687f5d46346faa2b65ca0fa7d635afec6b01b5025951192d3fa4d1829c3cb` | 15,791 models |
| `traits.json` | `adfaf5814a41b34b6d816892f00722e422bdb548dc6379207102b3d844728baf` | 15,967 trait rows, budget 262,144; 73,577,494 bytes |
| `covers.json` | `af2b530b5f5022c4f331e35508a366691e1b720e9326f6e6e8a3999c5408ed0a` | 15,791 verified models, zero invalid |
| `cover-links.yaml` | `8707d3e522ba56a7500fd0fd3808aff3f5e598113979e2c6a2986144909eb32b` | 15,791 exact maps, 979 EC1 records |
| `jacobian-transfers.json` | `92c3a75954717c7f4248f6b19385a1d99af759fc44a2752c2d14b5d80022b5b1` | 15,791 models, 46,222 modeled routes |
| `docs/ic/leaderboard.json` | `abc6ba9e9a06e35e104f17a2f6eb2c349d4e698059dc8977faa616ac89a5d908` | main's curve roster and result status; no new performance result from this rebuild |
| `docs/browser/data.json` | `11da300c1018743f7790921234109b6feabd4f758acf3fd40961929b948e0751` | searchable catalog; 15,791 curves, 240 sessions, 14,830 yields |

The latest main's trait cost weighting changes 13 conductor entries from the
PR's earlier generated traits, only in their endomorphism resolution and
grouping keys. The curve roster, order fields, and source digests are unchanged;
the current trait digest above is for the exact merged builder.

The cover geometry and Jacobian inventory are regenerated on the expanded
catalog. The 2026-10-19 cover-transfer assessment, PDF, and figures remain a
**historical 2,572-model snapshot**; they do not certify transfer for new
models. `scripts/check_historical_cover_transfer_assessment.mjs` replays that
assessment on its exact `a53a7e2...` inputs and compares it with the published
artifact. The current browser shows the new cover geometry, not an inferred
transfer speedup. Relative to this PR's earlier browser artifact, the merge
incorporates six leaderboard annotations and two ecbench annotations on curve
rows. It also indexes 14 sessions and 182 yield records from the already
tracked `research/ecbench_allocator_confound_20261023/sessions/` tree; method
and factor-base counters follow those records. No benchmark was run by this
rebuild, and it makes no new performance claim.

Local replay:

```sh
python3 scripts/build_curve_registry.py --check
python3 scripts/check_curve_names.py
python3 -O docs/curves/registry-rebuild-20261023/check_registry_transfer.py
cargo run --release --bin curve_traits -- check
cargo run --release --bin curve_cover_check -- --check
cargo run --release --bin jacobian_transfers -- check
python3 scripts/build_ic_leaderboard.py --check
python3 scripts/build_lab_browser.py --check
node scripts/check_historical_cover_transfer_assessment.mjs
bash research/sea_point_count_20261016/formal/verify.sh
```

On exact main `5cb5538`, the leaderboard check remains a baseline gate
failure: six rule-v7 Table A rows have neither an independent evidence
manifest nor a pre-gate digest. The
[problem record](../../../problems/rule-v7-leaderboard-independent-validation.md)
tracks the separate validation work. [PR #2512](https://github.com/aburan28/crypto/pull/2512)
now contains independent v6/v7 evidence and a locally passing gate, but hosted
checks and merge are pending. This registry rebuild does not bypass the gate
or treat those six rows as validated on its `5cb5538` baseline.

The generated `cover-links.yaml` is 44,446,725 bytes. It is JSON-compatible
YAML but does not fall under the repository's `docs/curves/*.json` large-text
exception. The PR therefore needs the owner/reviewer `large-file-ok` label
before its large-file gate can pass. Its Git history cost and downstream
catalog growth should be reviewed explicitly.
