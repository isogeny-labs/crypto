# n47 prime F4 m4 CLI smoke result

The frozen two-cell plan (`plan.json`, protocol SHA-256
`37211b972bbd51b2463b2f4d44692b30cdbe0ff367b79d43c01c75a1bbde142c`)
was committed as `8a6f7a8748f20e658e1ab689f19b05edcd94345e` before data.
The native binary SHA-256 was
`531c94dc9626d781cb3e5e02d92488c7efd3b4118c7d81e7d2995d7286481f8e`.

`crypto dataset generate` attempted both n47 prime-curve twist cells with
`f4`, `m=4`, work limit 50,000, 30 seconds per cell, 2,048 MiB memory and
64 MiB trace caps. Both cells sealed dynamic F4 input, native, linear,
group, events, and terminal-result artifacts. Both retained `partial`
solver outcomes with null first- and last-fall-degree labels. The native
`dataset verify` passed all artifact hashes and independently replayed the
known-answer identity in both cells. It reported two completed receipts,
two known-answer-verified cells, and zero independently solved cells.
`dataset report` produced two coverage rows, 18 measurement rows, 92 artifact
rows, and two observed-cell rows; its `summary.json` SHA-256 is
`0d4aa19a98b01d0fe94168186b0be617ce18b8e729d154a39a5fdeb73d9de8bf`.
The local source run SHA-256 is
`d85b876e577efba4a45972c1580ec95c5aaf17211625123a5436c3f881b23bb3`.

The first two invocations failed at build-provenance preflight before any
cell was generated: the binary lacked `CRYPTO_BUILD_GIT_COMMIT`, then lacked
`CRYPTO_BUILD_GIT_DIRTY=false`. Rebuilding with both fields embedded made
the unchanged frozen plan runnable. The run and report remain local at
`/home/ec2-user/ecdlp/wide-prime-m4-smoke-20261101` and its `-report`
directory. Native `dataset report` explicitly records
`experiment_export_schema=false` and
`complete_protocol_export_validation_performed=false`; the general
experiment-protocol checker is absent from this checkout. These rows are a
functional CLI and replay check, with no comparative or statistical claim.

The native `dataset export` also succeeded locally. Its
`ml-dataset.training-export/v1` manifest SHA-256 is
`b7263afd5a5d2d3548b441677f1f1e9642fc9a044b38158ad9dc71fcc042b472`.
It holds two unique allowlisted feature rows and two separately restricted
target rows, both in the same n47 training partition under the whole-field-size
split. This training export is distinct from the unavailable
`experiment.export/v1` protocol export checker.
