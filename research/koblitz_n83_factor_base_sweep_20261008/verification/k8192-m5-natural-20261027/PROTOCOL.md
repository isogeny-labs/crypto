# Primary N83 K8192 five-summand natural diagnostic

Freeze one `a=0`, `public_x_hash`, seed `2026100801` K8192 object and one
previously fixed public fixture-0 target. The object has 8,192
signed-Frobenius columns, 1,359,872 points and compressed BLAKE3
`43419f2ec08e1a7f15bcfb9dfcceac6ca386869a0eea0bb04d3934af621b3f8d`.
The source is the buildable K4096 control commit
`f61b33c1b1bb98efa22177a615270c160859a605`, with Linux arm64 exporter
SHA-256 `41c968ee8d82d085b8f9eca03ddcbfed69805e415408f36ddc1bcc58ccdeb898`.
Use the retained public fixture pair: `probe-corpus.json` SHA-256
`760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`
and `probe-validation.json` SHA-256
`1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f`.
Copy those sidecars into the external panel; preserve the frozen manifest,
compressed object and replay receipt.

Build one `m=5`, identity-mask-zero orbit-chain-S3 model with maximum
400,000 variables and 6,000,000 domain clauses. Run under one CPU, a 6 GiB
hard Docker memory cap, zero swap, no network, read-only source and a
300-second outer wall cap. Retain the exact command, output, status, raw
logs and container cleanup. A cap, OOM or producer failure stops the card.

Only after the model passes, run `primary-orbit-chain-cold` on fixture 0
with one natural trial, 32 maximum models, 1,000 conflicts, the same
variable/domain/memory caps, 600-second worker budget and 660-second outer
wall cap. Preserve all completed and censored counters. Do not retry. A
verified relation or scalar must pass the independent replay checks before
being counted. The prior K4096 m5 capped results are contextual; this
single K8192 run cannot establish a yield rate or runtime ranking.

The selection gate remains the complete one-target IC pipeline and
same-point rho under a declared benchmark tier, with setup, target work,
final matrix and correctness costs separated. Keep the selected total
runtime null unless all stages have been observed and verified.
