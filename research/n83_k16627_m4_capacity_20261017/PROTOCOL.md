# Frozen N83 K=16,627 four-summand capacity gate

**Status:** preregistered before adapter changes or worker execution. This
protocol is committed first and remains unchanged after outcomes. The run
receipts and interpretation are appended in later commits.

## Question, reference and hypothesis

Can the retained primary `GF(2^83)` signed-Frobenius base with K=16,627
columns be imported and used to construct a full-width four-summand
chained-S3 model under a hard 4 GiB memory cap? The base is the exact object
in `research/n83_k16627_prebuild_20261017/`, compressed BLAKE3
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`,
16,627 columns and 2,760,082 point records. Its construction, generic
replay and S3 custody are separate prior gates in PR #2124. The current
verified primary retained-base construction reference is K=1,182 from the
N83 sweep; the earlier K=2,048 orbit-domain capacity receipt is a distinct
smaller-base diagnostic. Neither supplies a matched K=16,627 m=4 result.

Hypothesis: after widening only the primary adapter's chained-S3 arity
guards from m=5..6 to m=4..6, the existing checked m=4 core can build one
identity-mask-zero orbit-domain model for this object within the envelope.
The four-summand core at clean commit `2b1d7183ab` is the implementation
starting point. No SAT search, natural relation, rank or total IC runtime is
part of this capacity gate.

## Frozen inputs and controls

| Input | Exact choice |
| --- | --- |
| Curve | `icv1-f2m83-tm6151469093347-debefd74`, `a=0`, `b=1`, low modulus terms `[0,1,2,45]` |
| Retained object | PR #2124 K=16,627 manifest, replay and upload receipts, plus the content-addressed compressed object named above; require the adapter's full digest, point and orbit replay |
| Public target | primary fixture `a=0, fixture=0` from `research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-corpus.json`; record source SHA-256 before running |
| Adapter command | `primary-orbit-chain-build PANEL_DIR 16627 public_x_hash 2026100801 4 0 250000 6000000 4096 NEW_OUTPUT_JSON` |
| Negative cap control | Repeat the same object, target and command with `max_variables=1`; require `UNKNOWN_variable_cap` after checked import and preflight, with no model or solver search |
| Core positive control | Run the checked m=4 planted-chain test in `binary_semaev_chain_sat` and the existing m=4 core example tests on the exact worker source before the capacity run |

The input staging directory contains only copies of the committed manifest,
replay, storage receipt, public corpus and compressed object. Hash each staged
file and reject a mismatch before the worker starts. Do not edit the frozen
object or replace the public fixture on failure.

The source corpus SHA-256 is
`760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`.
The committed K=16,627 manifest, replay and storage-receipt SHA-256 values
are `0c09f62bde61d3c3e987644508c53ab56fd3f545219013b9bde925ba29969275`,
`75b3bc17d8366e258e1b8f3f8de5f42911eb893c3d029a2e40c65b9e59cffe60`
and `70e82dfef4ad735a6974b46a4d6bca726dba4cc68be5f35ca354fab3262b445b`,
respectively. The object's SHA-256 is
`2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369`.

## Execution envelope and accounting

Build the native exporter with `cargo build --release --offline --locked
--no-default-features --example koblitz_n83_factor_base_export` in the pinned
arm64 Linux `rust:1.93.1-bookworm` image
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.
The source is read-only, the Cargo cache is read-only, the build target is
separate, network is disabled, and the build has at most four CPUs, 8 GiB,
zero swap and a 600-second outer wall cap. Record the binary SHA-256,
source commit, container image and exit/OOM state.

Run each capacity command in a fresh Linux container with one CPU, 4 GiB
hard memory, zero swap, no network and a 1,800-second outer wall cap. The
worker must see a clean committed source snapshot and matching compiled
source hashes. Record complete stdout/stderr, output JSON if created,
container exit/OOM state, cgroup memory peak, wall time, base-import time,
target-validation time and model-construction time. Capture the first run's
peak at exit; do not fill it from a later run. Run the negative control before
the full-cap model.

## Outcomes and next action

`PASS_model_construction_only` requires the full-cap worker to finish,
verify the exact retained object, install the m=4 Frobenius-orbit domain, and
emit a bound model receipt with measured variables, clauses and XOR rows
under the 4 GiB cap. The negative control must return
`UNKNOWN_variable_cap`. A failed negative control invalidates the capacity
comparison. OOM, outer timeout, source/input mismatch, missing receipt and
infrastructure failure remain separate censored or producer-failure
outcomes. Preserve raw logs and failed attempts.

If capacity passes, the next experiment is a degree-83 planted-relation
positive control on this exact imported base, followed by natural-query
yield and useful-rank comparison with retained K=2,048 and structured
2,001-column bases under a frozen common corpus. If capacity exceeds 4 GiB,
measure which phase failed and test a separately frozen memory or implicit
domain change. No capacity outcome by itself selects a factor base or
updates the IC scoreboard.
