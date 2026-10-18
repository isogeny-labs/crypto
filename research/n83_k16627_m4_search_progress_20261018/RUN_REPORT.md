# N83 K=16,627 four-summand post-root search progress

**Run:** 2026-10-18 06:56:55–07:26:55 UTC. **Status:** `UNKNOWN_wall_cap` for one planted, seven-x-block positive control. This is a solver-stage diagnostic on the primary curve `icv1-f2m83-tm6151469093347-debefd74`, not a natural-query relation or a factor-base selection.

## Frozen workload and provenance

The protocol was committed as `c158b99cba` before the observer implementation or this run. The accepted worker source is clean pre-stack commit `2e1ba2e4763242bb4c8870e22f93b0183c7b30f7`; [source-files.sha256](source-files.sha256) checks six compiled files both in that checkout and in the read-only snapshot, and [inputs-before-run.sha256](inputs-before-run.sha256) checks the five retained inputs. The Linux arm64 binary SHA-256 is `6d01aa0599124efdea5cf43acf2fdbc150ccac7330482893fd36a36f0cee7b09`, built offline in the pinned `rust:1.93.1-bookworm` image recorded in [BUILD_RECEIPT.md](BUILD_RECEIPT.md). The retained compressed base has BLAKE3 `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`, point-set BLAKE3 `564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0`, 16,627 signed-Frobenius columns and 2,760,082 points.

The exact full m=4 model has 201,598 SAT variables, 5,915,238 clauses and 3,901 XOR rows. The constructed witness pins four summand x blocks, two intermediate x blocks and the target x after installing the full orbit domain. The pre-SAT group check verified four distinct summand x values, affine partial sums, a rank-one row and rejection of a tampered target. The worker had one CPU, a hard 4 GiB cgroup, zero swap, no network, a read-only root and an outer 1,800-second cap. This shared macOS-host arm64 Linux container is an L0 diagnostic; there was no matched baseline or external-tool speed comparison.

## Observations

| Measure | Recorded value | Source |
| --- | ---: | --- |
| Root propagation | `Fixpoint` in 199.977792 ms; zero decisions/conflicts | [phase records](positive/worker.json.phases.jsonl) |
| SAT entry | 39.623697 s elapsed | [phase records](positive/worker.json.phases.jsonl) |
| Periodic progress | 175 ten-second samples, all with current phase `XorPropagation`; initial sample `Idle` | [progress records](positive/worker.json.progress.jsonl) |
| Last complete sample | 1,792.112144 s elapsed; 322,255 completed decisions, 360 completed conflicts, two completed restarts | [progress records](positive/worker.json.progress.jsonl) |
| Container result | exit 124; `OOMKilled=false`; cgroup peak 2,662,309,888 bytes | [state](positive/container-state.json), [peak](positive/peak.bytes) |
| Derived outcome | `UNKNOWN_wall_cap`; SAT answer, model, checked group lift and rank unset; unpinned arm not attempted | [outcome](positive/outcome.json) |

The phase field and counters are separately read relaxed atomics, so a single sample is not a simultaneous solver snapshot. Completed decisions rising across the samples establish repeated search after root, and periodic samples consistently landed in XOR propagation. Those snapshots do not measure an exact fraction of total CPU time or isolate which inner XOR operation costs most. The [fail-closed derivation](derive-capped-outcome.jq) checked the resource envelope, root return, SAT entry, 176 monotone, identity-matched progress records and last-sample time. Work after the last fsynced sample is unknown. The absence of worker JSON at the outer cap is checked separately; the container did not return a SAT or UNSAT result.

The earlier [root-phase control](../n83_k16627_m4_root_phase_20261018/RUN_REPORT.md) remains an independent capped arm with different worker source and 185.023167 ms root timing. These two root times are not a paired performance comparison. The pre-SAT rank-one group preflight does not establish a returned solver relation or natural-query useful rank. There is no complete cold index-calculus cost, matched-rho gap or selected m=2, m=3 or m=4 base from this run.

## Validation and next discriminator

[VALIDATION.md](VALIDATION.md) separates exact-source tests, worker build, raw-copy checks, the full release suite and stacked-branch limitations. [COMMANDS.md](COMMANDS.md) records the executable commands; [positive-files.sha256](positive-files.sha256) and [positive-files-check.log](positive-files-check.log) bind the compact raw result. Earlier failed build and sparse-checkout attempts remain in the study. No graphable end-to-end performance gain exists, so canonical cost and curve graphs are unchanged; the diagram here shows the observed stage sequence and unresolved downstream gates.

The next frozen experiment should mirror completed XOR-pass count, cumulative XOR time, row scans, repivots and row operations into observer atomics at pass boundaries. It should keep the same exact object and witness, repeat the capped arm and compare a direct-versus-observed small native chained-field fixture with original-field and group-lift checks. Only after identifying the inner cost should a solver policy change be timed against the frozen current IC pipeline and the near-size-matched explicit K=2,048 and structured 2,001-column bases. The broader selection gate remains in [the problem record](../../problems/n83-prebuilt-base-quality-m2-m4.md).
