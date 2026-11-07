# Original-owned search report transport

The actual proposed worker now requests a one-shot fixed partial report from its retained Boolean search owner. The central report-unavailable guard clears only when that call returns a paid report. The solver output continues to own all original search/capture custody.

Report admission uses the original live Budget, the existing parent-storage quote for an 8192-byte report, 16 times that bound for encoding work, 262144 additional bytes for fixed JSON object construction and capture-head copying, and the original trace reservation. Nothing resets the ledger, deadline, phase, or native attempt. The latest snapshot must be complete, its last chunk exact and acknowledged, and its head identical to the actual CaptureEmitter head. The report makes no storage retry. A final original-clock check follows JSON construction. Refusals retain custody and the unavailable guard; existing failure stage and first budget stop are preserved. The report attempt is consumed even when admission refuses.

The report identifies one pattern and one complete retained snapshot, exposes cumulative reservation counters, hashes, finite producer classifications, and scoped failure category. All-pattern coverage, solver completion, ECDLP completion, independent original-source replay, S3 custody and scalar recovery are explicitly false. FFD/LFD/degree-of-regularity fields are null. Producer classification is not independent replay. Reservations are explicitly not retired instructions. Snapshot encoding completion is not search completion.

Stopped/deadline-exhausted owners still cannot produce this report. This is intentional preservation of the original admission contract: reporting them would require an independently specified pre-admitted reporting allowance, not a fake clearing of the guard. A live incomplete search can be reported once an actual complete snapshot exists. Early pre-owner worker failures keep their original unavailable guard. No prime, F5/F6, SAT, or full-span support is inferred.

## Validation scope

Eleven focused Rust tests pass. They compile the actual unchanged report module with real cached serde_json and structural fixture owner/Budget/CaptureEmitter/quote types. These fixtures exercise the report control flow, not the genuine budget implementation or actual disk capture. The fixed JSON size test replaces all numeric values, including hash bytes, with u64::MAX (strictly wider than genuine hash bytes), uses the longest failure category, and checks the serialized result fits 8192 bytes. Other controls cover charging, one-shot delivery, stopped ledger, work/trace refusal, initial/final deadline refusal, changed head, incomplete snapshot/unpublished map, and prior failure retention.

Full owner integration, full CLI execution, real filesystem capture and report admission against the genuine Budget have not been compiled or replayed here. No Cargo rebuild, dependency rebuild, cloud job, scientific trial, or Git mutation occurred. An initial successful compile preceded two added controls and the preservation of the original failure stage; the final compile and 11-test run are preserved.

## Composition

Apply solvers.patch to the frozen assembled solvers.rs. The checkpoint agent already integrated owner.patch's module declaration and report_taken field/default into its updated owner; do not apply those edits twice. Add owned_search_report.rs adjacent to original_boolean_search.rs.

## Replay

From this packet directory:

```
timeout 60 bash -c 'ulimit -v 1572864; ulimit -t 60; rustc +1.98 --edition=2021 --test harness.rs -L dependency=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps --extern serde_json=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libserde_json-10f254ba1c7de3e0.rlib -o report-tests'
timeout 30 ./report-tests
```

The harness includes the report by its original absolute packet path. On relocation update that path only and retain both original and relocated source hashes. Test fixtures intentionally have no mathematical solver.
