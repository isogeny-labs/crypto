# N83 K=16,627 retained-base m=4 capacity gate

**Date:** 2026-10-17 UTC. **Protocol:** [PROTOCOL.md](PROTOCOL.md),
committed as `791bcf70ed` before code changes or a worker run; the
[pre-run control clarification](CONTROL_ADDENDUM.md) was committed as
`f74f6bc43a`. **Status:** model-capacity result; pre-stack release-suite
validation passed, with stacked-head validation and PR review pending. This report does not promote an IC
scoreboard row.

## Exact input and checked worker

The curve is `icv1-f2m83-tm6151469093347-debefd74`, `a=0`, `b=1` over
`GF(2^83)` with low modulus terms `[0,1,2,45]`. The retained
`public_x_hash`/seed-`2026100801` signed-Frobenius base has **16,627
columns** and **2,760,082 point records**. The compressed object's BLAKE3
is `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`.
Its earlier construction, all-point generic replay and S3 round trip are
recorded in [the retained-base study](../n83_k16627_prebuild_20261017/RUN_REPORT.md)
(PR #2124). The public target came from the pinned primary fixture zero;
no known-answer scalar was passed to the capacity worker.

The five staged input SHA-256 values and the Linux arm64 binary SHA-256 are
in [BUILD_RECEIPT.md](BUILD_RECEIPT.md) and
[inputs-before-negative.sha256](inputs-before-negative.sha256). The compiled
adapter source BLAKE3 in both worker receipts is
`d1e8b332163ab6f51b53ac0e3cb2e7d781bd2760651beb00d63f6b6df34102ad`.
The source [attestation](source-attestation.json) binds clean commit
`e54bb7d6a930571ad0e68d51c684ec68f65bda88`; the worker compared its
compiled bytes against all five snapshot files before base import. The
native [controls](CONTROL_RECEIPT.md) passed before either worker.

Both runs used the same pinned offline arm64 image, one CPU, a 4,294,967,296
byte hard cgroup, zero swap, no network, read-only root and a 1,800-second
outer wall cap. The full arm's [host config](full/host-config.json) confirms
one billion nanocpus, 4 GiB memory and memory+swap, `none` network and a
read-only root. The worker independently read the cgroup limit and zero
swap in both receipts. Each run used a fresh container and output directory.

## Observations

| Measure | Negative variable-cap control | Full m=4 model arm |
| --- | ---: | ---: |
| `max_variables` | 1 | 250,000 |
| Status | `UNKNOWN_variable_cap` (expected) | `PASS_model_construction_only` |
| Required variables / domain clauses | 201,598 / 5,729,152 | 201,598 / 5,729,152 |
| Installed SAT variables / clauses / XOR rows | none | 201,598 / 5,915,238 / 3,901 |
| Base import | 77.488 s | 67.294 s |
| Target check / model construction | 0.006 s / skipped | 0.009 s / 0.325 s |
| Worker process wall | 77.876 s | 67.742 s |
| Cgroup memory peak | 2,661,171,200 B | 2,661,126,144 B |
| Container outcome | exit 0, no OOM | exit 0, no OOM |

The full model has 62,001 recorded AND gates and three S3 nodes. The
negative [worker receipt](negative/worker.json), [state](negative/container-state.json)
and [peak](negative/peak.bytes) show that exact-base import and preflight
completed before the variable cap stopped model creation. The full
[worker receipt](full/worker.json), [state](full/container-state.json) and
[peak](full/peak.bytes) show a complete model under the frozen limit. The
full container ran from `23:41:20Z` to `23:42:29Z`; the negative one from
`23:38:10Z` to `23:39:28Z`. Both workers reported zero solver search,
relation and rank execution.

These walls are descriptive L0 observations on a shared host; the
repository-wide release suite and other work overlapped the containers.
The arms differ in model cap and were not designed as a speed comparison.
The cgroup peak is for each whole process, not a separate import or model
peak. The similar peaks therefore do not establish the model's standalone
memory increment. No same-host equal-output external tool comparison was
run, and no fastest-model claim follows.

## Interpretation and next gate

The preregistered capacity condition passed for **this exact base, m=4,
identity mask zero, and Frobenius-orbit domain**. A 4 GiB zero-swap container
can import the retained object and construct this full-width model. This
removes the specific adapter and model-capacity gap at the tested settings.
It does not establish that SAT search can finish, that natural targets have
useful four-summand decompositions, or that accepted relations add useful
rank. The K=16,627 base's larger column count also leaves linear algebra
and amortization costs unmeasured. m=2 and m=3 remain open.

The first [construction report](../n83_k16627_prebuild_20261017/RUN_REPORT.md)
gives separate L0 construction and replay times, and the current workers
give import/model times. They came from different capped phases and host
loads, and S3 transfer, solver search, relation collection, rank, target
extraction and verification remain uncharged. They cannot be combined into
one end-to-end IC cost or compared with rho as a pipeline row. The current
verified IC baseline and matched-rho gap remain unchanged.

The next experiment is a **degree-83 planted four-summand relation control
on this exact imported K=16,627 object**, with full group lift and signed
orbit coefficients checked. If that passes, freeze a common natural-query
corpus and compare relation yield and useful rank with retained K=2,048
and the structured 2,001-column base, including cold import and setup.
The [capacity diagram](capacity-diagram.svg) shows the completed and open
phases. The canonical IC progress charts are unchanged because this run
added no verified relation, rank or complete-cost row.
