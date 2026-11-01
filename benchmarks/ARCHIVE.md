# Unvalidated benchmark archive

This archive retains dated benchmark summaries that are not admitted by the
results table in [`README.md`](README.md). Each entry in
[`ARCHIVE.json`](ARCHIVE.json) stores the exact former row text, its SHA-256,
the disposition and reason for removing it from the admitted table, and paths
to the supporting study records. The archived wording is historical evidence;
it is not a validation receipt and does not establish the truth of its numeric
claims.

Allowed dispositions are `incomplete`, `censored`, `documentary`, and
`superseded`. The validation gate checks that the row is absent from the
admitted table, the digest matches the exact stored text, and the cited source
files exist. For pull requests it also requires a disposition for every row
removed from the base table and prevents prior archive entries from being
deleted or changed. An archived row that remains in the admitted table still
fails the normal validation gate.

## Archived entries

These rows remain reviewable with their original text in [ARCHIVE.json](ARCHIVE.json). A source link confirms provenance, not independent validation.

| Entry | Disposition | Source record |
|:--|:--|:--|
| [20261020-ic-oracle-setup-blocked](20261020-ic-oracle-setup-blocked/) | incomplete | [source](../research/ic_oracle_setup_blocked_20261020/README.md) |
| [20261019-ic-matched-rho-v8](20261019-ic-matched-rho-v8/) | incomplete | [source](../research/ic_tool_program/rule/v8/README.md) |
| [20261017-two-sylow-fp64](20261017-two-sylow-fp64/) | incomplete | [source](../research/two_sylow_fp64_20261017/README.md) |
| [20261014-ic-program-paper](20261014-ic-program-paper/) | documentary | [source](../papers/ecdlp-index-calculus-program/README.md) |
| [20261019-pdp-split-closure-ecbench](20261019-pdp-split-closure-ecbench/) | censored | [source](../research/pdp_split_slots_20261012/README.md) |
| [20261019-factor-base-designs-ek1](20261019-factor-base-designs-ek1/) | incomplete | [source](../research/factor_base_designs_20261015/README.md) |
| [20261017-safecurves-trapdoor-slots](20261017-safecurves-trapdoor-slots/) | documentary | [source](../research/safecurves_trapdoor_slots_20261017/PROTOCOL.md) |
| [20261019-adaptive-pair-cells](20261019-adaptive-pair-cells/) | incomplete | [source](../research/autolab_cycle_20261019_v3/REPORT.md) |
| [20261018-factor-base-shape-leads](20261018-factor-base-shape-leads/) | incomplete | [source](../research/factor_base_shape_leads_20261018/README.md) |
| [20261017-rootfinding-paper-synthesis](20261017-rootfinding-paper-synthesis/) | documentary | [source](../papers/norm-rootfinding/README.md) |
| [20261017-rootfinding-native-followup](20261017-rootfinding-native-followup/) | documentary | [source](../research/rootfinding_publication_20261017_v2/README.md) |
| [20261017-ic-a20-build-line-streams](20261017-ic-a20-build-line-streams/) | incomplete | [source](../research/ic_tool_program/explorations/A20-build-line-streams-20261017/README.md) |
| [20261017-ic-a19-filter-pages-build-parts](20261017-ic-a19-filter-pages-build-parts/) | incomplete | [source](../research/ic_tool_program/explorations/A19-filter-pages-build-parts-20261017/README.md) |
| [20261017-ic-a18-wider-base](20261017-ic-a18-wider-base/) | incomplete | [source](../research/ic_tool_program/explorations/A18-wider-base-2p47-20261017/README.md) |
| [20261017-ic-a17-filter-huge-pages](20261017-ic-a17-filter-huge-pages/) | incomplete | [source](../research/ic_tool_program/explorations/A17-filter-huge-pages-20261017/README.md) |
| [20261017-ic-a16-huge-pages](20261017-ic-a16-huge-pages/) | incomplete | [source](../research/ic_tool_program/explorations/A16-huge-pages-20261017/README.md) |
| [20261017-ic-a15-filter-slots](20261017-ic-a15-filter-slots/) | incomplete | [source](../research/ic_tool_program/explorations/A15-filter-slots-20261017/README.md) |
| [20261017-ic-scan-stages-v8](20261017-ic-scan-stages-v8/) | incomplete | [source](../research/ic_tool_program/explorations/scan-stages-v8-20261017/README.md) |
| [20261017-ic-a14-prefetch-size-gate](20261017-ic-a14-prefetch-size-gate/) | incomplete | [source](../research/ic_tool_program/explorations/A14-prefetch-size-gate-20261017/README.md) |
| [20261017-ic-r12-admitted-prefetch](20261017-ic-r12-admitted-prefetch/) | incomplete | [source](../research/ic_tool_program/rounds/R12-admitted-prefetch/README.md) |
| [20261016-ic-r11-build-streams](20261016-ic-r11-build-streams/) | incomplete | [source](../research/ic_tool_program/rounds/R11-build-streams/README.md) |
| [20261016-ic-r08-scan-fold](20261016-ic-r08-scan-fold/) | incomplete | [source](../research/ic_tool_program/rounds/R08-scan-fold/README.md) |
| [20261016-ic-r06-scan-key](20261016-ic-r06-scan-key/) | incomplete | [source](../research/ic_tool_program/rounds/R06-scan-key/README.md) |
| [20261016-ic-r10-main-head](20261016-ic-r10-main-head/) | incomplete | [source](../research/ic_tool_program/rounds/R10-main-head/README.md) |
| [20261016-n83-prebuilt-factor-base](20261016-n83-prebuilt-factor-base/) | incomplete | [source](../research/n83_prebuilt_factor_base_20261016/PROTOCOL.md) |
| [20261018-pdp-ps3-bitsliced](20261018-pdp-ps3-bitsliced/) | incomplete | [source](../research/pdp_oracle_speedup_20261011/ps3_bitsliced/README.md) |
