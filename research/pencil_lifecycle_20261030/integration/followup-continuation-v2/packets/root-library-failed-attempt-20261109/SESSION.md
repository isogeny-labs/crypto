# T-1972 failed attempt archive

## What was done
Archived11 native/control files with hashes and original modes. Read initial-query/worker/root-terminal invocation join. Preserved unsealed supervisor failures separately. No test or compiler rerun. [pcc:unverified]

## Decisions made
Charge1,800 seconds as recorded; no test result inferred from empty commands. Keep later custody reconciliation separate from failed supervisor sealing. [pcc:unverified]

## Open threads
Producer-owned bounded successor fixing log ownership. [pcc:unverified]

## Errors
Original failure: root-owned0600 logs caused PermissionError during supervisor checkpoint, so worker never received ownership permit. [pcc:unverified]
