# Natural-query gate validation ledger

| Gate | State | Receipt or remaining work |
| --- | --- | --- |
| Protocol before implementation and execution | committed as `8ed3cb05eb` | [frozen protocol](PROTOCOL.md) |
| Reused compiled worker source and binary | six compiled files and Linux binary hashes pass, including review-head recheck against the retained snapshot | [source check](source-check.log), [binary check](binary-check.log), [source recheck](source-recheck-final.log), [binary recheck](binary-recheck-final.log), [preceding build receipt](../n83_k16627_m4_selector_only_20261018/BUILD_RECEIPT.md) |
| Five retained panel inputs | SHA-256 check passes, including review-head recheck | [panel manifest](panel-inputs.sha256), [check](panel-check.log), [recheck](panel-recheck-final.log) |
| Public target and validation sidecar | SHA-256 check passes, including review-head recheck | [fixture manifest](fixture-inputs.sha256), [check](fixture-check.log), [recheck](fixture-recheck-final.log) |
| Natural-query container wrapper | shell syntax checked; no degree-83 invocation yet | [run-natural.sh](run-natural.sh) |
| P/A1/B/A2 sequence | pending a quiet measurement host; no arm invoked | [timestamped host-availability record](HOST_AVAILABILITY.md); fresh outputs, control replays and outcome still owed |
| Full release library on final stacked review head | blocked before tests: exit 101, 26 inherited ecbench/quartic source errors after tracked local dependencies and fixtures were materialized | [final stacked log](cargo-release-lib-stacked.log), [initial sparse-checkout diagnostic](cargo-release-lib-stacked-sparse.log), [open build problem](../../problems/current-main-library-build-20261017.md) |
| Touched examples on natural-query delta | none; the delta adds a shell wrapper, manifests and protocol, with no Rust source changes | [base PR #2202 validation](../n83_k16627_m4_selector_only_20261018/VALIDATION.md) |
| CI scope syntax on stacked review head | blocked by the inherited duplicate `root` declaration | [raw log](ci-scope-stacked.log) |

The source and input checks establish identity only. They do not establish a natural relation, rank, runtime comparison or a factor-base selection. An internal or outer cap will be retained as censored with its durable files and explicit missing-receipt status.

The final stacked review head differs from the retained worker snapshot in a parent-integrated library file. This protocol invokes the exact frozen Linux worker binary and binds it to the six-file source snapshot; it does not claim that the merged review head can currently rebuild that binary. The initial stacked Cargo attempt also found one tracked standards JSON omitted by sparse checkout, which was materialized before the 26-error rerun.
