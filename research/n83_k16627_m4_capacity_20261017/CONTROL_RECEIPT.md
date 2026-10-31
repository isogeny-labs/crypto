# Native controls before the capacity worker

The release library test executable SHA-256 is
`fc4a74c3555e1334298a89284536a1dde6ead5730f10b3f18e8e679a167ad020`.
It was built from the adapter branch with the m=4 guard at `8adde7c049`;
the only subsequent committed change before these controls was the build
receipt. Each exact test was invoked directly on that compiled release test
binary, before any K=16,627 capacity worker:

| Exact native test | Outcome | Log |
| --- | --- | --- |
| `cryptanalysis::koblitz_index_calculus::tests::chained_s3_strategy_feeds_verified_relation_rank_and_target_solve` | PASS, one test; exercises m=4 and m=5 on its checked small-field fixture | [control 1](control-1.log) |
| `cryptanalysis::binary_semaev_chain_sat::tests::source_bound_covers_degree_83_without_constructing_a_model` | PASS, one test; checks the degree-83 m=4 source-variable bound | [control 2](control-2.log) |
| `cryptanalysis::binary_semaev_chain_sat::tests::frobenius_orbit_preflight_caps_before_mutation_and_counts_n83` | PASS, one test; checks degree-83 m=4 orbit-domain counts and cap behavior | [control 3](control-3.log) |

The exporter release example suite independently passed 19/19, as recorded
in [BUILD_RECEIPT.md](BUILD_RECEIPT.md). The repository-wide release
library suite was still running when these focused controls completed; its
final log and verdict will be recorded separately before this branch is
pushed. The focused controls validate the core at their stated fixture
sizes. They do not establish a K=16,627 degree-83 relation or rank.
