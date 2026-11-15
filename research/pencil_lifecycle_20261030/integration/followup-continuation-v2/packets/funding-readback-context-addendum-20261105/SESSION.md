# T-1614

## What was done
Two explicit-context Kubernetes reads and two GitHub publication reads, all retained with receipts.

## Decisions made
Preserve both context observations; classify current failure as Kubernetes authorization, not STS.

## Open threads
Authorized operator cap readback; GitHub PR synchronization.

## Errors
Both Kubernetes commands returned Forbidden; no execution was admitted.
