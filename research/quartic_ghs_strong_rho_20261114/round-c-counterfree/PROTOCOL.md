# Round C-CF: counter-free production arithmetic

Status: prospective implementation protocol, frozen before candidate code or
candidate output on 2026-11-18. Round ID:
`ghs-strong-rho-r3-counterfree-arithmetic`.

This round asks whether the merged Round B GHS strong-rho implementation can
provide a compile-time counter-free arithmetic specialization without
duplicating its field, curve, or walk logic. It is an implementation and
correctness task. It does not establish a wall-time gain, alter the counted
`M-eq` reference, execute the complete frontier grid, or move a leaderboard.

## Baseline and prior work

The baseline is `origin/main` revision
`ad2a014c0022cf000845014499a9649b6c2cef2d`; its merged Round B
`strong_rho.rs` SHA-256 is
`0bb8da07393873aa2b935403b86f5b7a0c24a52fff2d598a145a9fcca103229c`.
The existing `strong-b*` path and its field/native records remain the sole
count-authoritative implementation.

The design follows the repository precedent in
`src/cryptanalysis/jv_rho_strong.rs`: `Arith<const COUNT: bool>` uses a const
generic so production code can omit hooks without maintaining a second
arithmetic implementation. Here the const generic gates `FastF3`'s `Cell`
counter updates. High-level walk/native diagnostics remain available in the
first candidate; removing them is outside this round. The Round B protocol
and result define the source and q=223 control being preserved.

The shared research-KB MCP was not exposed and the repository wrapper found
no executable. This round therefore used the two repository sources above;
it makes no novelty or absence claim.

## Frozen modes and workload

- `strong-b<W>` is `COUNT=true`: existing JSON shape and all field/native
  counts remain authoritative.
- `fast-b<W>` is `COUNT=false`: it uses the same arithmetic and walk through a
  separate monomorphization, emits a visible `count_authoritative=false`
  marker, and must emit zero `FieldCounts`. Its wall field is diagnostic and
  is not compared in this round.
- Correctness control: public synthetic q=223 instance, seed 1, target index
  0, `ic-repeat`, widths 1, 8 and 32 in the standalone self-test. A paired
  driver control may use `GHS_CALIB_TRIPLES=1`; calibration and wall fields are
  ignored.

No profiler, timing screen, or benchmark may run unless checkpoint push/gate,
benchmark-record, baseline identity, A/A and ABAB requirements pass. The
current runtime has no research-KB or Conductor executable; their failures are
retained as implementation metadata, not experimental results.

## Requirements-to-evidence matrix

| Requirement | Evidence | Pass condition |
|:--|:--|:--|
| One arithmetic implementation | source review | const generic gates counter updates; field/curve formulas are not copied |
| Counted compatibility | existing and candidate `strong-b8` q=223 JSON plus self-test | same answer, walk fields, field counts and native counts as merged Round B |
| Counter-free arithmetic | `fast-b8` q=223 JSON and self-test | all setup/walk/verify `FieldCounts` are zero |
| Mathematical identity | paired counted/fast self-tests at widths 1, 8 and 32 | same found scalar, correctness, steps, DPs, restarts, look-ahead, cycles and cycle additions |
| Diagnostic identity | paired outputs | setup/walk native counts are equal; wall and calibration are explicitly excluded |
| Claim discipline | result note and unchanged dashboard | no throughput, E2E or frontier conclusion from this round |

## Stop and decision rules

Stop on a wrong field value, point, scalar, walk-structure mismatch, nonzero
fast field counter, changed counted counter, missing raw output, or checkpoint
refusal before any performance run. Preserve every failure.

The implementation is ready for integration only if all deterministic gates
pass and the existing `strong-b*` record remains count-authoritative. A later
profile may use `fast-b*` only after source/binary identity and checkpoint
gates pass; a profile can explain cost but cannot establish a throughput gain.
