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
| [20261012-isogeny-fiber-paired](20261012-isogeny-fiber-paired/) | incomplete | [source](../benchmarks/20261012-isogeny-fiber-paired/README.md) |
| [20261013-gaudry-quartic-d4](20261013-gaudry-quartic-d4/) | incomplete | [source](../research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md) |
| [20261013-isogeny-fiber-full-cost](20261013-isogeny-fiber-full-cost/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/full_cost_20261013/REPORT.md) |
| [20261013-isogeny-iso-control](20261013-isogeny-iso-control/) | incomplete | [source](../benchmarks/20261013-isogeny-iso-control/README.md) |
| [20261013-isogeny-nonisomorphic](20261013-isogeny-nonisomorphic/) | incomplete | [source](../benchmarks/20261013-isogeny-nonisomorphic/README.md) |
| [20261013-koblitz-higher-arity](20261013-koblitz-higher-arity/) | incomplete | [source](../benchmarks/20261013-koblitz-higher-arity/README.md) |
| [20261013-nist-gpu-rho-ladder](20261013-nist-gpu-rho-ladder/) | incomplete | [source](../benchmarks/ARCHIVE.md) |
| [20261014-binary-genus2-speed](20261014-binary-genus2-speed/) | incomplete | [source](../benchmarks/20261014-binary-genus2-speed/README.md) |
| [20261014-binary-genus2-speed-eight-targets](20261014-binary-genus2-speed-eight-targets/) | incomplete | [source](../benchmarks/20261014-binary-genus2-speed-eight-targets/README.md) |
| [20261014-binary-genus2-speed-followup](20261014-binary-genus2-speed-followup/) | incomplete | [source](../benchmarks/20261014-binary-genus2-speed-followup/README.md) |
| [20261014-binary-genus2-speed-seven-targets](20261014-binary-genus2-speed-seven-targets/) | incomplete | [source](../benchmarks/20261014-binary-genus2-speed-seven-targets/README.md) |
| [20261014-torsion-extension](20261014-torsion-extension/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/extension_field_20261014/REPORT.md) |
| [20261015-different-j-order-batch](20261015-different-j-order-batch/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/different_j_order_batch_20261015/RESULTS.md) |
| [20261015-isogeny-fiber-four-order-trend](20261015-isogeny-fiber-four-order-trend/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/RESULTS_V2.md) |
| [20261015-isogeny-fiber-order1201](20261015-isogeny-fiber-order1201/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/order1000/RESULTS.md) |
| [20261015-isogeny-fiber-portable-trend](20261015-isogeny-fiber-portable-trend/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/RESULTS_V1.md) |
| [20261015-isogeny-fiber-price-sensitivity](20261015-isogeny-fiber-price-sensitivity/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/sensitivity/RESULTS.md) |
| [20261015-torsion-pairing-transfer](20261015-torsion-pairing-transfer/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/pairing_transfer_20261015/RESULTS.md) |
| [20261016-chi-heldout-five-arm](20261016-chi-heldout-five-arm/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/chi_polynomial_20261015/different_j_resultant_20261016/HELDOUT_RESULTS_V2.md) |
| [20261016-chi-heldout-nine-arm](20261016-chi-heldout-nine-arm/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/chi_polynomial_20261015/different_j_resultant_20261016/HELDOUT_RESULTS_V4.md) |
| [20261016-chi-heldout-seven-arm](20261016-chi-heldout-seven-arm/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/chi_polynomial_20261015/different_j_resultant_20261016/HELDOUT_RESULTS_V3.md) |
| [20261016-chi-resultant-exactness](20261016-chi-resultant-exactness/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/chi_polynomial_20261015/different_j_resultant_20261016/RESULTS_V1.md) |
| [20261016-inverse-velu-four-order](20261016-inverse-velu-four-order/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/inverse_velu/trend_four_cell_v1/TREND_RESULTS_V1.md) |
| [20261016-isogeny-fiber-five-price](20261016-isogeny-fiber-five-price/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/sensitivity_five/RESULTS.md) |
| [20261016-isogeny-fiber-linux-host](20261016-isogeny-fiber-linux-host/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/linux_host_v1/README.md) |
| [20261016-isogeny-fiber-order1201-linux](20261016-isogeny-fiber-order1201-linux/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/linux_host_order1201_v1/README.md) |
| [20261016-isogeny-fiber-primitive-price](20261016-isogeny-fiber-primitive-price/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/primitive_price_20261016/RESULTS_V1.md) |
| [20261016-isogeny-inverse-velu](20261016-isogeny-inverse-velu/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/inverse_velu/RESULTS.md) |
| [20261016-isogeny-round5](20261016-isogeny-round5/) | incomplete | [source](../benchmarks/ARCHIVE.md) |
| [20261017-isogeny-volcano-cost-v2](20261017-isogeny-volcano-cost-v2/) | incomplete | [source](../benchmarks/20261017-isogeny-volcano-cost-v2/VALIDATION.json) |
| [20261018-cofactor-cleared-ic](20261018-cofactor-cleared-ic/) | incomplete | [source](../benchmarks/20261018-cofactor-cleared-ic/README.md) |
| [20261018-ecbench-comprehensive-sweep](20261018-ecbench-comprehensive-sweep/) | incomplete | [source](../research/ecbench_comprehensive_sweep_20261018/README.md) |
| [20261018-f4-early-trace-coset](20261018-f4-early-trace-coset/) | incomplete | [source](../benchmarks/20261018-f4-early-trace-coset/README.md) |
| [20261018-f4-gf2-port](20261018-f4-gf2-port/) | incomplete | [source](../benchmarks/20261018-f4-gf2-port/README.md) |
| [20261018-pkm-rootfind-prime](20261018-pkm-rootfind-prime/) | incomplete | [source](../benchmarks/20261018-pkm-rootfind-prime/VALIDATION.json) |
| [20261018-s4-oracle-cost](20261018-s4-oracle-cost/) | incomplete | [source](../research/s4_oracle_cost_20261018/RESULTS.md) |
| [20261018-sixsum-s4-resolvent](20261018-sixsum-s4-resolvent/) | incomplete | [source](../benchmarks/ARCHIVE.md) |
| [20261018-three-leads-audit](20261018-three-leads-audit/) | incomplete | [source](../benchmarks/20261018-three-leads-audit/README.md) |
| [20261018-three-leads-e2e](20261018-three-leads-e2e/) | incomplete | [source](../benchmarks/20261018-three-leads-e2e/README.md) |
| [20261018-wide-producer-engineering](20261018-wide-producer-engineering/) | incomplete | [source](../benchmarks/ARCHIVE.md) |
| [20261019-t20-n73-k600-e2e](20261019-t20-n73-k600-e2e/) | incomplete | [source](../benchmarks/ARCHIVE.md) |
| [20261020-t21-wide-producer-arithmetic](20261020-t21-wide-producer-arithmetic/) | incomplete | [source](../benchmarks/ARCHIVE.md) |
| [20261021-t22-inline-clmul](20261021-t22-inline-clmul/) | incomplete | [source](../benchmarks/ARCHIVE.md) |
| [20261022-deep-volcano-prime-gap](20261022-deep-volcano-prime-gap/) | documentary | [source](../benchmarks/20261022-deep-volcano-prime-gap/VALIDATION.json) |
| [20261022-kummer-paired-replay](20261022-kummer-paired-replay/) | incomplete | [source](../benchmarks/20261022-kummer-paired-replay/VALIDATION.json) |
| [20261022-pencil-degree-peeling](20261022-pencil-degree-peeling/) | incomplete | [source](../benchmarks/20261022-pencil-degree-peeling/VALIDATION.json) |
| [20261022-projective-law](20261022-projective-law/) | incomplete | [source](../benchmarks/20261022-projective-law/VALIDATION.json) |
| [20261022-strong-rho-batchinv](20261022-strong-rho-batchinv/) | incomplete | [source](../benchmarks/20261022-strong-rho-batchinv/VALIDATION.json) |
| [20261023-volcano-census-followup](20261023-volcano-census-followup/) | incomplete | [source](../benchmarks/20261023-volcano-census-followup/VALIDATION.json) |
| [20261028-ic-residual-sweep](20261028-ic-residual-sweep/) | incomplete | [source](../benchmarks/20261028-ic-residual-sweep/VALIDATION.json) |
| [20261028-ic-sweep-ext](20261028-ic-sweep-ext/) | incomplete | [source](../benchmarks/20261028-ic-sweep-ext/VALIDATION.json) |
| [20261028-ic-wide-batching](20261028-ic-wide-batching/) | incomplete | [source](../benchmarks/20261028-ic-wide-batching/VALIDATION.json) |
| [20261028-ic-wide-ext](20261028-ic-wide-ext/) | incomplete | [source](../benchmarks/20261028-ic-wide-ext/VALIDATION.json) |
| [20261029-ic-dg-blocked](20261029-ic-dg-blocked/) | incomplete | [source](../benchmarks/20261029-ic-dg-blocked/VALIDATION.json) |
| [20261029-ic-dg-wall](20261029-ic-dg-wall/) | incomplete | [source](../benchmarks/20261029-ic-dg-wall/VALIDATION.json) |
| [20261029-ic-packed-filter](20261029-ic-packed-filter/) | incomplete | [source](../benchmarks/20261029-ic-packed-filter/VALIDATION.json) |
| [20261029-ic-srenew-deferred-guard](20261029-ic-srenew-deferred-guard/) | incomplete | [source](../benchmarks/20261029-ic-srenew-deferred-guard/VALIDATION.json) |
| [20261029-ic-stride-renew-prime](20261029-ic-stride-renew-prime/) | incomplete | [source](../benchmarks/20261029-ic-stride-renew-prime/VALIDATION.json) |
| [20261029-icb-large-prime](20261029-icb-large-prime/) | incomplete | [source](../benchmarks/20261029-icb-large-prime/VALIDATION.json) |
| [20261031-pdp-ps6-charge](20261031-pdp-ps6-charge/) | incomplete | [source](../benchmarks/20261031-pdp-ps6-charge/VALIDATION.json) |
| [20261114-k1-pair-table-dimension](20261114-k1-pair-table-dimension/) | incomplete | [source](../benchmarks/20261114-k1-pair-table-dimension/VALIDATION.json) |
| [20261114-wallfix-runpod-abab](20261114-wallfix-runpod-abab/) | incomplete | [source](../benchmarks/20261114-wallfix-runpod-abab/VALIDATION.json) |
| [20261115-isogeny-dual-projected](20261115-isogeny-dual-projected/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/dual_projected_base_20261115/RESULTS_V1.md) |
| [20261115-isogeny-fiber-grid-v2](20261115-isogeny-fiber-grid-v2/) | incomplete | [source](../research/isogeny_torsion_fibers_20261012/factor_base_grid_20261016/RESULTS_V2.md) |
| [20261115-ksum-prune-tableb](20261115-ksum-prune-tableb/) | incomplete | [source](../benchmarks/20261115-ksum-prune-tableb/VALIDATION.json) |
| [20261115-tableb-binary-ksum](20261115-tableb-binary-ksum/) | incomplete | [source](../benchmarks/20261115-tableb-binary-ksum/VALIDATION.json) |
| [20261115-tableb-ksum](20261115-tableb-ksum/) | incomplete | [source](../benchmarks/20261115-tableb-ksum/VALIDATION.json) |
