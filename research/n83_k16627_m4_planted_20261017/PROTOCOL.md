# Frozen N83 K=16,627 planted four-summand control

**Status:** preregistered before worker edits or execution. Preserve this file after outcomes; append receipts and interpretation separately.

## Question and reference

Can the exact retained primary GF(2^83) signed-Frobenius base used by the [model-capacity gate](../n83_k16627_m4_capacity_20261017/RUN_REPORT.md) support a checked four-summand SAT witness and a full-width signed-orbit relation row? The prior gate built a model but did not search it. This experiment is a positive control for the encoder, solver and group lift on a constructed public point. It is not a natural-query yield or full index-calculus comparison.

The curve is `icv1-f2m83-tm6151469093347-debefd74`, `a=0`, `b=1`, low modulus terms `[0,1,2,45]`. The retained object is the K=16,627, 2,760,082-point `public_x_hash`, seed `2026100801` object with compressed BLAKE3 `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`. Require the same manifest, replay receipt, storage receipt, full point-set digest and source attestation as the capacity gate. Do not rebuild or silently replace the base.

## Frozen target and arms

After checked import, choose the first stored point of signed-orbit columns 0, 1, 2 and 3, in that order. Add these four points with the primary curve group law to make target `R`. Record their compressed coordinates, point indices, x coordinates, column labels, four intermediate sums and `R` before model construction. Require all five points and the intermediate sums used by identity-mask-zero S3 to be affine and the four x coordinates to be distinct; otherwise report `INVALID_PLANTED_CONTROL` and stop without choosing a replacement target.

| Arm | Exact workload | Expected status |
| --- | --- | --- |
| Pinned positive | Build the full K=16,627, m=4 Frobenius-orbit model at identity mask 0; add unit clauses fixing all four summand x coordinates to the recorded planted values; solve with at most 100,000 conflicts and 1,800 seconds outer wall. | `PASS_PLANTED_RELATION` only for SAT, original-field S3 replay, exact x tuple, checked point sum, checked signed-Frobenius row and rank one in a fresh wide tracker. |
| Unpinned search | Build the same full model and target with no planted x clauses; cap at 5,000 conflicts and 900 seconds outer wall, at most eight SAT models. | A checked relation is a hit; `UNKNOWN_search_cap`, OOM and timeout retain their distinct censored meanings. The planted witness does not make a capped unpinned miss a refutation. |
| Tamper control | Replay the returned pinned relation row against `R + G` instead of `R`, where `G` is the primary generator. | The independent group check must reject it. |

The pinned arm is first. Run the unpinned arm only after a valid pinned positive result; otherwise its prerequisite is unverified and it is marked not attempted. Do not infer natural yield from either arm. Do not substitute a smaller base, smaller field or a different target if a cap is reached.

## Execution and accounting

Implement the worker in Rust in the existing primary adapter/exporter. Use a clean committed source snapshot and a native release binary built offline from the pinned Rust 1.93.1 arm64 image used by the capacity gate. Before running the worker, pass the touched example tests, the existing degree-83 chained-S3 controls, source/input hash checks and a local build. A failed source check or invalid base is a producer failure.

Each arm runs in a fresh Linux container with one CPU, 4 GiB hard cgroup memory, zero swap, no network and read-only root, under its outer wall cap. Record binary/source/input hashes, exact command, full stdout/stderr, container exit and OOM state, cgroup peak, base import, model build, solver conflicts/models/time, field replay, group lift, row replay and rank time. Keep construction/storage from the prior gate separate. This is an L0 shared-host diagnostic; no stage speedup or end-to-end cost is selected by it. No same-output external solver comparison is claimed.

## Outcome and next step

A valid pinned result establishes only that this exact constructed target traverses the full-width model and checked relation path. The unpinned capped arm measures whether the existing search finds a relation on that favorable target within the frozen bound. Any `UNKNOWN`, OOM, timeout, invalid model, lift mismatch or missing receipt is preserved with its exact phase and does not count as zero mathematical yield. If both controls pass, freeze a separate common natural-query corpus for K=16,627, retained K=2,048 and structured K=2,001; compare accepted relations, useful rank and cold costs with matched IC and rho references. If the pinned arm fails, inspect its first invalid phase before changing the encoding or caps in a new protocol.
