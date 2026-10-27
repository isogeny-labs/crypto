# Primary N83 K16627 four-summand orbit-chain diagnostic

Freeze one source-pinned natural-query cell after the K16627 object passed
construction and generic replay. Use the exact `a=0`, `public_x_hash`,
seed `2026100801`, 16,627-column object with compressed BLAKE3
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`.
Use the K4096 control's source commit
`f61b33c1b1bb98efa22177a615270c160859a605` and the same Linux arm64
exporter SHA-256
`41c968ee8d82d085b8f9eca03ddcbfed69805e415408f36ddc1bcc58ccdeb898`.
The fixed public target is fixture 0 in the retained K4096 fixture pair:
`probe-corpus.json` SHA-256
`760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`
and `probe-validation.json` SHA-256
`1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f`.
Copy these sidecars into the external panel/fixture directory without
changing the manifest or compressed object.

First run `primary-orbit-chain-build` for `K=16627`, `m=4`, identity mask
`0`, maximum 1,000,000 variables and 12,000,000 domain clauses. Use the
existing 6 GiB hard Docker memory limit, zero swap, one CPU, no network,
read-only repository, and a 300-second outer wall cap. Preserve the model
receipt, stdout/stderr, exact command, exit and container cleanup. A cap,
OOM or producer failure remains a terminal diagnostic and stops this card.

Only if the model build passes, run one `primary-orbit-chain-cold` natural
query on that same public point with one trial, 32 maximum models, 1,000
conflicts, the same variable/domain caps and memory envelope, and 600-second
worker and 660-second outer wall caps. Preserve the full status and counters,
including zero model/relation/rank cells and censored terminals. There are
no automatic retries and no promotion from a planted or bounded control.

This card tests model feasibility and one bounded natural attempt at the
four-summand support threshold. The existing K4096 five-summand trial is
context, not a paired runtime baseline. A complete factor-base selection
requires useful rank, matrix and target stages, independent scalar replay,
all preparation costs, and a same-target online IC/rho pair under its
declared tier. Keep `selected_best_total_runtime` null here.
