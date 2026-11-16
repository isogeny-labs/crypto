# Result packaging

## What was done
Read actual native/caller receipts, rehashed terminal manifest entries, copied exact bytes and built receipt archive.

## Decisions made
Preserve failure and full2400-second charge. Report nested timings separately.

## Open threads
Root publication and independent result review; any successor requires separate source review/admission.

## Errors
Actual worker target monitor FileNotFoundError; materializer killed/reaped exit-9. No compiler/test invocation.
