# Independent source review: matrix candidate additions

No source blocker was found in the added budget admission method. The patch preserves the actual original ledger and adds one Matrix-only addition reservation; it does not return from Extraction to Matrix or reset phase accounting.

Reviewed packet: `/home/ec2-user/ml-dataset-resume-20261029/matrix-candidate-addition-budget-20261107`, manifest SHA-256 `6abce11a07838e48eabbc7dbd151ace62ad3e2f2e19c628bd9d6f5493fd77db8`. Patch SHA-256 `2eac3887f143723d6651afe658b02d21030b9d361edcee58ebcd189d043079d8`. Proposed full source SHA-256 `a591fba26f09c820a2a341978805182b083023d25a504a08f9183ab4c55f1f1f`.

## Exact source and extraction checks

`verify_source.sh` copied and verified all nine manifest members, including the retained test log and binary (the binary was not executed). It copied the bound original `koblitz_m3.rs` (596,653 bytes; SHA-256 `52e23adf80499146f4db56553291a37e16b8b365f48d3c772576f7b7fd991317`) into a new scratch tree. Ordinary `git apply --check` and apply succeeded. The resulting file equals `koblitz_m3.proposed.rs` byte for byte.

Removing exactly the new method and its documentation block from the proposed file reproduces the original file's SHA-256. Existing `reserve_addition`, `check`, `observe`, phase transitions, initial admission and later code are unchanged.

The declared extraction was reproduced from the proposed full file: start at `const NATIVE_WORK_MAX`, stop before the documentation of `admit_original`, remove only the `serde::Serialize`/`serde::Deserialize` derives and `#[serde(deny_unknown_fields)]`, and prepend `use std::time::{Duration,Instant};`. The result is byte-identical to the packet's `budget_core.rs` (9,675 bytes; SHA-256 `76a664ba514ba04e932bf75d754c5337e9b7451265788ace4cb51a8785480afc`). The extracted core therefore contains the actual Budget fields and first implementation block, including the added method, rather than a substitute ledger.

This extraction excludes `admit_original`, the later `reserve_checking` implementation, original worker admission, capture, and lift adapters. Passing its tests cannot establish those integrations. This reviewer did not compile or rerun tests.

## Method contract

In `koblitz_m3.proposed.rs:306-315`, `reserve_matrix_candidate_addition`:

1. Calls the unchanged `check(now)` first. That preserves an existing stop and checks the original deadline. In Matrix, the deadline is the original attempt deadline, not a new clock.
2. Requires exactly `Phase::Matrix`; a wrong phase installs the original sticky `Stop::Phase` without an addition charge.
3. Compares the unchanged `additions` against the unchanged `addition_limit`, installs sticky `Stop::Additions` if exhausted, and otherwise increments only `additions`.
4. Does not modify the phase, ownership identity, native/phase-native counters, phase ceilings, memory/work limits, or either deadline.

The equality guard is consistent with the ledger's existing invariant: the private addition counter starts at zero, both addition methods increment only after the same limit check, and no API refunds/resets it. `Budget::new` also rejects configured-work multiplication overflow before establishing its shared addition limit. The new method neither weakens that cap nor introduces a separate allowance.

The existing final-extraction `reserve_addition` remains unchanged and uses this same counter. A caller can consume candidate additions in Matrix, complete the native phase, then enter final Extraction without replenishing the allowance. Reaching the allowance exactly can still finish successfully; only a request for another addition stops the ledger.

## Retained controls and next integration check

The retained `tests.log` reports six passed, zero failed. Source review confirms those six test bodies cover preservation of phase/owner/deadlines/native history; sharing candidate and final-extraction additions; exact-limit terminal completion; rejection in all other phases; sticky deadline refusal; and preservation of a saturated native byte ceiling after a candidate addition. The helper constructs the actual Budget and drives its actual begin/observe/finish methods. One control assigns private phases directly to enumerate invalid states; this is a unit boundary test, not evidence of a reachable worker path.

A genuine assembled owner/lift test should additionally reject the next candidate addition while still in Matrix at the exact shared cap, retain its prior count/first stop, and verify one admission immediately before each real signed group addition. It should demonstrate continuation without `finish_native`/`begin_native` between rejected candidates. The owning adapter must use this new method for provisional lift checks and reserve final Extraction for the accepted result/publication path.

The source-level change is suitable for that next integration step. **Full worker integration remains uncompiled.** No scientific generation, cloud request, production edit, or pending SAT rebuild was performed by this review.
