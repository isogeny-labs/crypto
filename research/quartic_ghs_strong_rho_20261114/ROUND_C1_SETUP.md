# Round C1: fail-closed jump-only batched setup

Status: prospective corrective protocol, frozen before the corrected candidate
is implemented or measured on 2026-11-18. Round ID:
`ghs-strong-rho-r3b-batched-window2-jumps`.

## Reason for the correction

Post-result integration review found an unexercised accounting defect in the
Round C initial-walker fallback. Round C tentatively generated and evaluated
all walker coefficients in a batch. If any point was the identity, it restored
the RNG, native counters, and field counters before executing the sequential
retry path. Restoring counters hid physically executed tentative arithmetic
and RNG draws. The retained `q=223` record did not take that fallback, so its
observed counts describe that control path, but the implementation is not
integration-ready.

Round C1 removes the defective path rather than weakening accounting. It uses
the cold-charged width-2 simultaneous multiplier for the 32 jump points only.
All eight initial walkers use the existing lane-ordered `new_walker` function,
including its identity retry. There is no tentative walker work, counter
rollback, RNG rollback, or direct construction of `Walker`. This also keeps
walker initialization centralized for compatibility with later cached hot-loop
state.

## Baseline and accounting

The exact Round C source under correction is commit
`f3f2cbcc11ad8ddea9a3c1730144c406bed9fb2b`; the original baseline remains
`ad2a014c0022cf000845014499a9649b6c2cef2d`. The width-2 table remains cold and
fully charged. The target multiplication, table, 32 batched jumps, eight
sequential walkers, canonicalisation, retries, and verification are included.

Primary setup cost remains the complete operation vector and the symbolic
`M-eq(R) = fq_mul + R*fq_inv`. Acceptance requires candidate `fq_mul` and
`fq_inv` to be componentwise no worse than the legacy baseline with at least
one strict improvement. No wall-time value is an acceptance signal.

## Frozen control and gates

The deterministic control remains public synthetic `q=223`, instance seed 1,
target index 0, target policy `ic-repeat`, `strong-b8`, calibration triples 1,
single thread. The corrected and legacy setup arms must have identical:

- `ma` and `mb` arrays and all 32 jump points;
- initial walker points, coefficients, and post-setup RNG state;
- planted logarithm, answer, steps, DPs, restarts, look-ahead retries, cycles,
  and cycle additions; and
- walk/verification field counts and all native counts.

The complete group-law and width-2 point-equality tests remain mandatory. The
self-test must additionally make it explicit that walker construction is the
legacy sequential path in both arms. Fail on any identity/state/count
divergence or setup `fq_mul`/`fq_inv` regression. Retain failures.

This is a corrective deterministic code/count round only. The checkpoint,
profile, isolation, complete matched grid, independent validation, PCC,
publication, frontier, and leaderboard gates are unchanged.
