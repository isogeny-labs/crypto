# Reproduce the source and analysis qualification

Use one CPU/thread and fresh paths. Mathematical checks and statistical analysis run in Rust. Python here only edits source metadata, hashes files or renders already recorded outputs. No real comparison has run.

## Source and builds

Check out the PR source and verify `SOURCE_V2.sha256` from the repository root. Source files, literal inputs and every locked dependency version must match. The source scope includes the unchanged actual production library, not an extracted substitute. `SOURCE.sha256` belongs to the initial failed compile; its root-worker preimage is in `failed-source-v1/`.

Follow `build-v2.sh` with NEW raw/receipt paths. It builds normal tools and all five libtest workers under release opt0/LTOoff/debug0/incrementalfalse/codegen16/default-features=false, CPU0/jobs1 and one numerical thread. Use the recorded Rust1.100.0 toolchain. This is correctness qualification, not performance compilation. Preserve original Cargo/root/historical locks. The current project lock preserves all old registry versions/checksums and adds only its new path graph.

Copy normal tools and libtest executables to new paths, remove write permission and hash them before calls. Run every libtest worker once with `--test-threads=1 --nocapture`; retain full stdout/stderr/exit. Do not replace supplied workers, pre-call records or results.

## No-clock transport

The qualification-only `SYNTHETIC_INPUT_PINS.json` binds all four frozen worker identities and the six reference inputs. Its schema/synthetic marker deliberately cannot authorize the actual collector. Relocate only paths in a NEW pin file, preserving every input/worker digest. Existing historical workers are the frozen 85/96/flat274 tools; their hashes are not silently updated.

```sh
root-transport-qualify NEW_FROZEN_ROOT_WORKER REFERENCE_INPUTS NEW_QUALIFICATION_PINS NEW_TRANSPORT_OUT
```

Run the qualifier on CPU0 under the declared 2GiB address-space and stage/output caps. It runs all 432 native/wrapped root combinations, passing `--qualify` to every worker. The worker never creates a performance clock in that mode. Parent readback requires null phase fields, complete packed proof arithmetic, exact input bytes, checked/native basis and statistics binding, and four semantic canaries per call. Every failure is retained and rejects completeness.

## Synthetic analysis

The supplied data archive allows replay without rebuilding or rerunning the root. Verify archive SHA sidecars from the repository root; extract only into an empty directory. Inspect source and custody records before executing archived tools. Local ELF hashes are not execution signatures.

```sh
root-supervised-analyze NEUTRAL_OUT NEUTRAL_OUT/monitor.jsonl NEUTRAL_OUT/prospective-pins.json NEW_NEUTRAL_ANALYSIS
root-supervised-analyze HYPOTHETICAL_OUT HYPOTHETICAL_OUT/monitor.jsonl HYPOTHETICAL_OUT/prospective-pins.json NEW_HYPOTHETICAL_ANALYSIS
```

Each complete analysis replays all 8,640 proofs. Monitor flags and clocks are synthetic. The neutral result rejects threshold arithmetic. The invented 20× result clears the arithmetic, while admission remains false. Do not label either as a benchmark, observed contention or measured A/A noise. [pcc:unverified]

To reconstruct the full fixtures, follow `run-synthetic.sh` with new outputs. It uses frozen historical240 generic/packed proofs, historical281 flat proofs and the new no-clock root transport tree. Every reused proof is independently checked before copying to a local hash-addressed synthetic proof cache; hardlinks only reduce duplicate storage. The same proof is checked again by analysis.

`run-canaries.sh` makes copied hardlink views and uses atomic replacement of the clock receipt or summary row. Expected exits are 1 at phase/boundary binding and missing/duplicate cell, with no report. Verify `CANARY_SOURCE_BEFORE.sha256` from the archive extraction directory afterward; all neutral bytes must remain unchanged. The supplied two complete sabotage views can be analyzed directly into fresh refusal directories.

The draft collector's `--print-roster` is metadata only. Do not invoke its real comparison until the separate canonical-document dependency and prospective protocol/source/compiler/control pins are resolved. Original strongest-control minimum, full boundary, repetitions, A/A, six cases, every failure/proof and 15× acceptance remain unchanged. The existing global compiler failures are not rerun or converted into a pass here.
