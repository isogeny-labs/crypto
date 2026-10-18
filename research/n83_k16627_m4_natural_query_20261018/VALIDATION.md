# Natural-query gate validation ledger

| Gate | State | Receipt or remaining work |
| --- | --- | --- |
| Protocol before implementation and execution | committed as `8ed3cb05eb` | [frozen protocol](PROTOCOL.md) |
| Reused compiled worker source and binary | six compiled files and Linux binary hashes pass | [source check](source-check.log), [binary check](binary-check.log), [preceding build receipt](../n83_k16627_m4_selector_only_20261018/BUILD_RECEIPT.md) |
| Five retained panel inputs | SHA-256 check passes | [panel manifest](panel-inputs.sha256), [check](panel-check.log) |
| Public target and validation sidecar | SHA-256 check passes | [fixture manifest](fixture-inputs.sha256), [check](fixture-check.log) |
| Natural-query container wrapper | shell syntax checked; no degree-83 invocation yet | [run-natural.sh](run-natural.sh) |
| P/A1/B/A2 sequence | pending other library suites and a quiet measurement host | fresh outputs, control replays and outcome still owed |
| Full release library and touched examples on final source | pending | required before push |

The source and input checks establish identity only. They do not establish a natural relation, rank, runtime comparison or a factor-base selection. An internal or outer cap will be retained as censored with its durable files and explicit missing-receipt status.
